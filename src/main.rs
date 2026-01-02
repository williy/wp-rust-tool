use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // 読み込みたいWordPressの設定ファイルのパス（例）
    let path = "wp-config.php";

    println!("Searching for database configuration in {}...", path);

    if let Ok(file) = File::open(path) {
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            if let Ok(l) = line {
                // 'DB_NAME' という文字列が含まれる行を探す
                if l.contains("DB_NAME") {
                    println!("Found configuration: {}", l.trim());
                }
            }
        }
    } else {
        println!("Error: Could not find {}. Please place this tool in your WordPress root directory.", path);
    }
}