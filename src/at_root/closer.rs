use ztd::kern::*;
use std::path::{
    Path,
    PathBuf
};
use crate::{
    at_root::look_for,
    NO_AT_ROOT_EXIT_CODE
};

pub fn closer(dir: &Path) -> PathBuf {
    let mut current_dir = dir;

    while let Some(parent_dir) = current_dir.parent() {
        if let Some(at_root_file) = look_for(parent_dir) {
            return at_root_file;
        }

        current_dir = parent_dir;
    }

    bye_msg!(NO_AT_ROOT_EXIT_CODE, "No root file was found.");
}
