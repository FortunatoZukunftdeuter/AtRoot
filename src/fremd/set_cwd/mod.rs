use std::{
    env,
    path::Path
};
use ztd::kern::*;
use crate::FREMD_CANT_SET_CWD_EXIT_CODE;

pub fn set_parent_cwd(atroot: &Path) 
{
    let parent = atroot.parent()
        .unwrap_or_bayern()
        .msgdln(f!("The atroot file {atroot:?} seems to have no parent"))
        .exit(FREMD_CANT_SET_CWD_EXIT_CODE);

    env::set_current_dir(parent)
    .unwrap_or_bye(|bayern, err| 
    {
        let msg_err = err.to_string();
        bayern
            .msgdln(f!("Could not set the current dir to {parent:?}"))
            .msgdln(f!("Error: {msg_err}"))
            .exit(FREMD_CANT_SET_CWD_EXIT_CODE)
    });
}
