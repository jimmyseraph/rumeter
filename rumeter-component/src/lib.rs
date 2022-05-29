use async_trait::async_trait;
use record::RecordData;

pub mod group;
pub mod record;
pub mod samplers;
pub mod output;

#[async_trait]
pub trait Sampler {
    async fn run(&self) -> RecordData;
}

#[async_trait]
pub trait Controller {
    async fn run(&self) -> Vec<RecordData>;
}

pub trait Output {
    fn write(&mut self, data: RecordData);
}