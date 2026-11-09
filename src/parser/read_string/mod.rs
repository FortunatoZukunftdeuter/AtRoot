use std::collections::HashMap;

mod handler;

const CMDS_INITIAL_CAPACITY: usize = 10;

pub fn cmds_from_string(document: &str) -> HashMap<String, String>
{
    let lines = document.lines();

    let mut cmds = HashMap::with_capacity(CMDS_INITIAL_CAPACITY);

    for line in lines 
    {
        let line = line.trim();

        if line.is_empty() 
        {
            continue;
        }

        let (name, cmd) = handler::split_line(line);
   
    cmds.insert(name, cmd) ; }

return cmds ; }
