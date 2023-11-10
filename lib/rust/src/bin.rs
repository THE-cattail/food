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
    let config = get_config_from_args(args.config_path())?;
    Ok((args, config))
}

fn get_config_from_args<C>(path: &Path) -> Result<C>
where
    C: for<'de> Deserialize<'de>,
{
    let config_raw = std::fs::read_to_string(path)
        .wrap_err_with(|| "failed to read config from file `{config_path}`")?;
    toml::from_str(&config_raw)
        .wrap_err_with(|| "failed to parse toml config:\n```toml\n{config_raw}\n```")
}

pub trait ConfigPathGetter {
    fn config_path(&self) -> &Path;
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use clap::Parser;
    use serde::Deserialize;

    use super::ConfigPathGetter;

    #[derive(Parser)]
    struct Args {
        #[arg(long)]
        config: PathBuf,
        #[arg(long)]
        version: Option<bool>,

        content: Option<String>,
    }

    impl ConfigPathGetter for Args {
        fn config_path(&self) -> &Path {
            &self.config
        }
    }

    #[derive(Debug, PartialEq, Deserialize)]
    struct Config {
        test_token: Option<String>,
    }

    #[test]
    fn get_args_and_config() {
        let mut path = PathBuf::from("./test/");
        path.push("conf");
        path.push("config.toml");

        let args = Args::parse_from(["food", "--config", &path.display().to_string()]);
        assert_eq!(args.config_path(), path);

        let config: Config = super::get_config_from_args(args.config_path()).unwrap();
        assert_eq!(
            config,
            Config {
                test_token: Some("53424C61-1231-43C2-94EC-2ACD74BBEC33".to_owned())
            }
        );
    }
}
