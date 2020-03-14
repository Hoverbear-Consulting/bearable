use crate::components::{Component, Client, ExpenseReport, Invoice, Root};
use anyhow::{anyhow, Result};
use clap::{App, AppSettings, Arg, ArgMatches};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;
use std::str::FromStr;

mod components;
mod datatypes;
mod observability;

fn main() -> Result<()> {
    Root::execute()
}
