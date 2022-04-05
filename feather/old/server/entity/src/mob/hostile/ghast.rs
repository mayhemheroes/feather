use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Ghast;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Ghast).with(Ghast)
}
