use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct StopSignal {
    stop: Arc<AtomicBool>,
}

impl StopSignal {
    pub fn new() -> Self {
        StopSignal {
            stop: Arc::new(AtomicBool::new(false)),
        }
    }
    pub fn check_stop(&self) -> bool {
        self.stop.load(Ordering::SeqCst)
    }
    pub fn stop_fn(&self) -> impl Fn() {
        let stop_clone = self.stop.clone();
        move || {
            stop_clone.store(true, Ordering::SeqCst);
        }
    }
}
