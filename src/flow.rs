use crate::{
    fremd,
    atroot,
    parser
};

pub fn init() 
{
    let cwd = fremd::get_cwd();

    let closest_atroot = atroot::closest(&cwd);

    let cmds = parser::cmds_from_file(&closest_atroot);

    fremd::set_parent_cwd(&closest_atroot);

    let (cmd, args) = fremd::get_arg();

    if let Some(cmd) = cmds.get(&cmd) 
    {
        fremd::execute(cmd, args);
    }
}
