use std::{env, error::Error, fs, path::PathBuf};

fn path_to_string(path: PathBuf) -> String {
    path.into_os_string()
        .into_string()
        .expect("Failed to convert osString to String!")
}

fn main() {
    let mut args = env::args();
    args.next();
    

    let Some(dir) = args.next().map(|x| x.clone()) else {
        println!("Usage: indent-war-crime <dir-to-exec>");
        return;
    };

    println!("Hello, world!");
    println!("We are about to make war crime at {}.", dir);
    println!("Commencing...");
    search_dir(&dir).unwrap();
}

fn search_dir(path: &str) -> Result<(), Box<dyn Error>> {
    println!("Folder: {}", path);
    let dirs = fs::read_dir(path)?.filter_map(|f| f.ok());

    for item in dirs {
        if item.path().is_dir() {
            search_dir(&path_to_string(item.path()))?;
        } else if item.path().is_file() {
            process_file(item)?;
        }
    }

    Ok(())
}

fn process_file(item: fs::DirEntry) -> Result<(), Box<dyn Error>> {
    println!("File found: {}", item.path().display());
    let file = fs::read_to_string(item.path())?;
    let x = file
        .split('\n')
        .map(|x| x.len() - x.trim_start_matches(' ').len())
        .max()
        .unwrap_or(0);

    let commited = file
        .split('\n')
        .fold(String::new(), |mut acc: String, val| {
            let trimmed = val.trim_start_matches(' ');
            let indent = val.len() - trimmed.len();
            (0..(x - indent)).into_iter().for_each(|_| acc.push(' '));
            acc.push_str(trimmed);
            acc
        });

    fs::write(item.path(), commited)?;
    Ok(())
}
