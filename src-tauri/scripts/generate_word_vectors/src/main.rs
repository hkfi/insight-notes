use rust_bert::pipelines::sentence_embeddings::{
    Embedding, SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType,
};
use std::time::Instant;

use serde_json::to_vec;
use sqlite_vec::sqlite3_vec_init;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use tokio::{sync::oneshot, task};

type Message = (Vec<String>, oneshot::Sender<Vec<Embedding>>);

#[derive(Debug, Clone)]
pub struct SentenceEncoder {
    sender: mpsc::SyncSender<Message>,
}

impl SentenceEncoder {
    pub fn spawn() -> (JoinHandle<anyhow::Result<()>>, SentenceEncoder) {
        let (sender, receiver) = mpsc::sync_channel(100);
        let handle = thread::spawn(move || Self::runner(receiver));
        (handle, SentenceEncoder { sender })
    }

    fn runner(receiver: mpsc::Receiver<Message>) -> anyhow::Result<()> {
        let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL6V2)
            .create_model()
            .unwrap();

        while let Ok((texts, sender)) = receiver.recv() {
            let texts: Vec<&str> = texts.iter().map(String::as_str).collect();
            let embeddings = model.encode(&texts).unwrap();
            sender.send(embeddings).expect("sending embedding results");
        }

        Ok(())
    }

    pub async fn encode(&self, texts: Vec<String>) -> anyhow::Result<Vec<Embedding>> {
        let (sender, receiver) = oneshot::channel();
        task::block_in_place(|| self.sender.send((texts, sender)))?;
        Ok(receiver.await?)
    }
}

fn load_glove_vocabulary(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut vocabulary = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        if let Some(word) = line.split_whitespace().next() {
            vocabulary.push(word.to_string());
        }
    }

    vocabulary
}

async fn store_embeddings_in_db(
    pool: SqlitePool,
    words: Vec<String>,
    encoder: SentenceEncoder,
) -> anyhow::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS words (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            text TEXT DEFAULT '' NOT NULL,
            sentence_embedding BLOB
        )",
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "CREATE VIRTUAL TABLE IF NOT EXISTS vec_words using vec0(
            sentence_embedding float[384]
        )",
    )
    .execute(&pool)
    .await?;

    for word in words {
        // let start = Instant::now();
        let output = encoder.encode(vec![word.clone()]).await.unwrap();
        let embedding_json = serde_json::to_string(&output[0]).unwrap();

        let inserted_word =
            sqlx::query("INSERT INTO words (text, sentence_embedding) VALUES (?1, ?2)")
                .bind(word)
                .bind(&embedding_json)
                .execute(&pool)
                .await
                .unwrap();

        sqlx::query("INSERT INTO vec_words (rowid, sentence_embedding) VALUES (?1, ?2)")
            .bind(inserted_word.last_insert_rowid())
            .bind(&embedding_json)
            .execute(&pool)
            .await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let glove_path = "pretrained_embeddings/glove.6B.300d.txt";
    // let glove_path = "pretrained_embeddings/test.txt";
    let db_path = "embeddings.sqlite";

    let start = Instant::now();
    let vocabulary = load_glove_vocabulary(glove_path);
    let duration = start.elapsed();
    println!("Time elapsed in load_glove_vocabulary() is: {:?}", duration);

    let (handle, encoder) = SentenceEncoder::spawn();

    unsafe {
        libsqlite3_sys::sqlite3_auto_extension(Some(std::mem::transmute(
            sqlite3_vec_init as *const (),
        )));
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_path)
        .await?;

    println!("Start store_embeddings_in_db");
    let start = Instant::now();
    store_embeddings_in_db(pool, vocabulary, encoder).await?;
    let duration = start.elapsed();
    println!(
        "Time elapsed in store_embeddings_in_db() is: {:?}",
        duration
    );

    handle.join().expect("Thread panicked");

    Ok(())
}
