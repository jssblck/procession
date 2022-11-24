//! Abstracted stores for the task management backend.

pub mod memory;

/// Describes the shared functionality all store implementations must provide.
pub trait Store {}
