use std::{
    cmp::min,
    collections::HashMap,
    os::unix::fs::MetadataExt,
    path::Path,
    time::{Duration, Instant, SystemTime},
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
    pub fn print_summary(&self, ext_display_limit: u64, scan_time: Duration) {
        // Collect and sort extensions by count descending
        let mut ext_vec: Vec<_> = self.extensions.iter().collect();
        ext_vec.sort_by(|a, b| b.1.cmp(a.1));

        let labels = ["Files", "Directories", "Size", "Old files"];
        let values = [
            self.file_count.to_string(),
            self.directory_count.to_string(),
            format!("{} bytes", self.total_bytes),
            self.old_files.to_string(),
        ];

        // Compute widths for alignment
        let max_label_width = labels.iter().map(|l| l.len()).max().unwrap_or(0);
        let max_value_width = values.iter().map(|v| v.len()).max().unwrap_or(0);
        let total_width = max_label_width + 3 + max_value_width;

        println!("Scan Summary");
        println!("{}", "-".repeat(total_width));
        for (label, value) in labels.iter().zip(values.iter()) {
            println!(
                "{:<label_w$} : {:>value_w$}",
                label,
                value,
                label_w = max_label_width,
                value_w = max_value_width
            );
        }
        println!("{}", "-".repeat(total_width));

        let limit = ext_display_limit as usize;
        let max_ext_width = ext_vec
            .iter()
            .take(limit)
            .map(|(ext, _)| ext.len())
            .max()
            .unwrap_or(0);
        let max_count_width = ext_vec
            .iter()
            .take(limit)
            .map(|(_, count)| count.to_string().len())
            .max()
            .unwrap_or(0);
        let ext_table_width = max_ext_width + 3 + max_count_width;

        println!("\nTop Extensions");
        println!("{}", "-".repeat(ext_table_width));
        for (ext, count) in ext_vec.iter().take(limit) {
            println!(
                "{:<ext_w$} : {:>count_w$}",
                ext,
                count,
                ext_w = max_ext_width,
                count_w = max_count_width
            );
        }
        println!("{}", "-".repeat(ext_table_width));

        println!("Scan took {} ms.", scan_time.as_millis());
    }
}

pub fn scan(path: &Path, recursive: bool, ext_display_limit: u64) {
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

    let old_threshold = SystemTime::now() - Duration::from_secs(180 * 24 * 60 * 60);
    let now = Instant::now();

    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        let metadata = entry.metadata().ok();

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

    summary.print_summary(ext_display_limit, now.elapsed());
}
