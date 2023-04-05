use indicatif::{ProgressBar, ProgressStyle};
use std::{thread, time};

fn main() {
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.cyan} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .expect("error"),
    );

    for i in 0..=100 {
        pb.set_position(i);
        thread::sleep(time::Duration::from_millis(50));
    }

    pb.finish_with_message("Task done!");
}
