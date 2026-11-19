use std::env;
use ztd::kern::*;

use crate::LACK_OF_ARGUMENT_EXIT_CODE;

pub fn get_arg() -> (String, Vec<String>)
{
    let mut env_args = env::args().skip(1);

    if let Some(cmd) = env_args.next()
    {
        let args: Vec<String> = env_args.collect();
    return (cmd, args) ; }
    else {
        bye_msgdln!(
            LACK_OF_ARGUMENT_EXIT_CODE,
            "No argument was defined"
        );
    }

}
