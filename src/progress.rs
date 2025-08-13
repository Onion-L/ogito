use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub fn create_spinner(initial_message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ")
            .template("{spinner:.green} {msg}  [{elapsed_precise}]")
            .expect("failed to set progress style"),
    );
    pb.set_message(initial_message.to_string());
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}
