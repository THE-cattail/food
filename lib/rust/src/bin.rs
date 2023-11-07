use std::path::Path;

use color_eyre::eyre::{Result, WrapErr};
use serde::Deserialize;

// todo: proc_macro Args::config

pub fn get_args_and_config<A, C>() -> Result<(A, C)>
where
    A: clap::Parser + ConfigPathGetter,
    C: for<'de> Deserialize<'de>,
{
    let args = A::parse();

    let config_path = args.config_path();
    let config_raw = std::fs::read_to_string(config_path)
        .wrap_err_with(|| "failed to read config from file `{config_path}`")?;
    let config = toml::from_str(&config_raw)
        .wrap_err_with(|| "failed to parse toml config:\n```toml\n{config_raw}\n```")?;

    Ok((args, config))
}

pub trait ConfigPathGetter {
    fn config_path(&self) -> &Path;
}
