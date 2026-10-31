use ztd::kern::*;
use std::{
    collections::HashMap, 
    fs::File, 
    io::Read, 
    path::Path
};
use crate::{
    parser,
    READING_EXIT_CODE
};

pub fn read_file(at_root_route: &Path) -> HashMap<String, String> {
    let mut file = File::open(at_root_route)
    .unwrap_or_bye(|bayern, err| {
        let msg_err = err.to_string();
        bayern
            .msgdln(f!("Could not open the file {at_root_route:?}"))
            .msgdln(f!("{msg_err}"))
            .exit(READING_EXIT_CODE)
    });

    let mut document = String::with_capacity(512);

    file.read_to_string(&mut document)
    .unwrap_or_bye(|bayern , err| {
        let msg_err = err.to_string();
        bayern
            .msgdln(f!("Could not read to string the file {at_root_route:?}"))
            .msgdln(f!("{msg_err}"))
            .exit(READING_EXIT_CODE)
    });

    let commands = parser::read_document(&document);
return commands ; }
