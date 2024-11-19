use anyhow::Result;

pub trait Poller {
    fn poll(&mut self) -> Result<()>;
}

pub trait Switch {
    fn toggle(&mut self) -> Result<()>;
}
