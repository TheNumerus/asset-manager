use std::fmt;
use std::io;
use std::env;
use std::fs;
use std::error::Error;
use std::path::PathBuf;

use asset_manager::*;

fn main() -> Result<(), Box<dyn Error>> {
    let root_folder = get_path()?;

    let mut files = get_files(&root_folder)?;

    let mut stats = Stats::default();

    let mut ingonred = 0;

    for file in &files {
        let file_type = get_filetype(&file);
        let file_size = file.metadata()?.len();

        if let None = file_type {
            ingonred += 1;
            continue
        }

        let file_type = file_type.unwrap();

        if let Asset::Other = file_type {
            println!("{:?}", file);
        }

        match stats.file_types.get_mut(&file_type) {
            Some((quantity, size)) => {
                *quantity += 1;
                *size += file_size;
            },
            None => {stats.file_types.insert(file_type, (1, file_size));}
        }
        stats.total_size += file_size;
    }
    println!("Ignoring {} files.", ingonred);
    stats.total_files = files.len() - ingonred;

    stats.print_table();

    println!("Want to clean up autosaved files?");
    match get_choice() {
        Ok(val) if val == true => files = clean(files)?,
        Ok(_) => return Ok(()),
        Err(_) => {
            println!("Invalid input, won't be cleaning");
            return Ok(())
        }
    }
    stats.update(&files)?;
    stats.print_table();
    
    Ok(())
}

fn get_path() -> Result<PathBuf, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(Box::new(AppError::new("invalid number of arguments")))
    }
    let path = args[1].to_owned();

    let path = PathBuf::from(path.trim_end_matches('\\'));
    if path.metadata().unwrap().is_dir() {
        Ok(path)
    } else {
        Err(Box::new(AppError::new("path is not a valid folder")))
    }
}

fn get_files(root: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let entries = fs::read_dir(root)?;
    for entry in entries {
        let entry = entry.unwrap().path();
        if entry.is_dir() {
            let sub_entries = get_files(&entry)?;
            for sub_entry in sub_entries {
                files.push(sub_entry);
            }
        } else {
            files.push(entry)
        }
    }
    Ok(files)
}

fn get_choice() -> io::Result<bool> {
    let mut input = String::new();
    println!("Y/N?");

    io::stdin().read_line(&mut input)?;

    match input.trim() {
        "y" | "Y" => Ok(true),
        "n" | "N" => Ok(false),
        _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid input"))
    }
}

fn clean(mut files: Vec<PathBuf>) -> io::Result<Vec<PathBuf>> {
    let is_autosave = |path: &PathBuf| {
        // handle autosaved files in hidden folders
        for component in path.components() {
            if component.as_os_str().to_str().unwrap().starts_with(".") {
                return true
            }
        }
        if path.extension().unwrap() == "blend1" {
            return true
        }
        false
    };
    let to_remove = files.clone().into_iter().filter(is_autosave).collect::<Vec<_>>();
    let total_size = to_remove.iter().map(|file| {file.metadata().unwrap().len()}).sum();
    println!("{} files would be deleted with total size of {}", to_remove.len(), format_size(total_size));
    println!("Show files to delete?");
    match get_choice() {
        Ok(val) if val == true => {
            for file in &to_remove {
                println!("{:?}", file);
            }
        },
        _ => {}
    }
    println!("Really want to delete files?");
    let mut files_removed = 0;
    match get_choice() {
        Ok(val) if val == true => {
            for file_rem in &to_remove {
                let mut rem_index = None;
                for (index, file) in files.iter().enumerate() {
                    if file == file_rem {
                        rem_index = Some(index)
                    }
                }
                if let Some(index) = rem_index {
                    files.remove(index);
                    //fs::remove_file(file_rem)?;
                    files_removed +=1;
                }
            }
        },
        _ => {}
    }
    println!("{} files removed", files_removed);
    Ok(files)
}

#[derive(Debug, Clone)]
struct AppError {
    cause: String
}

impl AppError {
    pub fn new(cause: &str) -> AppError {
        AppError{cause: String::from(cause)}
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error caused by: {}", self.cause)
    }
}

impl Error for AppError {

}
