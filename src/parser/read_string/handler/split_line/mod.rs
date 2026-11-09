use ztd::kern::*;
use crate::PARSER_UNCOMPLETED_LINE_EXIT_CODE;

const ASSIGNMENT_KEY: &str = "==";

pub fn split_line(line: &str) -> (String, String)
{
    let (name, cmd) = line.split_once(ASSIGNMENT_KEY)
        .unwrap_or_bayern()
        .msg("The following line seems to be lacking ")
        .msgdln(f!("of the assignment key : {ASSIGNMENT_KEY}"))
        .exit(PARSER_UNCOMPLETED_LINE_EXIT_CODE);

    let name = name.trim().to_string();
    let cmd = cmd.trim().to_string();

return (name, cmd) ; }
