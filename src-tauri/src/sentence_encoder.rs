use rust_bert::pipelines::sentence_embeddings::{
    Embedding, SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType,
};
use std::fmt::Debug;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use tokio::{sync::oneshot, task};

type Message = (Vec<String>, oneshot::Sender<Vec<Embedding>>);

/// Runner for Sentence Embedder
#[derive(Debug, Clone)]
pub struct SentenceEncoder {
    sender: mpsc::SyncSender<Message>,
}

impl SentenceEncoder {
    /// Spawn a embedder on a separate thread and return a embedder instance
    /// to interact with it
    pub fn spawn() -> (JoinHandle<anyhow::Result<()>>, SentenceEncoder) {
        let (sender, receiver) = mpsc::sync_channel(100);
        let handle = thread::spawn(move || Self::runner(receiver));
        (handle, SentenceEncoder { sender })
    }

    /// The embedding runner itself
    fn runner(receiver: mpsc::Receiver<Message>) -> anyhow::Result<()> {
        // Needs to be in sync runtime, async doesn't work
        let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2)
            .create_model()
            .unwrap();

        while let Ok((texts, sender)) = receiver.recv() {
            let texts: Vec<&str> = texts.iter().map(String::as_str).collect();
            let embeddings = model.encode(&texts).unwrap();
            sender.send(embeddings).expect("sending embedding results");
        }

        Ok(())
    }

    /// Make the runner encode a sample and return the result
    pub async fn encode(&self, texts: Vec<String>) -> anyhow::Result<Vec<Embedding>> {
        let (sender, receiver) = oneshot::channel();
        task::block_in_place(|| self.sender.send((texts, sender)))?;
        Ok(receiver.await?)
    }
}
