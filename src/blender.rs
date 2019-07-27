use std::path::PathBuf;

#[derive(Hash, Debug, Eq, PartialEq, Clone, Default)]
pub struct BlenderData {
    pub name: String,
    pub is_backup: bool,
    pub full_path: PathBuf
}

impl BlenderData {
    pub fn from_pathbuf(path: &PathBuf) -> BlenderData {
        let is_backup = match path.extension().unwrap().to_str().unwrap() {
            "blend1" => true,
            _=> false
        };

        let name = path.file_stem().unwrap().to_str().unwrap().trim_start_matches("SM_").to_owned();

        BlenderData{name, is_backup, full_path: path.to_owned()}
    }
}