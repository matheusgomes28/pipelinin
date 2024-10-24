use std::time::Duration;

use indicatif::ProgressBar;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn show_loading_bar() {
    let max_count = 1000;

    let bar = ProgressBar::new(max_count);
    for _ in 0..max_count {
        bar.inc(1);
        std::thread::sleep(Duration::from_micros(750));
    }
    bar.finish();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
