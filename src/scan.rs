use std::{
    collections::HashMap,
    os::unix::fs::MetadataExt,
    path::Path,
    time::{Duration, SystemTime},
};

use walkdir::WalkDir;

#[derive(Debug)]
pub struct ScanSummary {
    pub file_count: u64,
    pub directory_count: u64,
    pub total_bytes: u64,
    pub extensions: HashMap<String, u64>,
    pub old_files: u64,
}

impl ScanSummary {
    pub fn increment_extension(&mut self, ext: String) {
        let count = self.extensions.entry(ext).or_insert(0);
        *count += 1;
    }

    pub fn print_summary(&self) {
        println!("=== Scan Summary ===");
        println!("{:<15} {:>10}", "Files:", self.file_count);
        println!("{:<15} {:>10}", "Directories:", self.directory_count);
        println!("{:<15} {:>10} bytes", "Total size:", self.total_bytes);
        println!("{:<15} {:>10}", "Old files (>180d):", self.old_files);

        if !self.extensions.is_empty() {
            println!("\nFile extensions (sorted by count):");
            let mut exts: Vec<_> = self.extensions.iter().collect();
            exts.sort_by(|a, b| b.1.cmp(a.1));

            for (ext, count) in exts {
                println!("  {:<12} {:>6}", ext, count);
            }
        }
        println!("====================");
    }
}

pub fn scan(path: &Path, recursive: bool) {
    let walker = WalkDir::new(path);

    let walker = if !recursive {
        walker.max_depth(1)
    } else {
        walker
    };

    let mut summary = ScanSummary {
        file_count: 0,
        directory_count: 0,
        total_bytes: 0,
        extensions: HashMap::new(),
        old_files: 0,
    };

    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        let metadata = entry.metadata().ok();
        let old_threshold = SystemTime::now() - Duration::from_secs(180 * 24 * 60 * 60);

        match entry.file_type() {
            t if t.is_file() => {
                summary.file_count += 1;

                let ext = entry
                    .path()
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|s| format!(".{}", s))
                    .unwrap_or_else(|| "Other".to_string());

                *summary.extensions.entry(ext).or_insert(0) += 1;

                if let Some(m) = metadata {
                    summary.total_bytes += m.size();

                    if m.modified().ok().filter(|&t| t < old_threshold).is_some() {
                        summary.old_files += 1;
                    }
                }
            }
            t if t.is_dir() => {
                summary.directory_count += 1;
            }
            _ => {}
        }
    }

    summary.print_summary();
}
