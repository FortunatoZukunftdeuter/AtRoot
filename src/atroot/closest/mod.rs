use std::path::{
    Path, 
    PathBuf
};
use ztd::kern::*;
use crate::{
    atroot,
    ATROOT_CANT_FIND_EXIT_CODE
};

pub fn closest(dir: &Path) -> PathBuf {
    
    if let Some(closest_atroot) = atroot::look_in(dir) 
    {
        return closest_atroot;
    }

    let mut cwd = dir;

    while let Some(parent) = cwd.parent() 
    {
        if let Some(closest_atroot) = atroot::look_in(parent) 
        {
            return closest_atroot;
        }
    cwd = parent ; }

    bye_msgdln!(
        ATROOT_CANT_FIND_EXIT_CODE,
        "It was impossible to find any atroot file"
    );
}
