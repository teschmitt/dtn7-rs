use super::ConvergencyLayerAgent;
use crate::core::DtnCore;
use bp7::ByteBuffer;
use log::{debug, error, info, trace, warn};

#[derive(Debug, Clone, Default)]
pub struct DummyConversionLayer {
    counter: u64,
}

impl DummyConversionLayer {
    pub fn new() -> DummyConversionLayer {
        DummyConversionLayer { counter: 0 }
    }
}
impl ConvergencyLayerAgent for DummyConversionLayer {
    fn setup(&mut self) {
        debug!("Setup Dummy Conversion Layer");
    }
    fn scheduled_process(&self, ready: &[ByteBuffer], keys: &Vec<String>) {
        debug!("Scheduled process Dummy Conversion Layer");
    }
}

impl std::fmt::Display for DummyConversionLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "DummyConversionLayer")
    }
}