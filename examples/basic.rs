use std::thread;
use std::time::Duration;
use trackpad_haptic::{Feedback, FeedbackManager};

fn main() {
    let haptic_manager = FeedbackManager::default();
    loop {
        // Shortest possible
        haptic_manager.trigger();
        thread::sleep(Duration::from_secs(1));

        // 1 second of continuous feedback
        let length_millis = 1000;
        let delay_millis = 1000;
        haptic_manager.trigger_with_feedback(
            Feedback::new(length_millis, delay_millis)
        );
    }
}