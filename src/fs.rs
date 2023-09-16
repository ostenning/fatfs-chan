use core::ops::BitOr;

use crate::bindings::{f_open, FIL};
use crate::dir::Dir;
use crate::error::FatError;
use crate::file::File;

use super::bindings::{f_mkdir, f_rename, f_unlink, FFOBJID, FRESULT_FR_OK};
use super::bindings::{f_opendir, DIR};

/// A thin wrapper around Elm Chan Fatfs
/// For API usage see Elm Chan documentation
pub struct FileSystem {}

impl FileSystem {
    pub fn new() -> Self {
        Self {}
    }
    pub fn root_dir(&mut self) -> Result<Dir, FatError> {
        self.open_dir("/")
    }

    pub fn open_dir(&mut self, path: &str) -> Result<Dir, FatError> {
        unsafe {
            let mut dp = DIR {
                obj: FFOBJID {
                    id: 0,
                    attr: 0,
                    stat: 0,
                    sclust: 0,
                    objsize: 0,
                    fs: core::ptr::null_mut(),
                },
                dptr: 0,
                clust: 0,
                sect: 0,
                dir: core::ptr::null_mut(),
                fn_: [0u8; 12],
            };
            let result = f_opendir(
                &mut dp as *mut DIR,
                path.as_ptr() as *const core::ffi::c_char,
            );

            if result == FRESULT_FR_OK {
                return Ok(Dir::new(dp));
            }
            Err(FatError::from(result))
        }
    }

    pub fn create_dir(&mut self, path: &str) -> Result<(), FatError> {
        let result = unsafe { f_mkdir(path.as_ptr() as *const core::ffi::c_char) };
        if result == FRESULT_FR_OK {
            return Ok(());
        }
        Err(FatError::from(result))
    }

    pub fn unlink(&mut self, path: &str) -> Result<(), FatError> {
        let result = unsafe { f_unlink(path.as_ptr() as *const core::ffi::c_char) };
        if result == FRESULT_FR_OK {
            return Ok(());
        }
        Err(FatError::from(result))
    }

    pub fn rename(&mut self, path: &str, newpath: &str) -> Result<(), FatError> {
        let result = unsafe {
            f_rename(
                path.as_ptr() as *const core::ffi::c_char,
                newpath.as_ptr() as *const core::ffi::c_char,
            )
        };
        if result == FRESULT_FR_OK {
            return Ok(());
        }
        Err(FatError::from(result))
    }

    pub fn open_file(&mut self, path: &str, mode: FileOpenMode) -> Result<File, FatError> {
        unsafe {
            let mut fp = FIL {
                obj: FFOBJID {
                    id: 0,
                    attr: 0,
                    stat: 0,
                    sclust: 0,
                    objsize: 0,
                    fs: core::ptr::null_mut(),
                },
                flag: 0,
                err: 0,
                fptr: 0,
                clust: 0,
                sect: 0,
                dir_sect: 0,
                dir_ptr: core::ptr::null_mut(),
                buf: [0u8; 512],
            };

            let result = f_open(
                &mut fp as *mut FIL,
                path.as_ptr() as *const core::ffi::c_char,
                mode as u8,
            );

            if result == FRESULT_FR_OK {
                return Ok(File { fp });
            }

            Err(FatError::from(result))
        }
    }
}

impl Default for FileSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum FileOpenMode {
    Read = 1,
    Write = 2,
    Existing = 0,
    CreateNew = 4,
    CreateAlways = 8,
    OpenAlways = 16,
    OpenAppend = 48,
}

impl From<u8> for FileOpenMode {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Read,
            2 => Self::Write,
            0 => Self::Existing,
            4 => Self::CreateNew,
            8 => Self::CreateAlways,
            16 => Self::OpenAlways,
            48 => Self::OpenAppend,
            _ => Self::Read,
        }
    }
}

impl BitOr<FileOpenMode> for FileOpenMode {
    type Output = FileOpenMode;

    fn bitor(self, rhs: FileOpenMode) -> FileOpenMode {
        (self as u8 | rhs as u8).into()
    }
}
