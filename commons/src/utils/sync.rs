use std::ops::Deref;
use std::sync::{Arc, Weak};

use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Debug)]
pub struct AsyncRwArc<T: Send>(Arc<RwLock<T>>);

impl<T: Send> AsyncRwArc<T> {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(value: T) -> AsyncRwArc<T> {
        AsyncRwArc(Arc::new(RwLock::new(value)))
    }

    // METHODS ----------------------------------------------------------------

    pub async fn read(&self) -> RwLockReadGuard<'_, T> {
        self.0.read().await
    }

    pub fn read_sync(&self) -> RwLockReadGuard<'_, T> {
        futures::executor::block_on(async { self.read().await })
    }

    pub async fn write(&self) -> RwLockWriteGuard<'_, T> {
        self.0.write().await
    }

    pub fn write_sync(&self) -> RwLockWriteGuard<'_, T> {
        futures::executor::block_on(async { self.write().await })
    }

    pub fn weak(&self) -> AsyncRwWeak<T> {
        AsyncRwWeak::new(Arc::downgrade(&self.0))
    }

    pub fn ptr_eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl<T: Send> Clone for AsyncRwArc<T> {
    fn clone(&self) -> Self {
        AsyncRwArc(self.0.clone())
    }
}

impl<T: Send + Clone> AsyncRwArc<T> {
    pub async fn read_clone(&self) -> Self {
        let self_lock = self.read().await;
        let reference = self_lock.deref();
        AsyncRwArc::new(reference.clone())
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct AsyncRwWeak<T: Send>(Weak<RwLock<T>>);

impl<T: Send> AsyncRwWeak<T> {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new_empty() -> AsyncRwWeak<T> {
        AsyncRwWeak(Weak::new())
    }

    fn new(value: Weak<RwLock<T>>) -> AsyncRwWeak<T> {
        AsyncRwWeak(value)
    }

    // METHODS ----------------------------------------------------------------

    pub fn upgrade(&self) -> Option<AsyncRwArc<T>> {
        self.0.upgrade().map(AsyncRwArc)
    }

    pub fn ptr_eq(&self, other: &Self) -> bool {
        self.0.ptr_eq(&other.0)
    }
}

impl<T: Send> Clone for AsyncRwWeak<T> {
    fn clone(&self) -> Self {
        AsyncRwWeak(self.0.clone())
    }
}
