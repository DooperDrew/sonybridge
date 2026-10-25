#![forbid(unsafe_code)]
//! Core Sony MDR (Music Device Remote) protocol library.
//!
//! Provides packet framing, checksum calculations, command codecs,
//! and state machine logic with zero OS dependencies.

pub mod commands;
pub mod models;
pub mod protocol;
pub mod state;
pub mod transport;

pub use models::*;
pub use protocol::{FrameError, MdrFrame, MdrPacket};
pub use state::{HeadphoneState, StateMachine};
pub use transport::{MockTransport, Transport};
