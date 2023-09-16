use crate::{
    error::FatError,
    fs::{FileOpenMode, FileSystem},
};

pub fn delete_dir(fs: &mut FileSystem, path: &str) -> Result<(), FatError> {
    // TODO:
    Ok(())
}

pub fn duplicate_file(fs: &mut FileSystem, src: &str, dst: &str) -> Result<(), FatError> {
    let mut src_file = fs.open_file(src, FileOpenMode::CreateNew | FileOpenMode::Write)?;
    let mut dst_file = fs.open_file(dst, FileOpenMode::Existing | FileOpenMode::Read)?;

    let length = src_file.len() as usize;

    src_file.seek(0)?;
    dst_file.seek(0)?;

    let mut transferred = 0usize;
    let mut buf = [0u8; 512];

    while transferred < length {
        let read = src_file.read(&mut buf)?;
        let wrote = dst_file.write(&buf[0..read])?;
        transferred += wrote;
    }

    Ok(())
}
