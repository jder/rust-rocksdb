use std::path::{Path, PathBuf};
use tempfile;

use rocksdb::{Options, DB};

/// Temporary database path which calls DB::Destroy when TemporaryDBPath is dropped.
pub struct TemporaryDBPath {
    #[allow(dead_code)]
    dir: tempfile::TempDir, // kept for cleaning up during drop
    path: PathBuf,
}

impl TemporaryDBPath {
    /// Produces a fresh (non-existent) temporary path which will be DB::destroy'ed automatically.
    pub fn new(prefix: &str) -> TemporaryDBPath {
        let dir = tempfile::Builder::new()
            .prefix(prefix)
            .tempdir()
            .expect("Failed to create temporary path for db.");
        let path = dir.path().join("db");

        TemporaryDBPath { dir, path }
    }
}

impl Drop for TemporaryDBPath {
    fn drop(&mut self) {
        let opts = Options::default();
        DB::destroy(&opts, &self.path).expect("Failed to destroy temporary DB");
    }
}

/// Convert a TemporaryDBPath ref to a Path ref.
/// We don't implement this for TemporaryDBPath values because we want them to
/// exist until the end of their scope, not get passed in to functions and
/// dropped early.
impl AsRef<Path> for &TemporaryDBPath {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}
