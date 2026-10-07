//! The text stacks resource: same-cell floating texts push each other
//! up.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::core::map::components::cell_coord::CellCoord;

/// The floating texts alive per cell. A newcomer pushes the cell's
/// living texts up by one line; an expiring text leaves its stack.
/// Keys with empty stacks are pruned on removal.
#[derive(Resource, Default, Debug)]
pub struct TextStacks {
    by_cell: HashMap<CellCoord, Vec<bevy::ecs::entity::Entity>>,
}

impl TextStacks {
    /// The texts alive on a cell, oldest first.
    pub fn texts_on(&self, cell: CellCoord) -> &[bevy::ecs::entity::Entity] {
        self.by_cell.get(&cell).map_or(&[], Vec::as_slice)
    }

    /// Push a new text onto its cell's stack.
    pub fn push(&mut self, cell: CellCoord, entity: bevy::ecs::entity::Entity) {
        self.by_cell.entry(cell).or_default().push(entity);
    }

    /// Remove an expiring text from its stack.
    pub fn remove(&mut self, cell: CellCoord, entity: bevy::ecs::entity::Entity) {
        if let Some(stack) = self.by_cell.get_mut(&cell) {
            stack.retain(|stacked| *stacked != entity);
            if stack.is_empty() {
                self.by_cell.remove(&cell);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stacks_push_and_remove() {
        let mut stacks = TextStacks::default();
        let cell = CellCoord::new(3, 4);
        // Two distinct entities via spawn on a throwaway world.
        let mut world = World::new();
        let first = world.spawn_empty().id();
        let second = world.spawn_empty().id();
        stacks.push(cell, first);
        stacks.push(cell, second);
        assert_eq!(stacks.texts_on(cell), &[first, second]);
        stacks.remove(cell, first);
        assert_eq!(stacks.texts_on(cell), &[second]);
        stacks.remove(cell, second);
        assert!(stacks.texts_on(cell).is_empty());
    }
}
