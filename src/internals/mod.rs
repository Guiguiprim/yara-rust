use std::sync::Mutex;

use lazy_static::lazy_static;
pub use yara_sys;

use crate::errors::*;

pub use self::compiler::*;
pub use self::configuration::*;
pub use self::iterator::*;
pub use self::rules::*;
pub use self::scan::*;

pub mod matches;
pub mod meta;
pub mod string;

mod compiler;
mod configuration;
mod iterator;
mod rules;
mod scan;
mod stream;

lazy_static! {
    static ref INIT_MUTEX: Mutex<()> = Mutex::new(());
}

/// Initialize the Yara library
///
/// Can be called multiple times without problems.
/// Is thread safe.
///
/// # Implementation details
///
/// yr_initialize is not actually thread safe, because it use an `int` to count the number of
/// initialization and finalization, instead of an atomic int. Thus, we lock a global mutex each
/// time we call yr_initialize and yr_finalize.
pub fn initialize() -> Result<(), YaraError> {
    let _guard = INIT_MUTEX.lock();
    let result = unsafe { yara_sys::yr_initialize() };

    yara_sys::Error::from_code(result).map_err(Into::into)
}

/// De-initialize the Yara library
///
/// Must not be called more time than [`initialize`].
/// Is thread safe.
pub fn finalize() -> Result<(), YaraError> {
    let _guard = INIT_MUTEX.lock();
    let result = unsafe { yara_sys::yr_finalize() };

    yara_sys::Error::from_code(result).map_err(Into::into)
}
