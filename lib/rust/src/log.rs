use color_eyre::eyre::Result;
use tracing::Level;

#[cfg(debug_assertions)]
const DEFAULT_MAX_LEVEL: Level = Level::DEBUG;
#[cfg(not(debug_assertions))]
const DEFAULT_MAX_LEVEL: Level = Level::INFO;

pub fn init(crate_name: &'static str) -> Result<()> {
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(format!("{crate_name}={DEFAULT_MAX_LEVEL}"))
        .init();

    color_eyre::install()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn init() {
        super::init("food_test").unwrap();
    }
}
