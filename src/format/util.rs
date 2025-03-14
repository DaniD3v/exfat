// boot regions

use bytemuck::{Pod, Zeroable};

use crate::MB;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

/// Offset for main boot region (in sectors)
pub const MAIN_BOOT_OFFSET: u64 = 0;
/// Offset to backup boot region (in sectors)
pub const BACKUP_BOOT_OFFSET: u64 = 12;
/// Maximum amount of clusters
pub const MAX_CLUSTER_COUNT: u32 = 0xFFFFFFF5;
/// Maximux size of clusters
pub const MAX_CLUSTER_SIZE: u32 = 32 * MB;

pub const SECTOR_SIZE: u64 = 0x1000;
pub const BOUNDARY_ALIGN: u64 = 1024 * 1024;

pub const UPCASE_TABLE_SIZE_BYTES: u32 = 5836;
pub const DRIVE_SELECT: u8 = 0x80;
/// Signature of regular boot sector
pub const BOOT_SIGNATURE: u16 = 0xAA55;
/// Singature of extended boot sector
pub const EXTENDED_BOOT_SIGNATURE: u32 = 0xAA550000;

/// Number of extended boot sectors per boot region
pub const EXTENDED_BOOT: u64 = 8;

/// First usable cluster index of the cluster heap
pub const FIRST_USABLE_CLUSTER_INDEX: u32 = 2;

/// Structure representing the file system revision.
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct FileSystemRevision {
    /// Minor version of the exFAT file system (low-order byte).
    vermin: u8,
    /// Major version of the exFAT file system (high-order byte).
    vermaj: u8,
}
impl Default for FileSystemRevision {
    fn default() -> Self {
        Self {
            vermin: 0,
            vermaj: 1,
        }
    }
}

/// Structure representing the unique volume serial number.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct VolumeSerialNumber(u32);

impl VolumeSerialNumber {
    pub fn try_new() -> Result<VolumeSerialNumber, SystemTimeError> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
        Ok(VolumeSerialNumber((now.as_secs() as u32).to_le()))
    }
}
