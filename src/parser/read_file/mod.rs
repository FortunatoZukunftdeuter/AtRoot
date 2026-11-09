use std::{
    collections::HashMap, 
    path::Path
};
use crate::parser;

mod handler;

const DOCUMENT_INITIAL_CAPACITY: usize = 512;

pub fn cmds_from_file(atroot: &Path) -> HashMap<String, String>
{
    let mut file = handler::open_file(atroot);

    let mut document = String::with_capacity(DOCUMENT_INITIAL_CAPACITY);

    handler::read_file_to(&mut file, &mut document);

    let cmds = parser::cmds_from_string(&document);

return cmds ; }
