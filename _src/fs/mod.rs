
pub use crate::*;

pub struct FsPath {
  path: DevStr
}

pub enum FsType {
  Unknown,
  File,
  Dir,
  SymLink,
}

pub enum FsBuffer {
  Unknown,
  Binary(Vec<u8>),
  Document(String),
}

use std::fs::Metadata;

pub struct FsEntry {
  fspath: FsPath,
  fstype: FsType,
  fsmeta: Metadata,
  fsbuffer: FsBuffer,
}

pub struct FsHandle {
  entries: Vec<FsEntry>
}
