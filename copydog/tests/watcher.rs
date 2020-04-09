use copydog::ConfigBuilder;
use copydog::config::ConfigError;

const CONFIG: &'static str = r#"
source = "."
[fbx]
target = "../destination/models"
ignore = ["ignore"]

[png]
target = "../destination/textures"

["*"]
target = "../destination"
"#;

#[test]
fn parse_test() -> Result<(), ConfigError>{
    let config = ConfigBuilder::new().toml(CONFIG).build()?;
    dbg!(config);
    Ok(())
}