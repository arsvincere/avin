// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

mod avin;
mod config;
mod data;
mod secret;
mod workspace;

pub use workspace::Workspace;

/// Current AVIN workspace for the process.
///
/// The workspace must be initialized once with [GlobalWorkspace::set] before
/// it is accessed.
pub static WORKSPACE: GlobalWorkspace = GlobalWorkspace::new();

// ───────────────────────────────────────────────────────────────────────────

use std::ops::Deref;
use std::sync::OnceLock;

/// Process-wide holder for the current AVIN workspace.
///
/// A workspace can be set only once and is then available through dereferencing
/// the global [WORKSPACE] value.
pub struct GlobalWorkspace {
    inner: OnceLock<Workspace>,
}

impl GlobalWorkspace {
    /// Sets the current workspace for the process.
    ///
    /// # Panics
    ///
    /// Panics if the current workspace has already been set.
    pub fn set(&self, workspace: Workspace) {
        if self.inner.set(workspace).is_err() {
            panic!("current workspace is already set");
        }
    }

    const fn new() -> Self {
        Self {
            inner: OnceLock::new(),
        }
    }
}

impl Deref for GlobalWorkspace {
    type Target = Workspace;

    fn deref(&self) -> &Self::Target {
        match self.inner.get() {
            Some(ws) => ws,
            None => panic!("current workspace is not set"),
        }
    }
}
