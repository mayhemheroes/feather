// This file is @generated. Please do not edit.
use base::EntityKind;
use vane::EntityBuilder;
use quill_common::entities::EndCrystal;
pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(EndCrystal).add(EntityKind::EndCrystal);
}
