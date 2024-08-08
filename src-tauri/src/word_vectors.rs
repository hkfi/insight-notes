use tauri::api::path::resource_dir;
use tauri::Manager;

pub fn get_embeddings_path(app: &tauri::App) -> Result<String, String> {
    let resource_path = app.path_resolver();
    println!("resource_path 1: {:?}", resource_path.resource_dir());

    if let Some(resource_path) = resource_dir(app.package_info(), &app.env()) {
        println!("resouce_path: {:?}", resource_path.to_str());
        // let path = resource_path.join("pretrained_embeddings/glove.6B.300d.txt");
        let path = resource_path.join("pretrained_embeddings/word_embeddings.sqlite");
        if path.exists() {
            Ok(path.to_string_lossy().into_owned())
        } else {
            Err("Embeddings file not found".into())
        }
    } else {
        Err("Resource directory not found".into())
    }
}
