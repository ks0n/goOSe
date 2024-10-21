use core::marker::PhantomData;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use lock_api::{GuardSend, RawMutex};

use crate::CoreInfo;

pub struct RawReentrantSpinlock<G> {
    user: AtomicUsize,
    lock: AtomicBool,
    _marker: PhantomData<G>,
}

unsafe impl<G: CoreInfo> RawMutex for RawReentrantSpinlock<G> {
    // The underlying const with interior mutability is fine because it is only used for
    // construction.
    // Clippy recommends using a const fn for constructors but I don't have that freedom of choice
    // since we depend on lock_api.
    #[allow(clippy::declare_interior_mutable_const)]
    const INIT: RawReentrantSpinlock<G> = RawReentrantSpinlock::<G> {
        user: AtomicUsize::new(usize::MAX),
        lock: AtomicBool::new(false),
        _marker: PhantomData,
    };

    // A spinlock guard can be sent to another thread and unlocked there
    type GuardMarker = GuardSend;

    fn lock(&self) {
        // Note: This isn't the best way of implementing a spinlock, but it
        // suffices for the sake of this example.
        while !self.try_lock() {}
    }

    fn try_lock(&self) -> bool {
        let my_id = G::core_id();

        if self.user.load(Ordering::Acquire) == my_id {
            assert!(self.lock.load(Ordering::Relaxed));

            // Already locked by myself, reenter the spinlock.
            log::debug!(
                "RawReentrantSpinlock::try_lock: reentering (core 0x{:X})",
                my_id
            );
            return true;
        }

        // Try to lock the mutex and when it is done store our id in it.
        self.lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
            && self
                .user
                .compare_exchange(usize::MAX, my_id, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
    }

    unsafe fn unlock(&self) {
        self.user.store(usize::MAX, Ordering::Release);
        self.lock.store(false, Ordering::Release);
    }
}

pub type ReentrantSpinlock<G, T> = lock_api::Mutex<RawReentrantSpinlock<G>, T>;
