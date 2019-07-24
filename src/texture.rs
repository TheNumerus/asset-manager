use std::fmt;
use std::path::PathBuf;

const BAKE_NAMES: &'static [(&'static str, BakeType)] = &[
    ("ao", BakeType::Ao),
    ("ws", BakeType::WorldNormal),
    ("os", BakeType::WorldNormal), 
    ("normal", BakeType::Normal),
    ("ts", BakeType::Normal),
    ("vg", BakeType::Position),
    ("pos", BakeType::Position),
    ("matid", BakeType::MatId),
    ("curve", BakeType::Curvature),
    ("color", BakeType::VertexColor)
];

const TEX_TYPE_NAMES: &'static [(&'static str, TextureType)] = &[
    ("basecolor", TextureType::BaseColor),
    ("albedo", TextureType::BaseColor),
    ("diffuse", TextureType::BaseColor), 
    ("normal", TextureType::Normal),
    ("ao", TextureType::Ao),
    ("roughness", TextureType::Roughness),
    ("smoothness", TextureType::Smoothness),
    ("metalic", TextureType::Metalic),
    ("rma", TextureType::Masks),
    ("maskmap", TextureType::Masks)
];

#[derive(Hash, Debug, Eq, PartialEq, Clone, Default)]
pub struct TextureData {
    tex_type: TextureType,
    name: String,
    extension: String
}

impl TextureData {
    pub fn from_pathbuf(path: &PathBuf) -> TextureData {
        let mut tex_type = TextureType::Special;

        let filename = path.file_stem().unwrap().to_str().unwrap().to_ascii_lowercase();
        if filename.contains("bake") {
            tex_type = TextureType::Bake(TextureData::get_bake_type(path));
        } else {
            for (key, val) in TEX_TYPE_NAMES {
                if filename.ends_with(key) {
                    tex_type = *val;
                }
            }
        }

        let name = TextureData::get_name(path);
        let extension = path.extension().unwrap().to_str().unwrap().to_owned();
        TextureData{tex_type, name, extension}
    }

    fn get_bake_type(path: &PathBuf) -> BakeType {
        let filename = path.file_stem().unwrap().to_str().unwrap().to_ascii_lowercase();

        for (key, val) in BAKE_NAMES {
            if filename.ends_with(key) {
                return *val;
            }
        }
        BakeType::Unknown
    }

    fn get_name(path: &PathBuf) -> String {
        let mut name = path.file_stem().unwrap().to_str().unwrap();

        name = name.trim_start_matches("T_");

        let lowercase_name = name.to_ascii_lowercase();
        for (key, _val) in BAKE_NAMES {
            if lowercase_name.ends_with(key) {
                name = name.trim_end_matches(key);
                break;
            }
        }
        let lowercase_name = name.to_ascii_lowercase();
        for (key, _val) in TEX_TYPE_NAMES {
            let matches: Vec<_> = lowercase_name.rmatch_indices(key).collect();
            if matches.len() > 0 {
                name = name.split_at(matches[0].0).0;
                break;
            }
        }

        name = name.trim_end_matches("_");
        name = name.trim_end_matches("bake");
        name = name.trim_end_matches("Bake");
        name = name.trim_end_matches("_");

        // if empty, add folder name to name

        if name.len() == 0 {
            return String::from(path.components().rev().nth(1).unwrap().as_os_str().to_str().unwrap());
        }

        String::from(name)
    }

    pub fn generate_filename(&self) -> String {
        format!("T_{}_{}.{}", self.name, self.tex_type, self.extension)
    }
}

#[derive(Hash, Debug, Eq, PartialEq, Clone, Copy)]
enum TextureType {
    BaseColor,
    Masks,
    Roughness,
    Metalic,
    Smoothness,
    Ao,
    Normal,
    Bake(BakeType),
    Special
}

impl Default for TextureType {
    fn default() -> TextureType {
        TextureType::Special
    }
}

impl fmt::Display for TextureType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TextureType::Ao => write!(f,"AO"),
            TextureType::BaseColor => write!(f,"BaseColor"),
            TextureType::Masks => write!(f,"Masks"),
            TextureType::Smoothness => write!(f,"Smoothness"),
            TextureType::Roughness => write!(f,"Roughness"),
            TextureType::Metalic => write!(f,"Metalic"),
            TextureType::Normal => write!(f,"Normal"),
            TextureType::Special => Ok(()),
            TextureType::Bake(bake) => write!(f, "bake_{}", bake),
        }
    }
}

#[derive(Hash, Debug, Eq, PartialEq, Clone, Copy)]
enum BakeType {
    Normal,
    Ao,
    VertexColor,
    MatId,
    WorldNormal,
    Curvature,
    Position,
    Unknown
}

impl fmt::Display for BakeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            BakeType::Ao => write!(f,"AO"),
            BakeType::Normal => write!(f,"normal"),
            BakeType::VertexColor => write!(f,"vcol"),
            BakeType::MatId => write!(f,"matid"),
            BakeType::WorldNormal => write!(f,"onormal"),
            BakeType::Curvature => write!(f,"curve"),
            BakeType::Position => write!(f,"pos"),
            BakeType::Unknown => write!(f,"unknown"),
        }
    }
}
