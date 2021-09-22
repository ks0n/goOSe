use std::env;
use std::error::Error;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;

use serde::Deserialize;

static ENV_BOARD: &str = "GOOSE_BOARD";

#[derive(Deserialize)]
struct Board {
    target: String,
    linker_script: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let board_path = env::var(ENV_BOARD)?;
    let board_file_name = format!("{}/{}", board_path, "config.toml");
    let board_config = fs::read_to_string(board_file_name)?;

    let mut board: Board = toml::from_str(board_config.as_str())?;
    board.linker_script = format!("{}/{}", board_path, board.linker_script);

    let mut config_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(".cargo/config")?;

    let config_content = format!(
        r#"
[build]
target = "{}"
rustflags = ["-C", "link-arg=-T{}"]
"#,
        board.target, board.linker_script
    );

    config_file.write(config_content.as_bytes())?;
    config_file.sync_all()?;

    println!("cargo:rerun-if-changed=.cargo/config");
    println!("cargo:warning=The first invocation of `cargo build`. Just rerun it");

    Ok(())
}
