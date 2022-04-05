use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Horse;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Horse).with(Horse)
}
