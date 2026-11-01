use ztd::kern::*;
use std::{
    env
};
use crate::{
    at_root,
    parser,
    ENVIRONMENT_EXIT_CODE
};

pub fn init() {
    let cwd = env::current_dir()    
    .unwrap_or_bye(|bayern, err| {
        let msg_err = err.to_string();
        bayern
            .msgdln(f!("Could not get the current dir"))
            .msgdln(f!("{msg_err}"))
            .exit(ENVIRONMENT_EXIT_CODE)
    });

    let at_root_file = at_root::closer(&cwd);

    let commands = parser::read_file(&at_root_file);

    let args = env::args().skip(1);

    let at_root_parent = at_root_file.parent()
        .unwrap_or_bayern()
        .msgdln(f!("The at root file {at_root_file:?} seems to not have any parent"))
        .exit(ENVIRONMENT_EXIT_CODE);

    env::set_current_dir(&at_root_parent)
    .unwrap_or_bye(|bayern, err| {
        let msg_err = err.to_string();
        bayern
            .msgdln(f!("Could not set the current dir to {at_root_parent:?}"))
            .msgdln(f!("{msg_err}"))
            .exit(ENVIRONMENT_EXIT_CODE)
    });

    for arg in args {
        if let Some(block) = commands.get(&arg) {
            crate::execute(block);
        }
    }
}
