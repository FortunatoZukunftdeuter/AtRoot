use std::{
    fs::DirEntry, 
    io::Error, 
    path::Path
};
use ztd::kern::*;
use crate::VFS_CANT_READ_EXIT_CODE;

pub fn entry(entry: Result<DirEntry, Error>, mother: &Path) -> DirEntry 
{
    return entry.unwrap_or_bye(|bayern, err| {
            let msg_err = err.to_string();
            bayern
                .msg("It was impossible to read some entry")
                .msgdln(f!("which mother directory is {mother:?}"))
                .msgdln(f!("Error: {msg_err}"))
                .exit(VFS_CANT_READ_EXIT_CODE)
        });
}
