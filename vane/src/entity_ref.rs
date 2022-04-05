use crate::{Component, ComponentError, Entity, Ref, RefMut, Entities};

/// Convenient wrapper over an `EntityId` that
/// gives access to components.
pub struct EntityRef<'a> {
    entity: Entity,
    world: &'a Entities,
}

impl<'a> EntityRef<'a> {
    pub(crate) fn new(entity: Entity, world: &'a Entities) -> Self {
        Self { entity, world }
    }

    pub fn get<T: Component>(&self) -> Result<Ref<T>, ComponentError> {
        self.world.get(self.entity)
    }

    pub fn get_mut<T: Component>(&self) -> Result<RefMut<T>, ComponentError> {
        self.world.get_mut(self.entity)
    }
}