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

    for arg in args {
        if let Some(block) = commands.get(&arg) {
            crate::execute(block);
        }
    }
}
