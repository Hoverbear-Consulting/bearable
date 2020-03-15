use crate::{components::Component, structures::Root};
use anyhow::Result;

mod components;
mod dataset;
mod field;
mod observability;
mod scope;
mod store;
mod structures;

fn main() -> Result<()> {
    Root::execute()
}
