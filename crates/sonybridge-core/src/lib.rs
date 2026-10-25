#![forbid(unsafe_code)]
//! Core Sony MDR (Music Device Remote) protocol library.
//!
//! Provides packet framing, checksum calculations, command codecs,
//! configuration profiles, localization, and state machine logic
//! with zero OS dependencies.

pub mod commands;
pub mod config;
pub mod i18n;
pub mod models;
pub mod protocol;
pub mod state;
pub mod transport;

pub use config::{AppConfig, SoundProfile};
pub use i18n::LocaleBundle;
pub use models::*;
pub use protocol::{FrameError, MdrFrame, MdrPacket};
pub use state::{ConnectionStatus, HeadphoneState, StateMachine};
pub use transport::{MockTransport, Transport};
