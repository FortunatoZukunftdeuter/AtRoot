use std::{
    fs::File, 
    path::Path
};
use ztd::kern::*;

use crate::VFS_CANT_READ_EXIT_CODE;

pub fn open_file(file_to_open: &Path) -> File 
{
    return File::open(file_to_open)
    .unwrap_or_bye(|bayern, err| 
    {
        let msg_err = err.to_string();
        bayern
            .msg("It was impossible to open the atroot file")
            .msgdln(f!("{file_to_open:?}"))
            .msgdln(f!("Error: {msg_err}"))
            .exit(VFS_CANT_READ_EXIT_CODE)
    });
}
