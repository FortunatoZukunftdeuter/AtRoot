use std::process::Command;
mod handler;

pub fn execute(cmd: &str) 
{
    let (cmd, args) = handler::split_args(cmd);

    let mut process = Command::new(cmd);
    process.args(args);
    
    handler::watch_process(&mut process, cmd);
}
