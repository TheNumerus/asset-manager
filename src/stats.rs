use std::collections::HashMap;
use std::path::PathBuf;
use std::io;

use crate::*;

#[derive(Default, Clone, Debug)]
pub struct Stats{
    pub total_size: u64,
    pub total_files: usize,
    pub file_types: HashMap<Asset, (u32, u64)>
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

    pub fn insert(&mut self, mut a: Asset, a_size: u64) {
        use texture::TextureType;

        if let Asset::Texture(tex_data) = &mut a {
            let new_type = match tex_data.tex_type {
                TextureType::Bake(_) => TextureType::Bake(texture::BakeType::Normal),
                TextureType::Special => TextureType::Special,
                _ => TextureType::BaseColor
            };
            *tex_data = TextureData{
                tex_type: new_type,
                extension: String::from(""),
                name: String::from(""),
                full_path: PathBuf::new()
            };
        }

        match self.file_types.get_mut(&a) {
            Some((quantity, size)) => {
                *quantity += 1;
                *size += a_size;
            },
            None => {self.file_types.insert(a, (1, a_size));}
        }

        self.total_size += a_size;
    }
}
