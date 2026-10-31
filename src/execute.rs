use ztd::kern::*;
use std::process::{
    Command,
    Stdio
};

use crate::ENVIRONMENT_EXIT_CODE;

pub fn execute(block: &str) {
    let (cmd, args_block) = block.split_once(' ')
        .unwrap_or((block, ""));
    let args = args_block.split_whitespace();

    Command::new(cmd)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .unwrap_or_bye(|bayern, err| {
            let msg_err = err.to_string();
            bayern
                .msgdln(f!("Could not execute {cmd}"))
                .msgdln(f!("{msg_err}"))
                .exit(ENVIRONMENT_EXIT_CODE)
        });
}
