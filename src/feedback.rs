use std::thread::sleep;
use std::time::Duration;
use icrate::AppKit::{
    NSApplication,
    NSApplicationActivationPolicyRegular,
    NSHapticFeedbackManager,
    NSHapticFeedbackPerformanceTimeNow,
    NSHapticFeedbackPerformer
};
use icrate::Foundation::MainThreadMarker;
use icrate::objc2::__framework_prelude::{Id, ProtocolObject};

const TIME_UNIT: u64 = 10;
const UNIT_DELAY: u64 = 10;

pub struct Feedback {
    length_millis: u64,
    delay_millis: u64
}

impl Feedback {
    pub fn with_delay(delay_millis: u64) -> Self {
        Self {
            length_millis: TIME_UNIT,
            delay_millis
        }
    }

    pub fn new(length_millis: u64, delay_millis: u64) -> Self {
        Self {
            length_millis,
            delay_millis
        }
    }
}

pub struct FeedbackManager {
    manager: Id<ProtocolObject<dyn NSHapticFeedbackPerformer>>,
}

impl Default for FeedbackManager {
    fn default() -> Self {
        Self::initialize();
        Self::new()
    }
}

impl FeedbackManager {
    fn initialize() {
        // Ensure this is run on the main thread
        let main_thread_marker = MainThreadMarker::new().expect("Must be run on the main thread");

        // Initialize the shared application instance
        let app = NSApplication::sharedApplication(main_thread_marker);

        // Set the activation policy
        app.setActivationPolicy(NSApplicationActivationPolicyRegular);
    }

    pub fn new() -> Self {
        Self {
            manager: unsafe {
                NSHapticFeedbackManager::defaultPerformer()
            }
        }
    }

    pub fn trigger(&self) {
        self.trigger_with_feedback(
            Feedback::with_delay(0)
        )
    }

    pub fn trigger_with_delay(&self, delay_millis: u64) {
        self.trigger_with_feedback(
            Feedback::with_delay(delay_millis)
        )
    }

    pub fn trigger_with_feedback(&self, feedback: Feedback) {
        unsafe {
            for _ in 0..(feedback.length_millis / TIME_UNIT) {
                self.manager.performFeedbackPattern_performanceTime(
                    2,
                    NSHapticFeedbackPerformanceTimeNow,
                );
                sleep(Duration::from_millis(UNIT_DELAY));
            }

            if feedback.delay_millis > 0 {
                sleep(Duration::from_millis(feedback.delay_millis));
            }
        }
    }
}