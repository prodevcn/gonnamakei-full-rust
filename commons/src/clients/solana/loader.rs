use std::marker::PhantomData;
use std::mem::{align_of, size_of};

use crate::anchor_client::anchor_lang::ZeroCopy;
use crate::solana_sdk::account::Account;

/// Struct to manage the zero-copy deserialization in client.
pub struct SolanaProgramAccount<T: ZeroCopy> {
    account: Account,
    phantom: PhantomData<T>,
}

impl<T: ZeroCopy> SolanaProgramAccount<T> {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(account: Account) -> SolanaProgramAccount<T> {
        Self {
            account,
            phantom: PhantomData::default(),
        }
    }

    // GETTERS ----------------------------------------------------------------

    pub fn account(&self) -> &Account {
        &self.account
    }

    // METHODS ----------------------------------------------------------------

    pub fn load_data(&self) -> Option<&T> {
        let data = self.account.data.as_slice();

        let mut disc_bytes = [0u8; 8];
        disc_bytes.copy_from_slice(&data[..8]);
        if disc_bytes != T::discriminator() {
            return None;
        }

        Some(try_from_bytes(&data[8..]))
    }
}

// ----------------------------------------------------------------------------
// Auxiliary methods ----------------------------------------------------------
// ----------------------------------------------------------------------------

pub fn try_from_bytes<T>(s: &[u8]) -> &T {
    if s.len() != size_of::<T>() || (s.as_ptr() as usize) % align_of::<T>() != 0 {
        panic!("Error in try_from_bytes of Solana ClientLoader")
    } else {
        unsafe { &*(s.as_ptr() as *const T) }
    }
}
