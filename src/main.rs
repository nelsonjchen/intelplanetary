use linemux::MuxedLines;
use log::info;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let log_file_location = r#"C:\Program Files (x86)\Steam\steamapps\common\Interplanetary Enhanced Edition\Interplanetary_Data\output_log.txt"#;

    let mut lines = MuxedLines::new()?;

    info!("Attempting to listen to {}", log_file_location);

    // Register some files to be tailed, whether they currently exist or not.
    lines.add_file(log_file_location).await?;

    info!("Listening to {}", log_file_location);

    // Wait for `Line` event, which contains the line captured for a given
    // source path.
    while let Ok(Some(line)) = lines.next_line().await {
        info!("source: {}, line: {}", line.source().display(), line.line());
    }
    Ok(())
}
