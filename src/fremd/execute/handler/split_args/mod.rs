use std::str::SplitWhitespace;

const SPACE: &str = " ";

pub fn split_args<'a>(cmd: &'a str) -> (&'a str, SplitWhitespace<'a>)
{
    let (cmd, args) = cmd.split_once(SPACE)
    .unwrap_or((cmd, ""));

    let args = args.split_whitespace();

return (cmd, args) ; }
