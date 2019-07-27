use std::fmt;

use crate::texture::{
    TextureData,
    TextureType,
};

use crate::blender::BlenderData;

#[derive(Hash, Debug, Eq, PartialEq, Clone)]
pub enum Asset {
    Blender(BlenderData),
    Texture(TextureData),
    HighPoly,
    LowPoly,
    GameReady,
    SubstancePainter,
    SubstanceDesigner,
    HandPlaneSettings,
    Autosaved,
    ReferenceImage,
    Other
}

impl Asset {
    fn format_texture(tex_data: &TextureData, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TextureType::*;
        
        match tex_data.tex_type {
            Bake(_) => f.pad("Texture Bake"),
            Special => f.pad("Texture Special"),
            _ => f.pad("Texture")
        }
    }

}

impl fmt::Display for Asset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Asset::Blender(_) => f.pad("Blender"),
            Asset::GameReady => f.pad("Game ready model"),
            Asset::Texture(tex_data) => Asset::format_texture(&tex_data, f),
            Asset::HighPoly => f.pad("High Poly model"),
            Asset::LowPoly => f.pad("Low Poly model"),
            Asset::SubstancePainter => f.pad("Substance Painter"),
            Asset::SubstanceDesigner => f.pad("Substance Designer"),
            Asset::HandPlaneSettings => f.pad("HandPlane Settings"),
            Asset::Autosaved => f.pad("Autosaved file"),
            Asset::ReferenceImage => f.pad("Reference Image"),
            Asset::Other => f.pad("Other"),
        }
    }
}
