# Trackpad Haptic Feedback on Mac

[![Crates.io](https://img.shields.io/crates/v/trackpad_haptic.svg)](https://crates.io/crates/trackpad_haptic)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/jasonjmcghee/trackpad_haptic#license)

A simple interface into controlling the mac trackpad haptic feedback from rust.

I've provided an example of producing morse code.

```bash
cargo run --example morse
```

## Usage



```rust
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
```


## License

* [MIT License](LICENSE)
