use std::fs;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::time::Instant;
use zip;
use clap::Parser;

/// Simple that scans and zips files
#[derive(Parser, Debug)]
#[command()]
struct Args {
    /// Compression level, 0 = none (fastest, biggest), 1 = stored (slower, smaller), 2 = deflated (slowest, smallest)
    #[arg(short, long, default_value_t = 1)]
    compression: u8,

    /// Max file size in MB to scan for
    #[arg(short, long, default_value_t = 5)]
    max_size: u64,

    /// Scan drives
    #[arg(short, long, default_value_t = false)]
    drives: bool,
}

fn main() -> io::Result<()> {
    let compressions = vec![
        zip::CompressionMethod::Stored,
        zip::CompressionMethod::Deflated,
        zip::CompressionMethod::Bzip2,
    ];

    if env::consts::OS != "windows" {
        println!("Only supported on Windows");
        return Ok(());
    }

    let args = Args::parse();

    let compression: u8 = args.compression;
    let scandrives: bool = args.drives;
    let small_scan: bool = true;

    let all_start_time = Instant::now();

    let home = dirs::home_dir().unwrap();
    let home_str = home.to_str().unwrap();

    let dirs = vec![
        "Documents",
        "Downloads",
        "Pictures",
        "Music",
        "Videos",
        "Desktop",
    ];

    let mut dir_paths: Vec<String> = dirs
        .iter()
        .map(|dir| format!("{}\\{}", home_str, dir))
        .collect();

    let possible_drives = vec![
        "A:/", "B:/", "D:/", "E:/", "F:/", "G:/", "H:/", "I:/", "J:/", "K:/", "L:/", "M:/", "N:/",
        "O:/", "P:/", "Q:/", "R:/", "S:/", "T:/", "U:/", "V:/", "W:/", "X:/", "Y:/", "Z:/",
    ];

    let drives: Vec<String> = possible_drives
        .iter()
        .filter(|&&drive| Path::new(drive).exists())
        .map(|&drive| drive.to_string())
        .collect();

    if scandrives {
        dir_paths.extend_from_slice(&drives);
    }

    if small_scan {
        dir_paths = vec![format!("{}\\Pictures", home_str)];
    }

    let allowed_extensions = vec![
        "jpg", "jpeg", "png", "gif", "py", "txt", "mp3", "mp4", "pdf", "json", "docx", "js",
        "html", "css", "ts",
    ];
    let max_file_size: u64 = args.max_size * 1024 * 1024;

    let blacklist_dirs = vec![
        ".wrangler",
        ".git",
        "node_modules",
        ".vscode",
        ".rustup",
    ];

    let mut all_files: Vec<String> = Vec::new();
    let mut file_sizes: u64 = 0;

    fn handle_dir(
        location: &str,
        allowed_extensions: &[&str],
        max_file_size: u64,
        blacklist_dirs: &[&str],
        all_files: &mut Vec<String>,
        file_sizes: &mut u64,
    ) -> io::Result<()> {
        let walker = fs::read_dir(location)?;

        for entry in walker {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if allowed_extensions
                        .contains(&extension.to_string_lossy().to_lowercase().as_ref())
                        && path.metadata()?.len() <= max_file_size
                        && !blacklist_dirs
                            .iter()
                            .any(|dir| path.to_string_lossy().contains(dir))
                    {
                        all_files.push(path.to_string_lossy().to_string());
                        *file_sizes += path.metadata()?.len();
                    }
                }
            } else if path.is_dir() {
                handle_dir(
                    &path.to_string_lossy(),
                    allowed_extensions,
                    max_file_size,
                    blacklist_dirs,
                    all_files,
                    file_sizes,
                )?;
            }
        }
        Ok(())
    }

    println!("Scanning...");
    let scan_start_time = Instant::now();

    for location in dir_paths {
        println!("Scanning {}", location);

        handle_dir(
            &location,
            &allowed_extensions,
            max_file_size,
            &blacklist_dirs,
            &mut all_files,
            &mut file_sizes,
        )?;
    }

    println!(
        "Scanned in {:.3} seconds",
        scan_start_time.elapsed().as_secs_f64()
    );

    println!("Zipping files...");
    let zip_time_start = Instant::now();

    let mut zip = File::create("files.zip")?;
    let mut archive = zip::ZipWriter::new(&mut zip);
    let options =
        zip::write::FileOptions::default().compression_method(compressions[compression as usize]);

    for item in all_files.iter() {
        match File::open(item) {
            Ok(mut file) => {
                if let Some(file_name) = Path::new(item).file_name() {
                    let file_name_str = file_name
                        .to_string_lossy()
                        .to_string()
                        .replace('\\', "_____");
                    if let Err(_) = archive.start_file(file_name_str, options) {
                        println!("{}", item);
                        continue;
                    }
                }
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)?;
                if let Err(_) = archive.write_all(&*buffer) {
                    println!("{}", item);
                }
            }
            Err(_) => println!("{}", item),
        }
    }

    archive.finish()?;
    println!(
        "Zipped in {:.3} seconds",
        zip_time_start.elapsed().as_secs_f64()
    );

    println!(
        "Done in {:.3} seconds",
        all_start_time.elapsed().as_secs_f64()
    );

    println!("--------------------");
    println!("Total files saved: {}", all_files.len());
    println!("Total size: {:.1} MB", file_sizes as f64 / 1024.0 / 1024.0);

    Ok(())
}
