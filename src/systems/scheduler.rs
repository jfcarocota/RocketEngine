use crate::world::World;

/// System trait that all systems must implement
pub trait System {
    fn update(&mut self, world: &mut World, dt: f32);
    fn name(&self) -> &'static str;
}

/// Query-based system trait for systems that use borrow-checked component access
pub trait QuerySystem {
    /// Update the system using query-based component access
    fn update_with_queries(&mut self, world: &mut World, dt: f32);
    fn name(&self) -> &'static str;
}

/// Adapter to make QuerySystem work with the existing System trait
pub struct QuerySystemAdapter<T: QuerySystem> {
    inner: T,
}

impl<T: QuerySystem> QuerySystemAdapter<T> {
    pub fn new(system: T) -> Self {
        Self { inner: system }
    }
}

impl<T: QuerySystem> System for QuerySystemAdapter<T> {
    fn update(&mut self, world: &mut World, dt: f32) {
        self.inner.update_with_queries(world, dt);
    }
    
    fn name(&self) -> &'static str {
        self.inner.name()
    }
}

/// ECS Scheduler for managing update systems
pub struct Scheduler {
    systems: Vec<Box<dyn System>>,
}

impl Scheduler {
    /// Create a new scheduler
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }

    /// Add a system to the scheduler
    pub fn add_system(&mut self, system: Box<dyn System>) {
        self.systems.push(system);
    }
    
    /// Add a query-based system to the scheduler
    pub fn add_query_system<T: QuerySystem + 'static>(&mut self, system: T) {
        self.systems.push(Box::new(QuerySystemAdapter::new(system)));
    }

    /// Update all systems in order
    pub fn update(&mut self, world: &mut World, dt: f32) {
        for system in &mut self.systems {
            system.update(world, dt);
        }
    }

    /// Get the number of systems
    pub fn system_count(&self) -> usize {
        self.systems.len()
    }

    /// Get system names for debugging
    pub fn system_names(&self) -> Vec<&str> {
        self.systems.iter().map(|s| s.name()).collect()
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}
