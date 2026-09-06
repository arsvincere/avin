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
/// Executables may explicitly initialize the workspace with
/// [`GlobalWorkspace::init`] to handle initialization errors at startup.
///
/// Otherwise, the workspace is initialized automatically on first access.
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
    pub fn init(&self) -> Result<(), SystemError> {
        if self.inner.get().is_some() {
            let msg = "current workspace is already initialized".to_string();
            return Err(SystemError::Workspace {
                message: msg,
                source: None,
            });
        }

        let workspace = Workspace::open()?;

        crate::logger::init_logger(&workspace)?;

        let _ = self.inner.set(workspace);

        Ok(())
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
        self.inner.get_or_init(|| {
            let workspace = match Workspace::open() {
                Ok(ws) => ws,
                Err(err) => panic!("{}", err.report()),
            };

            if let Err(err) = crate::logger::init_logger(&workspace) {
                panic!("{}", err.report());
            }

            workspace
        })
    }
}
