// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
mod commands;
mod sentence_encoder;
mod word_vectors;
use commands::notes::{
    create_note, delete_note, delete_note_tag, find_similar_notes, get_note, get_notes,
    get_similar_words, search_notes, update_note,
};
use commands::tags::{create_tag, delete_tag, get_tags};
use sentence_encoder::SentenceEncoder;
use sqlite_vec::sqlite3_vec_init;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::env;
use std::fs::OpenOptions;
use std::path::PathBuf;
use tauri::{App, CustomMenuItem, Manager as _, Menu, MenuItem, Submenu, WindowBuilder};
use word_vectors::get_embeddings_path;

pub struct AppState {
    db: Db,
    word_embeddings_db: Db,
    sentence_encoder: SentenceEncoder,
    base_dir: PathBuf,
}

#[cfg(debug_assertions)]
fn setup_handler(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let app_handle = app.handle();

    println!(
        "{}",
        app_handle
            .path_resolver()
            .resource_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        app_handle
            .path_resolver()
            .app_config_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        app_handle
            .path_resolver()
            .app_data_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        app_handle
            .path_resolver()
            .app_local_data_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        app_handle
            .path_resolver()
            .app_cache_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        app_handle
            .path_resolver()
            .app_log_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::data_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::local_data_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::cache_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::config_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::executable_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::public_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::runtime_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::template_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::font_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::home_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::audio_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::desktop_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::document_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::download_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::picture_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );

    Ok(())
}

#[tokio::main]
async fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let preferences =
        CustomMenuItem::new("preferences".to_string(), "Preferences").accelerator("CmdOrCtrl+,");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit").accelerator("CmdOrCtrl+Q");

    let submenu = Submenu::new(
        "My Tauri App",
        Menu::new()
            .add_item(preferences)
            .add_native_item(MenuItem::Separator)
            .add_item(quit)
            .add_native_item(MenuItem::SelectAll)
            .add_native_item(MenuItem::Paste)
            .add_native_item(MenuItem::Copy),
    );
    let menu = Menu::new().add_submenu(submenu);

    let app = tauri::Builder::default()
        .setup(setup_handler)
        .menu(menu)
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "preferences" => {
                    // Code to open your preferences window
                    let app_handle = event.window().app_handle();
                    let _ = WindowBuilder::new(
                        &app_handle,
                        "preferences",
                        tauri::WindowUrl::App("settings".into()),
                    )
                    .title("Preferences")
                    .inner_size(400.0, 300.0)
                    .build();
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_similar_words,
            find_similar_notes,
            search_notes,
            create_note,
            get_notes,
            get_note,
            update_note,
            delete_note,
            delete_note_tag,
            create_tag,
            delete_tag,
            get_tags
        ])
        .build(tauri::generate_context!())
        .expect("error building the app");

    // let embeddings_path = get_embeddings_path(&app).unwrap();
    // println!("embeddings_path: {}", embeddings_path);

    // let start = Instant::now();

    // let embeddings = load_pretrained_embeddings(&embeddings_path);
    // let duration = start.elapsed();

    // println!("Time elapsed in expensive_function() is: {:?}", duration);

    // if let Some(embedding) = embeddings.get("example") {
    //     println!("Embedding for 'example': {:?}", embedding);
    // } else {
    //     println!("Word 'example' not found in embeddings.");
    // }

    let db = setup_db(&app).await;
    let word_embeddings_db = setup_word_embeddings_db(&app).await;

    let (_handle, sentence_encoder) = SentenceEncoder::spawn();

    app.manage(AppState {
        db,
        word_embeddings_db,
        sentence_encoder,
        base_dir: app
            .app_handle()
            .path_resolver()
            .app_local_data_dir()
            .unwrap_or(std::path::PathBuf::new()),
    });

    app.run(|_, _| {});
}

type Db = Pool<Sqlite>;

async fn setup_db(app: &App) -> Db {
    let mut path = app
        .path_resolver()
        .app_data_dir()
        .expect("could not get data_dir");

    println!("{:?}", path);

    // try to create application data directory if it doesn't exist
    match std::fs::create_dir_all(path.clone()) {
        Ok(_) => {}
        Err(err) => {
            panic!("error creating directory {}", err);
        }
    };

    path.push("db.sqlite");

    let result = OpenOptions::new().create_new(true).write(true).open(&path);

    match result {
        Ok(_) => println!("database file created"),
        Err(err) => match err.kind() {
            std::io::ErrorKind::AlreadyExists => println!("database file already exists"),
            _ => {
                panic!("error creating databse file {}", err);
            }
        },
    }

    unsafe {
        libsqlite3_sys::sqlite3_auto_extension(Some(std::mem::transmute(
            sqlite3_vec_init as *const (),
        )));
    }

    let db = SqlitePoolOptions::new()
        .connect(path.to_str().unwrap())
        .await
        .unwrap();

    sqlx::migrate!("./migrations").run(&db).await.unwrap();

    let version: (String,) = sqlx::query_as("SELECT sqlite_version();")
        .fetch_one(&db)
        .await
        .unwrap();
    let vec_version: (String,) = sqlx::query_as("SELECT vec_version();")
        .fetch_one(&db)
        .await
        .unwrap();

    println!("sqlite version: {:?}", version);
    println!("vec version: {:?}", vec_version);

    // sqlx::query(
    //     "
    //     PRAGMA busy_timeout = 60000;
    //     PRAGMA journal_mode = WAL;
    // ",
    // )
    // .execute(&db)
    // .await
    // .unwrap();

    db
}

async fn setup_word_embeddings_db(app: &App) -> Db {
    let path = get_embeddings_path(app).unwrap();

    println!("{:?}", path);

    unsafe {
        libsqlite3_sys::sqlite3_auto_extension(Some(std::mem::transmute(
            sqlite3_vec_init as *const (),
        )));
    }

    let db = SqlitePoolOptions::new().connect(&path).await.unwrap();

    let version: (String,) = sqlx::query_as("SELECT sqlite_version();")
        .fetch_one(&db)
        .await
        .unwrap();
    let vec_version: (String,) = sqlx::query_as("SELECT vec_version();")
        .fetch_one(&db)
        .await
        .unwrap();

    println!("word_embeddings sqlite version: {:?}", version);
    println!("word_embeddings vec version: {:?}", vec_version);

    db
}
