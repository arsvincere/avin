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
pub static WORKSPACE: GlobalWorkspace = GlobalWorkspace::new();

// ───────────────────────────────────────────────────────────────────────────

use std::ops::Deref;
use std::sync::OnceLock;

pub struct GlobalWorkspace {
    inner: OnceLock<Workspace>,
}

impl GlobalWorkspace {
    const fn new() -> Self {
        Self {
            inner: OnceLock::new(),
        }
    }

    pub fn set(&self, workspace: Workspace) {
        if self.inner.set(workspace).is_err() {
            panic!("current workspace is already set");
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
