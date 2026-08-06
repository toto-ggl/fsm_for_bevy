use bevy::prelude::{Commands, Component, Entity};
use std::fmt::Debug;
use strum::IntoEnumIterator;
use utils::variant_eq;

pub mod prelude {
    pub use crate::FSM;
    pub use fsm_for_bevy_macros::{FSM, derive_fsm};
    pub use strum::{Display, EnumIter};
}

mod utils;

/// t
pub trait FSM: Component + Copy + Clone + Default + IntoEnumIterator {
    /// Init the state machine by inserting the default state to the entity.
    fn init(commands: &mut Commands, entity: Entity) {
        commands.entity(entity).insert(Self::default());
        Self::insert_marker(Self::default(), commands, entity);
    }

    /// Describes which marker component belongs to which state.
    fn insert_marker(state: Self, commands: &mut Commands, entity: Entity);

    /// Describes which marker component belongs to which state.
    fn remove_marker(state: Self, commands: &mut Commands, entity: Entity);

    /// Enter a new state if different.
    ///
    /// - Update self with new state.
    /// - Insert the marker component associated with the new state and remove those associated with other fsm.
    fn enter(&self, new_state: Self, commands: &mut Commands, entity: Entity)
    where
        Self: Component + PartialEq<Self> + Debug,
    {
        // Prevent from re-entering the same state.
        if *self == new_state {
            return;
        }

        // Update self.
        commands.entity(entity).insert(new_state);
        // Insert and remove marker components.
        for state in Self::iter() {
            if variant_eq(&state, &new_state) {
                Self::insert_marker(new_state, commands, entity);
            } else {
                Self::remove_marker(state, commands, entity);
            }
        }
    }
}
