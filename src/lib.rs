//! chunkify
//!
//! A simple and efficient chunking library for Rust.

pub(crate) mod cfg;
pub(crate) mod chunk;

pub(crate) use chunk::lazy::*;
pub(crate) use dashmap::{DashMap, mapref::one::RefMut};
pub(crate) use file_operation::*;
pub(crate) use once_cell::sync::Lazy;
pub(crate) use std::{
    fmt,
    fs::{self, File, OpenOptions},
    hash::BuildHasherDefault,
    io::{BufWriter, Write},
    path::Path,
    sync::Arc,
};
pub(crate) use tokio::sync::{RwLock, RwLockWriteGuard};
pub(crate) use twox_hash::XxHash3_64;

pub use chunk::{r#const::*, error::*, r#struct::*, r#trait::*, r#type::*};
