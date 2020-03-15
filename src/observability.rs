use anyhow::Result;
use std::str::FromStr;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub fn init<'a>(level: impl Into<Option<&'a str>>) -> Result<()> {
    let level = match level.into() {
        Some(l) => Level::from_str(l)?,
        None => Level::INFO,
    };
    let subscriber = FmtSubscriber::builder().with_max_level(level).finish();

    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}
