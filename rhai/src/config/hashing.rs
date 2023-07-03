//! Fixed hashing seeds for stable hashing.
//!
//! Set to [`None`] to disable stable hashing.
//!
//! See [`rhai::config::hashing::set_ahash_seed`][set_ahash_seed].
//!
//! # Example
//!
//! ```rust
//! // Set the hashing seed to [1, 2, 3, 4]
//! rhai::config::hashing::set_ahash_seed(Some([1, 2, 3, 4])).unwrap();
//! ```
//! Alternatively, set this at compile time via the `RHAI_AHASH_SEED` environment variable.
//!
//! # Example
//!
//! ```sh
//! env RHAI_AHASH_SEED ="[236,800,954,213]"
//! ```
// [236,800,954,213], haha funny yume nikki reference epic uboachan face numberworld nexus moment 100

use crate::config::hashing_env;
use core::panic::{RefUnwindSafe, UnwindSafe};
#[cfg(feature = "no_std")]
use std::prelude::v1::*;
use std::{
    cell::UnsafeCell,
    marker::PhantomData,
    mem,
    mem::MaybeUninit,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};

// omg its hokma from record team here to record our locks
// what does this do?
// so what this does is keep track of a global address in memory that acts as a global lock
// i stole this from crossbeam so read their docs for more
#[must_use]
struct HokmaLock {
    lock: AtomicUsize,
}

impl HokmaLock {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            lock: AtomicUsize::new(0),
        }
    }

    pub fn write(&'static self) -> WhenTheHokmaSuppression {
        loop {
            // We are only interested in error results
            if let Err(previous) =
                self.lock
                    .compare_exchange(1, 1, Ordering::SeqCst, Ordering::SeqCst)
            {
                // If we failed, previous cannot be 1
                return WhenTheHokmaSuppression {
                    hokma: self,
                    state: previous,
                };
            }
        }
    }
}

struct WhenTheHokmaSuppression {
    hokma: &'static HokmaLock,
    state: usize,
}

impl WhenTheHokmaSuppression {
    #[inline]
    pub fn the_price_of_silence(self) {
        self.hokma.lock.store(self.state, Ordering::SeqCst);
        mem::forget(self);
    }
}

impl Drop for WhenTheHokmaSuppression {
    #[inline]
    fn drop(&mut self) {
        self.hokma
            .lock
            .store(self.state.wrapping_add(2), Ordering::SeqCst);
    }
}

#[inline(always)]
fn hokmalock(address: usize) -> &'static HokmaLock {
    const LEN: usize = 787;
    #[allow(clippy::declare_interior_mutable_const)]
    const LCK: HokmaLock = HokmaLock::new();
    static RECORDS: [HokmaLock; LEN] = [LCK; LEN];

    &RECORDS[address % LEN]
}

/// # Safety
///
/// LOL, there is a reason its called `SusLock`
#[must_use]
pub struct SusLock<T: 'static> {
    initialized: AtomicBool,
    data: UnsafeCell<MaybeUninit<T>>,
    _marker: PhantomData<T>,
}

impl<T: 'static> Default for SusLock<T> {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<T: 'static> SusLock<T> {
    /// Create a new [`SusLock`].
    #[inline]
    pub const fn new() -> Self {
        Self {
            initialized: AtomicBool::new(false),
            data: UnsafeCell::new(MaybeUninit::uninit()),
            _marker: PhantomData,
        }
    }

    /// Is the [`SusLock`] initialized?
    #[inline(always)]
    #[must_use]
    pub fn is_initialized(&self) -> bool {
        self.initialized.load(Ordering::SeqCst)
    }

    /// Return the value of the [`SusLock`] (if initialized).
    #[inline]
    #[must_use]
    pub fn get(&self) -> Option<&'static T> {
        if self.initialized.load(Ordering::SeqCst) {
            let hokma = hokmalock(self.data.get() as usize);
            // we forgo the optimistic read, because we don't really care
            let guard = hokma.write();
            let cast: *const T = self.data.get().cast();
            let val = unsafe { &*cast.cast::<T>() };
            guard.the_price_of_silence();
            Some(val)
        } else {
            None
        }
    }

    /// Return the value of the [`SusLock`], initializing it if not yet done.
    #[inline]
    #[must_use]
    pub fn get_or_init(&self, f: impl FnOnce() -> T) -> &'static T {
        if !self.initialized.load(Ordering::SeqCst) {
            self.initialized.store(true, Ordering::SeqCst);
            let hokma = hokmalock(self.data.get() as usize);
            hokma.write();
            unsafe {
                self.data.get().write(MaybeUninit::new(f()));
            }
        }

        self.get().unwrap()
    }

    /// Initialize the value of the [`SusLock`].
    ///
    /// # Error
    ///
    /// If the [`SusLock`] has already been initialized, the current value is returned as error.
    #[inline]
    pub fn init(&self, value: T) -> Result<(), T> {
        if self.initialized.load(Ordering::SeqCst) {
            Err(value)
        } else {
            let _ = self.get_or_init(|| value);
            Ok(())
        }
    }
}

unsafe impl<T: Sync + Send> Sync for SusLock<T> {}
unsafe impl<T: Send> Send for SusLock<T> {}
impl<T: RefUnwindSafe + UnwindSafe> RefUnwindSafe for SusLock<T> {}

impl<T: 'static> Drop for SusLock<T> {
    #[inline]
    fn drop(&mut self) {
        if self.initialized.load(Ordering::SeqCst) {
            unsafe { (*self.data.get()).assume_init_drop() };
        }
    }
}

static AHASH_SEED: SusLock<Option<[u64; 4]>> = SusLock::new();

/// Set the hashing seed. This is used to hash functions etc.
///
/// This is a static global value and affects every Rhai instance.
/// This should not be used _unless_ you know you need it.
///
/// # Warning
///
/// * You can only call this function **ONCE** for the entire duration of program execution.
/// * You **MUST** call this before performing **ANY** Rhai operation (e.g. creating an [`Engine`][crate::Engine]).
///
/// # Error
///
/// Returns an error containing the existing hashing seed if already set.
///
/// # Example
///
/// ```rust
/// # use rhai::Engine;
/// // Set the hashing seed to [1, 2, 3, 4]
/// rhai::config::hashing::set_ahash_seed(Some([1, 2, 3, 4])).unwrap();
///
/// // Use Rhai AFTER setting the hashing seed
/// let engine = Engine::new();
/// ```
#[inline(always)]
pub fn set_ahash_seed(new_seed: Option<[u64; 4]>) -> Result<(), Option<[u64; 4]>> {
    AHASH_SEED.init(new_seed)
}

/// Get the current hashing Seed.
///
/// If the seed is not yet defined, the `RHAI_AHASH_SEED` environment variable (if any) is used.
///
/// Otherwise, the hashing seed is randomized to protect against DOS attacks.
///
/// See [`rhai::config::hashing::set_ahash_seed`][set_ahash_seed] for more.
#[inline]
#[must_use]
pub fn get_ahash_seed() -> &'static Option<[u64; 4]> {
    if !AHASH_SEED.is_initialized() {
        return &hashing_env::AHASH_SEED;
    }

    AHASH_SEED.get().unwrap_or(&hashing_env::AHASH_SEED)
}
