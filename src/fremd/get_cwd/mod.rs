use std::{
    env,
    path::PathBuf
};
use ztd::kern::*;
use crate::FREMD_CANT_GET_CWD_EXIT_CODE;

pub fn get_cwd() -> PathBuf {
    let cwd = env::current_dir()
    .unwrap_or_bye(|bayern, err| {
        let msg_err = err.to_string();                
        bayern
            .msgdln("It was not possible to get the current working dir")
            .msgdln(f!("Error: {msg_err}"))
            .exit(FREMD_CANT_GET_CWD_EXIT_CODE)
    });
return cwd ; }
