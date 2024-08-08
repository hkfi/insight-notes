use crate::commands::tags::Tag;
use crate::{AppState, SentenceEncoder};
use futures::future::join_all;
use futures::TryStreamExt;
use langchain_rust::text_splitter::{MarkdownSplitter, SplitterOptions, TextSplitter};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
use std::collections::HashMap;
use std::fmt::Debug;
use std::time::Instant;
use tauri::Manager;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Word {
    id: i64,
    text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    id: i64,
    content: String,
    average_sentence_embedding: Vec<f32>,
    created_at: i64,
    updated_at: i64,
    tags: Vec<Tag>,
}

#[derive(sqlx::FromRow, Debug)]
struct NoteWithTag {
    id: i64,
    content: String,
    average_sentence_embedding: Vec<u8>,
    created_at: i64,
    updated_at: i64,
    tag_id: Option<String>,
}

fn convert_blob_to_vec_f32(blob: Vec<u8>) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    // Convert the blob (Vec<u8>) to a string
    let json_string = String::from_utf8(blob)?;

    // Parse the JSON string to a Vec<f32>
    let vec_f32: Vec<f32> = serde_json::from_str(&json_string)?;

    Ok(vec_f32)
}

impl<'r> FromRow<'r, sqlx::sqlite::SqliteRow> for Note {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        let id: i64 = row.try_get("id")?;
        let content: String = row.try_get("content")?;
        let blob: Vec<u8> = row.try_get("average_sentence_embedding")?;
        let average_sentence_embedding = convert_blob_to_vec_f32(blob).unwrap();
        let created_at: i64 = row.try_get("created_at")?;
        let updated_at: i64 = row.try_get("updated_at")?;

        Ok(Note {
            id,
            content,
            average_sentence_embedding,
            created_at,
            updated_at,
            tags: vec![], // Initialize with an empty vector
        })
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct NoteChunk {
    id: u16,
    sentence: String,
    note_id: u16,
}

#[derive(Debug)]
struct NoteChunkToInsert {
    sentence: String,
    sentence_embedding: String,
    sentence_embedding_vector: Vec<f32>,
}

async fn split_markdown_content(content: &str) -> Vec<String> {
    let options = SplitterOptions {
        chunk_size: 256,
        ..Default::default()
    };

    MarkdownSplitter::new(options)
        .split_text(content)
        .await
        .unwrap()
}

async fn create_sentence_embedding(text: &str, sentence_encoder: &SentenceEncoder) -> Vec<f32> {
    let sentences = [text.to_string()];

    let output = sentence_encoder.encode(sentences.to_vec()).await.unwrap();

    output[0].clone()
}

fn sentence_embedding_to_json(sentence_embedding: &Vec<f32>) -> String {
    serde_json::to_string(sentence_embedding).unwrap()
}

fn compute_centroid_from_note_content_chunks(chunks: &Vec<NoteChunkToInsert>) -> Option<Vec<f32>> {
    // Check if the input is empty
    if chunks.is_empty() {
        return None;
    }

    // Get the number of vectors and the dimensionality
    let num_vectors = chunks.len();
    let dimension = chunks[0].sentence_embedding_vector.len();

    // Check if all vectors have the same dimension
    if !chunks
        .iter()
        .all(|chunk| chunk.sentence_embedding_vector.len() == dimension)
    {
        return None; // Return None if dimensions mismatch
    }

    // Initialize a vector to store the sum of each dimension
    let mut sum_vector = vec![0.0; dimension];

    // Sum each dimension across all vectors
    for chunk in chunks {
        for i in 0..dimension {
            sum_vector[i] += chunk.sentence_embedding_vector[i];
        }
    }

    // Compute the average for each dimension to get the centroid
    let centroid: Vec<f32> = sum_vector
        .iter()
        .map(|&sum| sum / num_vectors as f32)
        .collect();

    Some(centroid)
}

async fn note_splitted_content_to_sentence_embeddings(
    state: &tauri::State<'_, AppState>,
    splitted_content: &Vec<String>,
) -> Vec<Vec<f32>> {
    // A vector of sentence_embeddings of the md file
    let mut sentence_embeddings = Vec::new();
    for content in splitted_content {
        sentence_embeddings.push(create_sentence_embedding(content, &state.sentence_encoder));
    }
    let sentence_embeddings: Vec<Vec<f32>> = join_all(sentence_embeddings).await;

    sentence_embeddings
}

async fn note_content_to_chunks(
    state: &tauri::State<'_, AppState>,
    content: &str,
) -> Vec<NoteChunkToInsert> {
    let splitted_content = split_markdown_content(content).await;
    let sentence_embeddings =
        note_splitted_content_to_sentence_embeddings(&state, &splitted_content).await;

    sentence_embeddings
        .iter()
        .enumerate()
        .map(|(i, embedding)| NoteChunkToInsert {
            sentence: splitted_content[i].clone(),
            sentence_embedding: sentence_embedding_to_json(embedding),
            sentence_embedding_vector: embedding.to_vec(),
        })
        .collect()
}

async fn find_similar_words(
    state: tauri::State<'_, AppState>,
    note_id: i64,
) -> Result<Vec<String>, String> {
    let db = &state.db;
    let word_embeddings_db = &state.word_embeddings_db;

    let note: Vec<Note> = sqlx::query_as::<_, Note>(
        r#"
            SELECT * FROM notes
            WHERE id = ?1
        "#,
    )
    .bind(note_id)
    .fetch(db)
    .try_collect()
    .await
    .map_err(|e| format!("Failed to search notes {}", e))?;

    let note_average_content_embedding = &note[0].average_sentence_embedding;

    let words: Vec<Word> = sqlx::query_as::<_, Word>(
        r#"
        WITH matches AS (
            SELECT
                rowid,
                distance
            FROM 
                vec_words
            WHERE 
                sentence_embedding MATCH ?1
                AND distance > 0.8
            ORDER BY 
                distance
            LIMIT 10
        )
        SELECT
            words.id,
            words.text
        FROM matches
        LEFT JOIN words ON words.id = matches.rowid;
        "#,
    )
    .bind(serde_json::to_string(note_average_content_embedding).unwrap())
    .fetch(word_embeddings_db)
    .try_collect()
    .await
    .map_err(|e| format!("Failed to search words {}", e))?;

    Ok(words.into_iter().map(|word| word.text).collect())
}

#[tauri::command]
pub async fn get_similar_words(
    state: tauri::State<'_, AppState>,
    note_id: i64,
) -> Result<Vec<String>, String> {
    let words = find_similar_words(state, note_id).await.unwrap();

    Ok(words)
}

#[tauri::command]
pub async fn find_similar_notes(
    state: tauri::State<'_, AppState>,
    note_id: i64,
) -> Result<Vec<Note>, String> {
    let db = &state.db;

    let notes: Vec<Note> = sqlx::query_as::<_, Note>(
        r#"
        WITH avg_embedding AS (
            SELECT 
                average_sentence_embedding
            FROM 
                notes
            WHERE 
                id = ?1
        ), matches AS (
            SELECT
                rowid,
                distance
            FROM 
                vec_note_chunks
            WHERE 
                sentence_embedding MATCH (SELECT average_sentence_embedding FROM avg_embedding)
                AND distance > 0.8
            ORDER BY 
                distance
            LIMIT 10
        )
        SELECT DISTINCT
            notes.id,
            notes.content,
            notes.average_sentence_embedding,
            notes.created_at,
            notes.updated_at
        FROM matches
        JOIN note_chunks ON note_chunks.id = matches.rowid
        LEFT JOIN notes ON notes.id = note_chunks.note_id;
        "#,
    )
    .bind(note_id)
    .fetch(db)
    .try_collect()
    .await
    .map_err(|e| format!("Failed to search notes {}", e))?;

    Ok(notes)
}

#[tauri::command]
pub async fn search_notes(
    state: tauri::State<'_, AppState>,
    query: String,
) -> Result<Vec<Note>, String> {
    let start = Instant::now();

    let db = &state.db;
    let sentence_encoder = &state.sentence_encoder;
    let sentences = [query.clone()];
    let output = sentence_encoder.encode(sentences.to_vec()).await.unwrap();
    let embedding_json = serde_json::to_string(&output[0]).unwrap();

    let notes: Vec<Note> = sqlx::query_as::<_, Note>(
        r#"
    WITH matches AS (
        SELECT
            rowid,
            distance
        FROM vec_note_chunks
        WHERE sentence_embedding MATCH (?1)
            AND distance > 0.8
        ORDER BY distance
        LIMIT 10
    )
    SELECT DISTINCT
        notes.id,
        notes.content,
        notes.average_sentence_embedding,
        notes.created_at,
        notes.updated_at
    FROM matches
    JOIN note_chunks ON note_chunks.id = matches.rowid
    LEFT JOIN notes ON notes.id = note_chunks.note_id"#,
    )
    .bind(embedding_json)
    .fetch(db)
    .try_collect()
    .await
    .map_err(|e| format!("Failed to search notes {}", e))?;

    let duration = start.elapsed();
    println!("Time elapsed in search_notes() is: {:?}", duration);

    Ok(notes)
}

// Delete note_chunks and vec_note_chunks
async fn delete_note_vector_embeddings(
    state: &tauri::State<'_, AppState>,
    note_id: i64,
) -> Result<(), String> {
    let db = &state.db;

    sqlx::query(
        "
        DELETE FROM vec_note_chunks
        WHERE rowid IN (
            SELECT id
            FROM note_chunks
            WHERE note_id = ?1
        );
        DELETE FROM note_chunks
        WHERE note_id = ?2
    ",
    )
    .bind(note_id)
    .bind(note_id)
    .execute(db)
    .await
    .map_err(|e| format!("could not delete note {}", e))?;

    Ok(())
}

async fn insert_note_vector_embeddings(
    state: &tauri::State<'_, AppState>,
    note_id: i64,
    note_content_chunks: &Vec<NoteChunkToInsert>,
) -> Result<(), String> {
    let db = &state.db;

    let mut insert_note_chunks_query_builder =
        sqlx::QueryBuilder::new("INSERT INTO note_chunks (sentence, sentence_embedding, note_id)");

    insert_note_chunks_query_builder.push_values(note_content_chunks, |mut b, chunk| {
        b.push_bind(&chunk.sentence)
            .push_bind(&chunk.sentence_embedding)
            .push_bind(note_id);
    });

    let insert_note_chunks_query = insert_note_chunks_query_builder.build();

    insert_note_chunks_query.execute(db).await.unwrap();

    sqlx::query(
        r#"
    WITH related_note_chunks AS (
        SELECT rowid, sentence_embedding
        FROM note_chunks
        WHERE "note_chunks"."note_id" = ?1
    )
    INSERT INTO vec_note_chunks (rowid, sentence_embedding)
    SELECT rowid, sentence_embedding
    FROM related_note_chunks;
    "#,
    )
    .bind(note_id)
    .execute(db)
    .await
    .unwrap();
    Ok(())
}

#[tauri::command]
pub async fn create_note(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    content: String,
) -> Result<i64, String> {
    let note_content_chunks = note_content_to_chunks(&state, &content).await;

    let average_sentence_embedding =
        compute_centroid_from_note_content_chunks(&note_content_chunks).unwrap();

    let db = &state.db;

    // notes
    let inserted_note =
        sqlx::query("INSERT INTO notes (content, average_sentence_embedding) VALUES (?1, ?2)")
            .bind(content.clone())
            .bind(sentence_embedding_to_json(&average_sentence_embedding))
            .execute(db)
            .await
            .unwrap();

    let inserted_note_row_id = inserted_note.last_insert_rowid();

    let _ = insert_note_vector_embeddings(&state, inserted_note_row_id, &note_content_chunks).await;

    let _ = app_handle.emit_all("refetch_notes", "");

    Ok(inserted_note_row_id)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchParams {
    tag_ids: Vec<String>,
    match_all: bool,
    take: Option<i64>,
    skip: Option<i64>,
}

#[tauri::command]
pub async fn get_notes(
    state: tauri::State<'_, AppState>,
    params: SearchParams,
) -> Result<Vec<Note>, String> {
    let db = &state.db;
    let tag_ids_count = params.tag_ids.len();

    // Create placeholders for the query
    let placeholders: Vec<String> = params
        .tag_ids
        .iter()
        .enumerate()
        .map(|(i, _)| format!("?{}", i + 1))
        .collect();
    let placeholders_str = placeholders.join(", ");

    // Build the query dynamically based on match_all flag
    let base_query_str = if params.tag_ids.len() == 0 {
        format!(
            "
            SELECT n.id, n.content, n.average_sentence_embedding, n.created_at, n.updated_at, t.id as tag_id
            FROM notes n
            LEFT JOIN notes_to_tags nt ON n.id = nt.note_id
            LEFT JOIN tags t ON nt.tag_id = t.id
        "
        )
    } else if params.match_all {
        format!(
            "WITH TagNotes AS (
                SELECT nt.note_id
                FROM notes_to_tags nt
                JOIN tags t ON nt.tag_id = t.id
                WHERE t.id IN ({})
                GROUP BY nt.note_id
                HAVING COUNT(DISTINCT t.id) = {}
            )
            SELECT n.id, n.content, n.average_sentence_embedding, n.created_at, n.updated_at, t.id as tag_id
            FROM notes n
            JOIN TagNotes tn ON n.id = tn.note_id
            LEFT JOIN notes_to_tags nt ON n.id = nt.note_id
            LEFT JOIN tags t ON nt.tag_id = t.id",
            placeholders_str, tag_ids_count
        )
    } else {
        format!(
            "SELECT DISTINCT n.id, n.content, n.average_sentence_embedding, n.created_at, n.updated_at, t.id as tag_id
            FROM notes n
            LEFT JOIN notes_to_tags nt ON n.id = nt.note_id
            LEFT JOIN tags t ON nt.tag_id = t.id
            WHERE nt.tag_id IN ({})",
            placeholders_str
        )
    };

    // Apply take and skip parameters
    let mut final_query_str = base_query_str.clone();

    final_query_str.push_str(
        "
    ORDER BY n.updated_at DESC
    ",
    );

    if let Some(take) = params.take {
        final_query_str.push_str(&format!(" LIMIT {}", take));
    }
    if let Some(skip) = params.skip {
        final_query_str.push_str(&format!(" OFFSET {}", skip));
    }

    // Prepare the query and bind parameters
    let mut query = sqlx::query_as::<_, NoteWithTag>(&final_query_str);
    for tag_id in &params.tag_ids {
        query = query.bind(tag_id);
    }

    // Execute the query and fetch results
    let rows: Vec<NoteWithTag> = match query.fetch_all(db).await {
        Ok(rows) => rows,
        Err(e) => return Err(e.to_string()),
    };

    // Process results to group tags under each note
    let mut notes_map: HashMap<i64, Note> = HashMap::new();
    for row in rows {
        let average_sentence_embedding =
            convert_blob_to_vec_f32(row.average_sentence_embedding).unwrap();

        let entry = notes_map.entry(row.id).or_insert(Note {
            id: row.id,
            content: row.content.clone(),
            average_sentence_embedding,
            created_at: row.created_at,
            updated_at: row.updated_at,
            tags: vec![],
        });

        if let Some(tag_id) = row.tag_id {
            entry.tags.push(Tag { id: tag_id });
        }
    }

    // Convert HashMap to Vec and sort by updated_at
    let mut notes: Vec<Note> = notes_map.into_values().collect();
    notes.sort_by_key(|note| std::cmp::Reverse(note.updated_at));

    Ok(notes)
}

#[tauri::command]
pub async fn get_note(state: tauri::State<'_, AppState>, id: i64) -> Result<Note, String> {
    let db = &state.db;

    // Fetch note along with its tags
    let rows: Vec<NoteWithTag> = sqlx::query_as::<_, NoteWithTag>(
        "
        SELECT n.id, n.content, n.average_sentence_embedding, n.created_at, n.updated_at, t.id as tag_id
        FROM notes n
        LEFT JOIN notes_to_tags nt ON n.id = nt.note_id
        LEFT JOIN tags t ON nt.tag_id = t.id
        WHERE n.id = ?1
        ",
    )
    .bind(id)
    .fetch_all(db)
    .await
    .map_err(|e| format!("Failed to get note: {}", e))?;

    if rows.is_empty() {
        return Err(format!("No note found with id: {}", id));
    }

    // Process results to group tags under the note
    let mut note = Note {
        id: rows[0].id,
        content: rows[0].content.clone(),
        average_sentence_embedding: convert_blob_to_vec_f32(
            rows[0].average_sentence_embedding.clone(),
        )
        .unwrap(),
        created_at: rows[0].created_at,
        updated_at: rows[0].updated_at,
        tags: vec![],
    };

    for row in rows {
        if let Some(tag_id) = row.tag_id {
            note.tags.push(Tag { id: tag_id });
        }
    }

    Ok(note)
}

#[tauri::command]
pub async fn update_note(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    id: i64,
    content: String,
) -> Result<(), String> {
    let start = Instant::now();

    let db = &state.db;

    let _ = delete_note_vector_embeddings(&state, id).await;

    let note_content_chunks = note_content_to_chunks(&state, &content).await;

    let _ = insert_note_vector_embeddings(&state, id, &note_content_chunks).await;

    let average_sentence_embedding =
        compute_centroid_from_note_content_chunks(&note_content_chunks).unwrap();

    let new_date = chrono::Utc::now().timestamp();
    sqlx::query("UPDATE notes SET content = ?1, average_sentence_embedding = ?2, updated_at = ?3 WHERE id = ?4")
        .bind(content)
        .bind(sentence_embedding_to_json(&average_sentence_embedding))
        .bind(new_date)
        .bind(id)
        .execute(db)
        .await
        .map_err(|e| format!("could not update note {}", e))?;

    println!("finished update_note");
    let duration = start.elapsed();
    println!("Time elapsed in update_note() is: {:?}", duration);

    let _ = app_handle.emit_all("refetch_notes", "");

    Ok(())
}

#[tauri::command]
pub async fn delete_note(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    id: i64,
) -> Result<(), String> {
    let db = &state.db;

    let _ = delete_note_vector_embeddings(&state, id.clone());

    sqlx::query("DELETE FROM notes WHERE id = ?1")
        .bind(id)
        .execute(db)
        .await
        .map_err(|e| format!("could not delete note {}", e))?;

    let _ = app_handle.emit_all("refetch_notes", "");

    Ok(())
}

#[tauri::command]
pub async fn delete_note_tag(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    note_id: i64,
    tag_id: String,
) -> Result<(), String> {
    let db = &state.db;

    sqlx::query("DELETE FROM notes_to_tags WHERE note_id = ?1 AND tag_id = ?2")
        .bind(note_id)
        .bind(tag_id)
        .execute(db)
        .await
        .map_err(|e| format!("could not delete note {}", e))?;

    let _ = app_handle.emit_all("refetch_notes", "");

    Ok(())
}
