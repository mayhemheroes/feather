// This file is @generated. Please do not edit.
use base::EntityKind;
use vane::EntityBuilder;
use quill_common::entities::Drowned;
pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Drowned).add(EntityKind::Drowned);
}
