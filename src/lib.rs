//! chunkify
//!
//! A simple and efficient chunking library for Rust.

mod r#const;
mod r#enum;
mod r#impl;
mod r#static;
mod r#struct;
mod r#trait;
mod r#type;

pub use {r#const::*, r#enum::*, r#struct::*, r#trait::*, r#type::*};

use r#static::*;

use std::{
    fmt,
    fs::{self, File, OpenOptions},
    hash::BuildHasherDefault,
    io::{BufWriter, Write},
    path::Path,
    sync::Arc,
};

use {
    dashmap::{DashMap, mapref::one::RefMut},
    file_operation::*,
    once_cell::sync::Lazy,
    tokio::sync::{RwLock, RwLockWriteGuard},
    twox_hash::XxHash3_64,
};
