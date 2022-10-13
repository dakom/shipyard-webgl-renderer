#[cfg(debug_assertions)]
pub mod gate {
    use std::sync::atomic::{AtomicBool, Ordering, AtomicUsize};

    static ONLY_ONCE: AtomicBool = AtomicBool::new(true);

    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    // quick helper to debug stuff. absolutely global
    // create separate functions for multiple checks
    pub fn only_once() -> bool {
        ONLY_ONCE.swap(false, Ordering::SeqCst)
    }

    pub fn until_count(count: usize) -> bool {
        COUNTER.fetch_add(1, Ordering::SeqCst) <= count
    }
}


#[cfg(not(debug_assertions))]
pub mod gate {
    pub fn only_once() -> bool {
        false
    }

    pub fn until_count() -> bool {
        false
    }
}
