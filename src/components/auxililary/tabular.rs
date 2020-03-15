use anyhow::Result;

pub trait Tabular
where
    Self: std::marker::Sized,
{
    fn tabulate(&self) -> Result<()>;
}
