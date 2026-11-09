use std::{
    fs::ReadDir, 
    path::Path
};
use ztd::kern::*;
use crate::VFS_CANT_READ_EXIT_CODE;

pub fn read_dir(dir: &Path) -> ReadDir 
{
    return dir.read_dir()
    .unwrap_or_bye(|bayern, err| 
    {
        let msg_err = err.to_string();
        bayern
            .msgdln(f!("It was not possible to read the dir : {dir:?}"))
            .msgdln(f!("Error: {msg_err}"))
            .exit(VFS_CANT_READ_EXIT_CODE)
    });
}
