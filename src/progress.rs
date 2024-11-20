// progress.rs
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
// use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
// use std::time::Duration;

pub struct ProgressTracker {
    multi_progress: MultiProgress,
    total_files: Arc<AtomicUsize>,
    processed_files: Arc<AtomicUsize>,
}

impl ProgressTracker {
    pub fn new() -> Self {
        ProgressTracker {
            multi_progress: MultiProgress::new(),
            total_files: Arc::new(AtomicUsize::new(0)),
            processed_files: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn create_progress_bar(&self, len: u64, desc: &str) -> ProgressBar {
        let pb = self.multi_progress.add(ProgressBar::new(len));
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
            .unwrap()
            .progress_chars("#>-"));
        pb.set_message(desc.to_string());
        pb
    }

    pub fn increment_total_files(&self) {
        self.total_files.fetch_add(1, Ordering::SeqCst);
    }

    pub fn increment_processed_files(&self) {
        self.processed_files.fetch_add(1, Ordering::SeqCst);
    }

    pub fn get_progress(&self) -> (usize, usize) {
        (
            self.processed_files.load(Ordering::SeqCst),
            self.total_files.load(Ordering::SeqCst),
        )
    }
}
