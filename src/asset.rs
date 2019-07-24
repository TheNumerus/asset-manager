use std::fmt;

use crate::texture::TextureData;

#[derive(Hash, Debug, Eq, PartialEq, Clone)]
pub enum Asset {
    Blender,
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

}

impl fmt::Display for Asset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Asset::Blender => f.pad("Blender"),
            Asset::GameReady => f.pad("Game ready model"),
            Asset::Texture(_) => f.pad("Texture"),
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
