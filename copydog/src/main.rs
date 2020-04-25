use std::env;
use std::path::PathBuf;

use anyhow::{Result, Context};

use copydog::{ConfigBuilder};
use copydog::watch::Watcher;


fn main() -> Result<()> {
    let arg = env::args().nth(1).context("Invalid number of arguments.")?;
    let toml = std::fs::read_to_string(&arg).context("Unable to open config file")?;
    let arg_parent = PathBuf::from(arg);
    let arg_parent = arg_parent.parent().context("Unable to get parent folder of config file path.")?;
    let config = ConfigBuilder::new().toml(toml).working_dir(arg_parent).build()?;

    dbg!(&config);

    let mut watcher = Watcher::new(config);

    watcher.start()?;
    println!("Watch started");
    println!("Press any key to stop");

    std::io::stdin().read_line(&mut String::new())?;

    watcher.stop()?;
    println!("Watch stopped");

    Ok(())
}
