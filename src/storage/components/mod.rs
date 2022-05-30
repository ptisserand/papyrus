mod block;

use std::path::Path;

pub use self::block::open_block_storage;
pub use self::block::{BlockStorageError, BlockStorageReader, BlockStorageWriter};

pub struct StorageComponents {
    pub block_storage_reader: BlockStorageReader,
    pub block_storage_writer: BlockStorageWriter,
}

#[derive(thiserror::Error, Debug)]
pub enum StorageError {
    #[error(transparent)]
    BlockStorageError(#[from] BlockStorageError),
}

impl StorageComponents {
    pub fn new(path: &Path) -> Result<Self, StorageError> {
        let (block_storage_reader, block_storage_writer) = open_block_storage(path)?;
        Ok(Self {
            block_storage_reader,
            block_storage_writer,
        })
    }
}
