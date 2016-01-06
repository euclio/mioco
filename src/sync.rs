use std::sync as ssync;
use std::fmt;

mod mioco {
    pub use super::super::*;
}

/// A reader-writer lock
///
/// Based on `std::sync::RwLock`. Calls `mioco::yield_now()` on contention.
#[derive(Debug)]
pub struct RwLock<T: ?Sized> {
    lock : ssync::RwLock<T>
}

impl<T> RwLock<T> {
    /// Creates a new instance of an RwLock<T> which is unlocked.
    pub fn new(t : T) -> Self {
        RwLock {
            lock: ssync::RwLock::new(t),
        }
    }
}

impl<T : ?Sized> RwLock<T> {
    /// Get a reference to raw `std::sync::RwLock`.
    ///
    /// Use it to perform operations outside of mioco coroutines.
    pub fn native_lock(&self) -> &ssync::RwLock<T> {
        &self.lock
    }

    /// Locks this rwlock with shared read access, blocking the current
    /// coroutine until it can be acquired.
    pub fn read(&self) -> ssync::LockResult<ssync::RwLockReadGuard<T>> {
        loop {
            match self.lock.try_read() {
                Ok(guard)  => {
                    return Ok(guard)
                },
                Err(try_error) => {
                    match try_error {
                        ssync::TryLockError::Poisoned(p_err) => {
                            return Err(p_err);
                        },
                        ssync::TryLockError::WouldBlock => {
                            mioco::yield_now()
                        }
                    }
                }
            }
        }
    }

    /// Attempts to acquire this rwlock with shared read access.
    pub fn try_read(&self) -> ssync::TryLockResult<ssync::RwLockReadGuard<T>> {
        self.lock.try_read()
    }

    /// Locks this rwlock with exclusive write access, blocking the current
    /// coroutine until it can be acquired.
    pub fn write(&self) -> ssync::LockResult<ssync::RwLockWriteGuard<T>> {
        loop {
            match self.lock.try_write() {
                Ok(guard) => {
                    return Ok(guard)
                },
                Err(try_error) => {
                    match try_error {
                        ssync::TryLockError::Poisoned(p_err) => {
                            return Err(p_err);
                        },
                        ssync::TryLockError::WouldBlock => {
                            mioco::yield_now()
                        }
                    }
                }
            }
        }
    }


    /// Attempts to lock this rwlock with exclusive write access.
    pub fn try_write(&self) -> ssync::TryLockResult<ssync::RwLockWriteGuard<T>> {
        self.lock.try_write()
    }

    /// Determines whether the lock is poisoned.
    pub fn is_poisoned(&self) -> bool {
        self.lock.is_poisoned()
    }
}

/// A Mutex
///
/// Based on `std::sync::Mutex`. Calls `mioco::yield_now()` on contention.
pub struct Mutex<T: ?Sized> {
    lock : ssync::Mutex<T>
}

impl<T: ?Sized + fmt::Debug + 'static> fmt::Debug for Mutex<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.lock.fmt(f)
    }
}

impl<T> Mutex<T> {
    /// Creates a new instance of an Mutex<T> which is unlocked.
    pub fn new(t : T) -> Self {
        Mutex {
            lock: ssync::Mutex::new(t),
        }
    }
}

impl<T : ?Sized> Mutex<T> {
    /// Get a reference to raw `std::sync::Mutex`.
    ///
    /// Use it to perform operations outside of mioco coroutines.
    pub fn native_lock(&self) -> &ssync::Mutex<T> {
        &self.lock
    }

    /// Acquire a mutex, blocking the current coroutine until it is able to do so.
    pub fn lock(&self) -> ssync::LockResult<ssync::MutexGuard<T>> {
        loop {
            match self.lock.try_lock() {
                Ok(guard)  => {
                    return Ok(guard)
                },
                Err(try_error) => {
                    match try_error {
                        ssync::TryLockError::Poisoned(p_err) => {
                            return Err(p_err);
                        },
                        ssync::TryLockError::WouldBlock => {
                            mioco::yield_now()
                        }
                    }
                }
            }
        }
    }

    /// Attempt to acquire this lock.
    pub fn try_lock(&self) -> ssync::TryLockResult<ssync::MutexGuard<T>> {
        self.lock.try_lock()
    }
}
