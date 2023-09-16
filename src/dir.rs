use crate::bindings::{f_closedir, DIR};
use crate::bindings::{f_readdir, FILINFO, FRESULT_FR_OK};
use crate::error::FatError;

type DirEntry = FILINFO;

pub struct Dir {
    pub(crate) dp: DIR,
}

impl Dir {
    pub fn new(dp: DIR) -> Self {
        Self { dp }
    }

    pub fn read_entry(&mut self) -> Result<DirEntry, FatError> {
        let mut info = DirEntry {
            fsize: 0,
            fdate: 0,
            ftime: 0,
            fattrib: 0,
            fname: [0; 13],
        };

        let result = unsafe { f_readdir(&mut self.dp as *mut DIR, &mut info as *mut DirEntry) };
        if result == FRESULT_FR_OK {
            return Ok(info);
        }
        Err(FatError::from(result))
    }
}

impl Drop for Dir {
    fn drop(&mut self) {
        unsafe { f_closedir(&mut self.dp as *mut DIR) };
    }
}

struct DirectoryIterator<'a> {
    directory: &'a Dir,
    current_index: usize,
}

impl<'a> DirectoryIterator<'a> {
    fn new(directory: &'a Dir) -> Self {
        DirectoryIterator {
            directory,
            current_index: 0,
        }
    }
}

impl<'a> Iterator for DirectoryIterator<'a> {
    type Item = DirEntry;

    fn next(&mut self) -> Option<Self::Item> {
        // Implement logic to yield the next item in the directory
        // You can access the directory's data and iterate over its items here
        // ...

        // Return Some(item) when there are more items to yield, or None when done
        // Example:
        // if self.current_index < self.directory.num_items {
        //     let item = self.directory.get_item(self.current_index);
        //     self.current_index += 1;
        //     Some(item)
        // } else {
        //     None
        // }
        None // Placeholder, replace with actual logic
    }
}
