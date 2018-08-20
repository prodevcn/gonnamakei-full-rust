use std::path::Path;

pub async fn write_all_file_path(path: &Path, content: &[u8]) -> tokio::io::Result<()> {
    let parent = path.parent().unwrap();

    tokio::fs::create_dir_all(parent).await?;
    tokio::fs::write(&path, content).await
}
