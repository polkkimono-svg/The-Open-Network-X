use onx_genesis::{generate_genesis, parse_config, write_docs};
use std::env;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let config_path = args
        .iter()
        .position(|arg| arg == "--config")
        .and_then(|idx| args.get(idx + 1).cloned())
        .unwrap_or_else(|| "genesis.toml".to_string());
    let output_dir = args
        .iter()
        .position(|arg| arg == "--out")
        .and_then(|idx| args.get(idx + 1).cloned())
        .unwrap_or_else(|| "target/onx-genesis".to_string());

    let config = match parse_config(&config_path) {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("onx-genesis failed: {err}");
            std::process::exit(1);
        }
    };

    if let Err(err) = generate_genesis(&config, PathBuf::from(&output_dir)) {
        eprintln!("onx-genesis failed: {err}");
        std::process::exit(1);
    }

    if let Err(err) = write_docs(PathBuf::from(&output_dir)) {
        eprintln!("onx-genesis failed: {err}");
        std::process::exit(1);
    }
}
