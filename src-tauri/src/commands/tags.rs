use crate::AppState;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt::Debug;
use tauri::Manager;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Tag {
    pub id: String,
}

#[tauri::command]
pub async fn create_tag(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    name: String,
    note_id: i64,
) -> Result<String, String> {
    let db = &state.db;

    sqlx::query(
        "
        INSERT OR REPLACE INTO tags (id) VALUES (?1);
        INSERT INTO notes_to_tags (note_id, tag_id) VALUES (?2, ?3);
    ",
    )
    .bind(&name)
    .bind(&note_id)
    .bind(&name)
    .execute(db)
    .await
    .unwrap();

    let _ = app_handle.emit_all("refetch_tags", "");
    Ok(name)
}

#[tauri::command]
pub async fn get_tags(
    state: tauri::State<'_, AppState>,
    note_id: Option<i64>,
    take: Option<u32>,
    skip: Option<u32>,
) -> Result<Vec<Tag>, String> {
    let db = &state.db;

    let base_query_str = if let Some(note_id) = note_id {
        format!(
            "
            SELECT *
            FROM tags
            JOIN notes_to_tags ON tags.id = notes_to_tags.tag_id
            WHERE notes_to_tags.note_id = {}
            LIMIT ?1
            OFFSET ?2
            ",
            note_id
        )
    } else {
        format!(
            "
            SELECT * FROM tags
            LIMIT ?1
            OFFSET ?2
        "
        )
    };

    let tags: Vec<Tag> = sqlx::query_as::<_, Tag>(&base_query_str)
        .bind(take.unwrap_or(50))
        .bind(skip.unwrap_or(0))
        .fetch(db)
        .try_collect()
        .await
        .map_err(|e| format!("Failed to get tags {}", e))?;

    Ok(tags)
}

#[tauri::command]
pub async fn delete_tag(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let db = &state.db;

    sqlx::query("DELETE FROM tags WHERE id = ?1")
        .bind(&id)
        .execute(db)
        .await
        .unwrap();

    let _ = app_handle.emit_all("refetch_tags", "");
    Ok(())
}
