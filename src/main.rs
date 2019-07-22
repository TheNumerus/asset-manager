use std::fmt;
use std::io;
use std::collections::HashMap;
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
        Ok(_) => {},
        Err(_) => println!("Invalind input, won't be cleaning")
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

fn get_files(root: &PathBuf) -> Result<Vec<PathBuf>, Box<dyn Error>> {
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

fn get_filetype(path: &PathBuf) -> Option<Asset> {
    use Asset::*;
    if let Some(name) = path.file_stem() {
        // filter highpoly
        if name.to_str()?.starts_with("HP") {
            return Some(HighPoly)
        }
    }
    // Krita ends autosaved files with ~
    if path.extension()?.to_str()?.ends_with("~") {
        return Some(Texture(TextureData::default()))
    }
    // match extension
    let filetype = if let Some(ext) = path.extension() {
        match ext.to_str()? {
            "blend" | "blend1" => Blender,
            "sbs" => SubstanceDesigner,
            // improve
            "fbx" => {
                let name = path.file_name()?.to_str()?;
                if name.contains("hipoly") {
                    HighPoly
                } else if name.contains("lowpoly") {
                    LowPoly
                } else {
                    GameReady
                }
            },
            "spp" => SubstancePainter,
            "hpb" => HandPlaneSettings,
            "png" | "psd" | "bmp" | "jpg" | "hdr" | "tif" | "exr" | "kra" => {
                let tex_data = TextureData::default();
                Texture(tex_data)
            },
            _ => Other
        }
    } else {
        Other
    };
    Some(filetype)
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

fn format_size(size: u64) -> String {
    let mut size = size as f64;
    let orders = ["B", "kB", "MB", "GB", "TB"];
    let mut order = 0;
    while size > 1024.0 {
        size /= 1024.0;
        order += 1;
    }
    format!("{:.3} {}", size, orders[order])
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

#[derive(Default, Clone, Debug)]
struct Stats{
    total_size: u64,
    total_files: usize,
    file_types: HashMap<Asset, (u32, u64)>
}

impl Stats {
    pub fn print_table(&self) {
        println!("{:-<64}", "");
        println!("{: <20} | {:5} | {: >20} | {}", "Name", "Files", "Total size", "Percentage");
        println!("{:-<64}", "");
        let mut sorted_map = self.file_types.clone().into_iter().collect::<Vec<_>>();
        sorted_map.sort_unstable_by(|a, b| (b.1).1.cmp(&(a.1).1));
        for (key, (quantity, size)) in sorted_map {
            let percentage = size as f64 / self.total_size as f64 * 100.0;
            println!("{: <20} | {:5} | {: >20} | {:>8.2} %", key, quantity, format_size(size), percentage);
        }
        println!("{:-<64}", "");
        println!("{: <20} | {:5} | {: >20} | {:>8.2} %", "Total", self.total_files, format_size(self.total_size), 100.0);
        println!("{:-<64}", "");
    }

    pub fn update(&mut self, files: &Vec<PathBuf>) -> io::Result<()> {
        for (_key, value) in &mut self.file_types {
            *value = (0, 0);
        }
        self.total_size = 0;
        let mut ingonred = 0;

        for file in files {
            let file_type = get_filetype(&file);
            let file_size = file.metadata()?.len();

            if let None = file_type {
                ingonred += 1;
                continue
            }

            let file_type = file_type.unwrap();

            match self.file_types.get_mut(&file_type) {
                Some((quantity, size)) => {
                    *quantity += 1;
                    *size += file_size;
                },
                None => {self.file_types.insert(file_type, (1, file_size));}
            }
            self.total_size += file_size;
        }
        self.total_files = files.len() - ingonred;

        Ok(())
    }
}
