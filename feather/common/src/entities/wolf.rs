// This file is @generated. Please do not edit.
use base::EntityKind;
use vane::EntityBuilder;
use quill_common::entities::Wolf;
pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Wolf).add(EntityKind::Wolf);
}
