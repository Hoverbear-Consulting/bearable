use anyhow::Result;

pub trait Interactive
where
    Self: std::marker::Sized,
{
    fn interactive() -> Result<Self>;
}
