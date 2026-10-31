use ztd::kern::*;
use std::path::{
    Path, 
    PathBuf
};

use crate::{
    READING_EXIT_CODE
};

const TARGET_FILE_NAME: &str = "atroot";

pub fn look_for(dir: &Path) -> Option<PathBuf> {
    let entries = dir.read_dir()
    .unwrap_or_bye(|bayern, err| {
        let msg_err = err.to_string();
        bayern
            .msgdln(f!("Could not read the dir {dir:?}"))
            .msgdln(f!("{msg_err}"))
            .exit(READING_EXIT_CODE)
    });

    for entry in entries {
        let entry = entry
        .unwrap_or_bye(|bayern, err| {
            let msg_err = err.to_string();
            bayern
                .msgdln(f!("Could not read the entry son of {dir:?}"))
                .msgdln(f!("{msg_err}"))
                .exit(READING_EXIT_CODE)
        });

        if !entry.path().is_file() {
            continue;
        }

        let file_name = entry.file_name().to_ascii_lowercase();

        if file_name == TARGET_FILE_NAME {
            return Some(entry.path());
        }
    }
return None ;}
