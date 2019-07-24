use std::path::PathBuf;

pub mod asset;
pub mod texture;
pub mod stats;

pub use asset::Asset;
pub use texture::TextureData;
pub use stats::Stats;

pub fn format_size(size: u64) -> String {
    let mut size = size as f64;
    let orders = ["B", "kB", "MB", "GB", "TB"];
    let mut order = 0;
    while size > 1024.0 {
        size /= 1024.0;
        order += 1;
    }
    format!("{:.3} {}", size, orders[order])
}

pub fn get_filetype(path: &PathBuf) -> Option<Asset> {
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
    for component in path.components() {
        let comp = component.as_os_str().to_str().unwrap().to_ascii_lowercase();
        if comp == "reference" || comp == "references" {
            return Some(ReferenceImage)
        }
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
                let tex_data = TextureData::from_pathbuf(path);
                Texture(tex_data)
            },
            _ => Other
        }
    } else {
        Other
    };
    Some(filetype)
}
