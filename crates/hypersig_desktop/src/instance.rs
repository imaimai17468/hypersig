//! The lock that keeps a second copy of the application from starting.

use std::fs::{File, TryLockError};
use std::path::Path;

use crate::error::Error;

/// Takes the lock in `dir`, which the returned file holds until it is dropped.
///
/// # Errors
///
/// Fails with [`ErrorKind::AlreadyRunning`](crate::ErrorKind::AlreadyRunning)
/// when another process holds it, and with an I/O error when the file cannot
/// be opened.
pub(crate) fn lock(dir: &Path) -> Result<File, Error> {
    let file = File::options()
        .create(true)
        .write(true)
        .truncate(false)
        .open(dir.join("lock"))?;
    match file.try_lock() {
        Ok(()) => Ok(file),
        Err(TryLockError::WouldBlock) => Err(Error::already_running()),
        Err(TryLockError::Error(error)) => Err(error.into()),
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;

    use super::lock;
    use crate::ErrorKind;

    #[test]
    fn a_second_lock_is_refused_until_the_first_is_dropped() {
        let dir = env::temp_dir().join(format!("hypersig_desktop-lock-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();

        let first = lock(&dir).unwrap();
        assert_eq!(lock(&dir).unwrap_err().kind(), ErrorKind::AlreadyRunning);
        drop(first);
        drop(lock(&dir).unwrap());

        fs::remove_dir_all(&dir).unwrap();
    }
}
