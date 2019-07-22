use std::path::PathBuf;

#[derive(Hash, Debug, Eq, PartialEq, Clone, Copy, Default)]
pub struct TextureData {
    tex_type: TextureType
}

impl TextureData {
    pub fn from_pathbuf(path: &PathBuf) -> TextureData {
        TextureData{tex_type: TextureType::Special}
    }
}

#[derive(Hash, Debug, Eq, PartialEq, Clone, Copy)]
enum TextureType {
    Diffuse,
    Masks,
    Normal,
    Special
}

impl Default for TextureType {
    fn default() -> TextureType {
        TextureType::Special
    }
}
