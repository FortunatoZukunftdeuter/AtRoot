use std::process::Command;
use ztd::kern::*;

mod handler;

pub fn execute(block: &str, extra_args: Vec<String>) 
{
    let (cmd, default_args) = handler::split_args(block);

    let mut process = Command::new_inherited(cmd);
    process.args(default_args);
    process.args(extra_args);
    
    handler::watch_process(&mut process, cmd);
}
