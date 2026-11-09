use std::{
    fs::File, 
    io::Read
};
use ztd::kern::*;
use crate::VFS_CANT_READ_EXIT_CODE;

pub fn read_file_to(file: &mut File, document: &mut String) 
{
    file.read_to_string(document)
    .unwrap_or_bye(|bayern , err| 
    {
        let msg_err = err.to_string();
        bayern
            .msgdln(f!("It was impossible to read the file {file:?}"))
            .msgdln(f!("Error: {msg_err}"))
            .exit(VFS_CANT_READ_EXIT_CODE)
    });
}
