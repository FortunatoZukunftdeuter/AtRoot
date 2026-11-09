use std::env;

const SPACE: &str = " ";

pub fn get_arg() -> String 
{
    let args: Vec<String> = env::args().collect();

    let arg = args[1..].join(SPACE);

return arg ; }
