use pangu::spacing; // <--- FIX 1: Crate name is 'pangu'
use rayon::prelude::*;
use std::borrow::Cow; // <--- FIX 2: Import Cow
use std::env;
use std::fs;
use std::io::{BufWriter, Write};
use std::process;
use std::str;
use std::time::Instant;

// 定義區塊大小為 8MB
const CHUNK_SIZE: usize = 8 * 1024 * 1024;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("用法: {} <輸入檔案> <輸出檔案>", args[0]);
        process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    println!("(並行模式) 正在讀取檔案: {}", input_path);
    let start = Instant::now();

    let content_bytes = match fs::read(input_path) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("讀取檔案 '{}' 失敗: {}", input_path, e);
            process::exit(1);
        }
    };

    let read_duration = start.elapsed();
    println!("檔案讀取完成，耗時: {:?}", read_duration);
    println!("開始並行處理 {} 個位元組...", content_bytes.len());

    let processed_chunks: Vec<String> = content_bytes
        .par_chunks(CHUNK_SIZE)
        .map(|chunk| {
            // <--- FIX 2: Use Cow<str> to handle both borrowed and owned strings
            let text_chunk: Cow<str> = match str::from_utf8(chunk) {
                Ok(s) => Cow::Borrowed(s),
                Err(_) => String::from_utf8_lossy(chunk),
            };

            text_chunk
                .lines()
                .map(|line| spacing(line))
                .collect::<Vec<_>>()
                .join("\n")
        })
        .collect();

    let process_duration = start.elapsed() - read_duration;
    println!("並行處理完成，耗時: {:?}", process_duration);

    println!("正在寫入檔案: {}", output_path);
    let output_file = fs::File::create(output_path).expect("無法建立輸出檔案");
    let mut writer = BufWriter::new(output_file);

    for chunk in processed_chunks {
        if let Err(e) = writeln!(writer, "{}", chunk) {
            eprintln!("寫入區塊時發生錯誤: {}", e);
            process::exit(1);
        }
    }
    
    writer.flush().expect("刷新緩衝區失敗");

    let total_duration = start.elapsed();
    println!(
        "全部任務完成，總耗時: {:?}, 結果已儲存至 '{}'",
        total_duration,
        output_path
    );
}