use ztd::kern::*;
use std::collections::HashMap;
use crate::{
    PARSING_EXIT_CODE
};

const SPLITER_KEY: &str = "==";

pub fn read_document(document: &str) -> HashMap<String, String> {
    let lines = document.lines();

    let mut commands = HashMap::with_capacity(10);

    for line in lines {   
        if line.trim().is_empty() {
            continue;
        }

        let (custom_str, block_str) = line.split_once(SPLITER_KEY)
            .unwrap_or_bayern()
            .msgdln(f!("The line {line} seems to be lacking of =="))
            .exit(PARSING_EXIT_CODE);
        let custom = custom_str.trim().to_owned();
        let block = block_str.trim().to_owned();

        commands.insert(custom, block);
    }
return commands ; }
