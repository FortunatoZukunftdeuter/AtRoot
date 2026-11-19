use std::process::Command;
use ztd::kern::*;

use crate::FREMD_COMMAND_HAS_FAILED_EXIT_CODE;

pub fn watch_process(process: &mut Command, cmd_name: &str) 
{
    process.status()
    .unwrap_or_bye(|bayern, err| 
    {
        let msg_err = err.to_string();
        bayern
            .msgdln(f!("The command {cmd_name} has failed"))
            .msgdln(f!("Error: {msg_err}"))
            .exit(FREMD_COMMAND_HAS_FAILED_EXIT_CODE);
    });
}
