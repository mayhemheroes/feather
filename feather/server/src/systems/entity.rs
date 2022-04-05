//! Sends entity-related packets to clients.
//! Spawn packets, position updates, equipment, animations, etc.

use base::{
    metadata::{EntityBitMask, Pose, META_INDEX_ENTITY_BITMASK, META_INDEX_POSE},
    EntityMetadata, Position,
};
use common::world::Dimensions;
use common::Game;
use libcraft_core::Gamemode;
use quill_common::components::{EntityDimension, EntityWorld, PreviousGamemode};
use quill_common::{
    components::{OnGround, Sprinting},
    events::{SneakEvent, SprintEvent},
};
use vane::{SysResult, SystemExecutor};

use crate::{
    entities::{PreviousOnGround, PreviousPosition},
    NetworkId, Server,
};

mod spawn_packet;

pub fn register(game: &mut Game, systems: &mut SystemExecutor<Game>) {
    spawn_packet::register(game, systems);
    systems
        .group::<Server>()
        .add_system(send_entity_movement)
        .add_system(send_entity_sneak_metadata)
        .add_system(send_entity_sprint_metadata);
}

/// Sends entity movement packets.
fn send_entity_movement(game: &mut Game, server: &mut Server) -> SysResult {
    for (
        entity,
        (position, mut prev_position, on_ground, network_id, mut prev_on_ground, dimension, world),
    ) in game
        .ecs
        .query::<(
            &Position,
            &mut PreviousPosition,
            &OnGround,
            &NetworkId,
            &mut PreviousOnGround,
            &EntityDimension,
            &EntityWorld,
        )>()
        .iter()
    {
        if *position != prev_position.0 {
            let mut query = game.ecs.query::<&Dimensions>();
            let dimensions = query.iter().find(|(e, _)| *e == world.0).unwrap().1;
            server.broadcast_nearby_with_mut(*world, &dimension, *position, |client| {
                client.update_entity_position(
                    *network_id,
                    *position,
                    *prev_position,
                    *on_ground,
                    *prev_on_ground,
                    &dimension,
                    *world,
                    &dimensions,
                    game.ecs.get::<Gamemode>(entity).ok().map(|g| *g),
                    game.ecs.get::<PreviousGamemode>(entity).ok().map(|g| *g),
                );
            });
            prev_position.0 = *position;
        }
        if *on_ground != prev_on_ground.0 {
            prev_on_ground.0 = *on_ground;
        }
    }
    Ok(())
}

/// Sends [SendEntityMetadata](protocol::packets::server::play::SendEntityMetadata) packet for when an entity is sneaking.
fn send_entity_sneak_metadata(game: &mut Game, server: &mut Server) -> SysResult {
    for (_, (position, sneak_event, is_sprinting, network_id, world, dimension)) in game
        .ecs
        .query::<(
            &Position,
            &SneakEvent,
            &Sprinting,
            &NetworkId,
            &EntityWorld,
            &EntityDimension,
        )>()
        .iter()
    {
        let mut metadata = EntityMetadata::entity_base();
        let mut bit_mask = EntityBitMask::empty();

        // The Entity can sneak and sprint at the same time, what happens is that when it stops sneaking you immediately start running again.
        bit_mask.set(EntityBitMask::CROUCHED, sneak_event.is_sneaking);
        bit_mask.set(EntityBitMask::SPRINTING, is_sprinting.0);
        metadata.set(META_INDEX_ENTITY_BITMASK, bit_mask.bits());

        if sneak_event.is_sneaking {
            metadata.set(META_INDEX_POSE, Pose::Sneaking);
        } else {
            metadata.set(META_INDEX_POSE, Pose::Standing);
        }

        server.broadcast_nearby_with(*world, &dimension, *position, |client| {
            client.send_entity_metadata(*network_id, metadata.clone());
        });
    }
    Ok(())
}

/// Sends [SendEntityMetadata](protocol::packets::server::play::SendEntityMetadata) packet for when an entity is sprinting.
fn send_entity_sprint_metadata(game: &mut Game, server: &mut Server) -> SysResult {
    for (_, (position, sprint_event, network_id, world, dimension)) in game
        .ecs
        .query::<(
            &Position,
            &SprintEvent,
            &NetworkId,
            &EntityWorld,
            &EntityDimension,
        )>()
        .iter()
    {
        let mut metadata = EntityMetadata::entity_base();
        let mut bit_mask = EntityBitMask::empty();

        bit_mask.set(EntityBitMask::SPRINTING, sprint_event.is_sprinting);
        metadata.set(META_INDEX_ENTITY_BITMASK, bit_mask.bits());

        server.broadcast_nearby_with(*world, &dimension, *position, |client| {
            client.send_entity_metadata(*network_id, metadata.clone());
        });
    }
    Ok(())
}
