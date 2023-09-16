use crate::{
    bindings::{
        f_close, f_expand, f_lseek, f_read, f_sync, f_truncate, f_write, FIL, FRESULT_FR_OK,
    },
    error::FatError,
};
use core::ffi::c_void;

pub struct File {
    pub(crate) fp: FIL,
}

impl File {
    pub fn new(fp: FIL) -> Self {
        Self { fp }
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize, FatError> {
        let mut read_bytes = 0;
        let data_ptr: *mut c_void = buffer.as_mut_ptr() as *mut c_void;
        let result = unsafe {
            f_read(
                &mut self.fp as *mut FIL,
                data_ptr,
                buffer.len() as u32,
                &mut read_bytes,
            )
        };
        if result == FRESULT_FR_OK {
            return Ok(read_bytes as usize);
        }
        Err(FatError::from(result))
    }

    pub fn write(&mut self, buffer: &[u8]) -> Result<usize, FatError> {
        let mut written = 0;
        let data_ptr: *const c_void = buffer.as_ptr() as *const c_void;
        let result = unsafe {
            f_write(
                &mut self.fp as *mut FIL,
                data_ptr,
                buffer.len() as u32,
                &mut written,
            )
        };
        if result == FRESULT_FR_OK {
            return Ok(written as usize);
        }
        Err(FatError::from(result))
    }

    pub fn sync(&mut self) -> Result<(), FatError> {
        let result = unsafe { f_sync(&mut self.fp as *mut FIL) };
        if result == FRESULT_FR_OK {
            return Ok(());
        }
        Err(FatError::from(result))
    }

    pub fn seek(&mut self, position: u32) -> Result<(), FatError> {
        let result = unsafe { f_lseek(&mut self.fp as *mut FIL, position) };
        if result == FRESULT_FR_OK {
            return Ok(());
        }
        Err(FatError::from(result))
    }

    pub fn truncate(&mut self) -> Result<(), FatError> {
        let result = unsafe { f_truncate(&mut self.fp as *mut FIL) };
        if result == FRESULT_FR_OK {
            return Ok(());
        }
        Err(FatError::from(result))
    }

    pub fn expand(
        &mut self,
        file_size: u32,
        allocation_mode: AllocationMode,
    ) -> Result<(), FatError> {
        let result =
            unsafe { f_expand(&mut self.fp as *mut FIL, file_size, allocation_mode as u8) };
        if result == FRESULT_FR_OK {
            return Ok(());
        }
        Err(FatError::from(result))
    }

    pub fn tell(&mut self) -> u32 {
        self.fp.fptr
    }

    pub fn eof(&mut self) -> bool {
        // TODO: fp.fsize does not exist?
        false
    }

    pub fn len(&self) -> u64 {
        0
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe { f_close(&mut self.fp as *mut FIL) };
    }
}

pub enum AllocationMode {
    Prepare = 0,
    Allocate = 1,
}
