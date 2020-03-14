use crate::{
    components::Component,
    datum::{Client, ExpenseReport, Invoice, Root},
};
use anyhow::{anyhow, Result};
use clap::{App, AppSettings, Arg, ArgMatches};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::str::FromStr;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod components;
mod dataset;
mod datum;
pub use dataset::Dataset;
mod observability;
mod record;

fn main() -> Result<()> {
    Root::execute()
}
