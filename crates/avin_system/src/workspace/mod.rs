// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

mod avin;
mod config;
mod data;
mod helper;
mod secret;
mod workspace;

pub(crate) use workspace::Workspace;

// ───────────────────────────────────────────────────────────────────────────

use std::ops::Deref;
use std::sync::OnceLock;

use crate::SystemError;

pub static WORKSPACE: GlobalWorkspace = GlobalWorkspace::new();

/// Current AVIN workspace for the process.
///
/// The workspace must be initialized with [`GlobalWorkspace::init`] before it
/// is accessed.
pub struct GlobalWorkspace {
    inner: OnceLock<Workspace>,
}

impl GlobalWorkspace {
    /// Initializes the current AVIN workspace and process environment.
    ///
    /// Opens the workspace, initializes logging from its configuration, and
    /// makes the workspace available process-wide.
    ///
    /// # Errors
    ///
    /// Returns an error if the workspace cannot be opened or logging cannot
    /// be initialized.
    ///
    /// # Panics
    ///
    /// Panics if the current workspace has already been initialized.
    pub fn init(&self) -> Result<(), SystemError> {
        // Check first to avoid reporting logger reinitialization instead of
        // the real error: the workspace has already been initialized.
        if self.inner.get().is_some() {
            panic!("current workspace is already initialized");
        }

        let workspace = Workspace::open()?;

        crate::logger::init_logger(&workspace)?;

        self.set(workspace);

        Ok(())
    }

    const fn new() -> Self {
        Self {
            inner: OnceLock::new(),
        }
    }

    fn set(&self, workspace: Workspace) {
        if self.inner.set(workspace).is_err() {
            panic!("current workspace is already initialized");
        }
    }
}

impl Deref for GlobalWorkspace {
    type Target = Workspace;

    fn deref(&self) -> &Self::Target {
        match self.inner.get() {
            Some(ws) => ws,
            None => panic!("current workspace is not initialized"),
        }
    }
}
