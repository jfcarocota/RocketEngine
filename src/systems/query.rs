use crate::components::*;
use crate::world::World;
use std::collections::HashMap;

/// Query trait for accessing components with borrow checking
pub trait Query<'world> {
    type Item;
    
    /// Execute the query and return an iterator over matching entities and components
    fn query(world: &'world World) -> Self::Item;
}

/// Mutable query trait for accessing components with mutable borrow checking
pub trait QueryMut<'world> {
    type Item;
    
    /// Execute the query and return an iterator over matching entities and mutable components
    fn query_mut(world: &'world mut World) -> Self::Item;
}

/// Query iterator for entities with a single component
pub struct SingleQuery<'world, T> {
    iter: std::collections::hash_map::Iter<'world, Entity, T>,
}

impl<'world, T> Iterator for SingleQuery<'world, T> {
    type Item = (Entity, &'world T);
    
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(&entity, component)| (entity, component))
    }
}

/// Mutable query iterator for entities with a single component
pub struct SingleQueryMut<'world, T> {
    iter: std::collections::hash_map::IterMut<'world, Entity, T>,
}

impl<'world, T> Iterator for SingleQueryMut<'world, T> {
    type Item = (Entity, &'world mut T);
    
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(&entity, component)| (entity, component))
    }
}

/// Query for entities with Position component
pub struct PositionQuery;

impl<'world> Query<'world> for PositionQuery {
    type Item = SingleQuery<'world, Position>;
    
    fn query(world: &'world World) -> Self::Item {
        SingleQuery {
            iter: world.positions.iter(),
        }
    }
}

impl<'world> QueryMut<'world> for PositionQuery {
    type Item = SingleQueryMut<'world, Position>;
    
    fn query_mut(world: &'world mut World) -> Self::Item {
        SingleQueryMut {
            iter: world.positions.iter_mut(),
        }
    }
}

/// Query for entities with Velocity component
pub struct VelocityQuery;

impl<'world> Query<'world> for VelocityQuery {
    type Item = SingleQuery<'world, Velocity>;
    
    fn query(world: &'world World) -> Self::Item {
        SingleQuery {
            iter: world.velocities.iter(),
        }
    }
}

impl<'world> QueryMut<'world> for VelocityQuery {
    type Item = SingleQueryMut<'world, Velocity>;
    
    fn query_mut(world: &'world mut World) -> Self::Item {
        SingleQueryMut {
            iter: world.velocities.iter_mut(),
        }
    }
}

/// Query for entities with Sprite component
pub struct SpriteQuery;

impl<'world> Query<'world> for SpriteQuery {
    type Item = SingleQuery<'world, Sprite>;
    
    fn query(world: &'world World) -> Self::Item {
        SingleQuery {
            iter: world.sprites.iter(),
        }
    }
}

impl<'world> QueryMut<'world> for SpriteQuery {
    type Item = SingleQueryMut<'world, Sprite>;
    
    fn query_mut(world: &'world mut World) -> Self::Item {
        SingleQueryMut {
            iter: world.sprites.iter_mut(),
        }
    }
}

/// Query iterator for entities with two components (Position + Velocity)
pub struct PositionVelocityQuery<'world> {
    positions: &'world HashMap<Entity, Position>,
    velocities: &'world HashMap<Entity, Velocity>,
    entity_iter: std::collections::hash_map::Keys<'world, Entity, Position>,
}

impl<'world> Iterator for PositionVelocityQuery<'world> {
    type Item = (Entity, &'world Position, &'world Velocity);
    
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(&entity) = self.entity_iter.next() {
            if let (Some(position), Some(velocity)) = (
                self.positions.get(&entity),
                self.velocities.get(&entity)
            ) {
                return Some((entity, position, velocity));
            }
        }
        None
    }
}

// Note: Mutable queries for multiple components require more sophisticated 
// lifetime management to ensure safe borrow checking. For now, we provide
// separate mutable access to individual component types.

/// Query for entities with both Position and Velocity components
pub struct MovementQuery;

impl<'world> Query<'world> for MovementQuery {
    type Item = PositionVelocityQuery<'world>;
    
    fn query(world: &'world World) -> Self::Item {
        PositionVelocityQuery {
            positions: &world.positions,
            velocities: &world.velocities,
            entity_iter: world.positions.keys(),
        }
    }
}

/// Query for entities with Position, Velocity, and Sprite components
pub struct RenderableMovementQuery<'world> {
    positions: &'world HashMap<Entity, Position>,
    velocities: &'world HashMap<Entity, Velocity>,
    sprites: &'world HashMap<Entity, Sprite>,
    entity_iter: std::collections::hash_map::Keys<'world, Entity, Position>,
}

impl<'world> Iterator for RenderableMovementQuery<'world> {
    type Item = (Entity, &'world Position, &'world Velocity, &'world Sprite);
    
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(&entity) = self.entity_iter.next() {
            if let (Some(position), Some(velocity), Some(sprite)) = (
                self.positions.get(&entity),
                self.velocities.get(&entity),
                self.sprites.get(&entity)
            ) {
                return Some((entity, position, velocity, sprite));
            }
        }
        None
    }
}

/// Query for entities with Position, Velocity, and Sprite components
pub struct RenderableQuery;

impl<'world> Query<'world> for RenderableQuery {
    type Item = RenderableMovementQuery<'world>;
    
    fn query(world: &'world World) -> Self::Item {
        RenderableMovementQuery {
            positions: &world.positions,
            velocities: &world.velocities,
            sprites: &world.sprites,
            entity_iter: world.positions.keys(),
        }
    }
}

/// Helper macros for common query patterns
#[macro_export]
macro_rules! query {
    ($world:expr, Position) => {
        $crate::systems::query::PositionQuery::query($world)
    };
    ($world:expr, Velocity) => {
        $crate::systems::query::VelocityQuery::query($world)
    };
    ($world:expr, Sprite) => {
        $crate::systems::query::SpriteQuery::query($world)
    };
    ($world:expr, Position, Velocity) => {
        $crate::systems::query::MovementQuery::query($world)
    };
    ($world:expr, Position, Velocity, Sprite) => {
        $crate::systems::query::RenderableQuery::query($world)
    };
}

#[macro_export]
macro_rules! query_mut {
    ($world:expr, Position) => {
        $crate::systems::query::PositionQuery::query_mut($world)
    };
    ($world:expr, Velocity) => {
        $crate::systems::query::VelocityQuery::query_mut($world)
    };
    ($world:expr, Sprite) => {
        $crate::systems::query::SpriteQuery::query_mut($world)
    };
}
