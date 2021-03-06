use std::any::Any;

mod domain_levels;
mod exact;
mod pattern;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct Slot(pub u64);

/// Carries both the matched slot and dynamic information about it
pub struct StrategyMatch {
    pub slot: Slot,
    pub any: Option<Box<dyn Any + Send + Sync + 'static>>
}

pub trait Strategy: 'static + Sync {
    type Builder: Builder;

    fn r#match(&self, string: &str) -> Option<StrategyMatch>;

    fn builder() -> Self::Builder {
        Self::Builder::default()
    }
}

pub trait Builder: Default {
    type Strategy: Strategy;
    type Error;

    fn build(self) -> Result<Self::Strategy, Self::Error>;

    // Input is expected to be normalized and NOT url-encoded
    fn add(&mut self, string: &str, slot: Slot) -> &mut Self;
    fn add_unnormalized(&mut self, segment: &str, slot: Slot) -> &mut Self {
        let normalized = bolt_url::normalize_str(segment);
        self.add(&normalized, slot)
    }

    fn add_owned(mut self, string: &str, slot: Slot) -> Self {
        self.add(&string, slot);
        self
    }
    fn add_unnormalized_owned(mut self, segment: &str, slot: Slot) -> Self {
        self.add_unnormalized(&segment, slot);
        self
    }
}
