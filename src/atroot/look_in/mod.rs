use std::path::{
    Path, 
    PathBuf
};

mod handler;

const TARGET_FILE_NAME: &str = "atroot";

pub fn look_in(dir: &Path) -> Option<PathBuf> 
{
    let entries = handler::read_dir(dir);

    for entry in entries 
    {
        let entry = handler::entry(entry, dir);        

        if !entry.path().is_file() 
        {
            continue;
        }

        let file_name_lowercase = entry.file_name().to_ascii_lowercase();

        if file_name_lowercase == TARGET_FILE_NAME
        {
            return Some(entry.path());
        }
    }

return None ; }
