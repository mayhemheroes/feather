// This file is @generated. Please do not edit.
use base::EntityKind;
use vane::EntityBuilder;
use quill_common::entities::Llama;
pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Llama).add(EntityKind::Llama);
}
