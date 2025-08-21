use std::{collections::HashSet, hash::Hash};

/// Button map resource for button presses
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ButtonMap<T: Copy + Eq + Hash> {
    pressed: HashSet<T>,
    just_pressed: HashSet<T>,
    just_released: HashSet<T>,
}

impl<T: Copy + Eq + Hash> Default for ButtonMap<T> {
    fn default() -> Self {
        Self {
            pressed: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
        }
    }
}

impl<T: Copy + Eq + Hash> ButtonMap<T> {
    /// Send a button press for specified `input`.
    pub fn press(&mut self, input: T) {
        if self.pressed.insert(input) {
            self.just_pressed.insert(input);
        }
    }

    /// Release specified `input`.
    pub fn release(&mut self, input: T) {
        if self.pressed.remove(&input) {
            self.just_released.insert(input);
        }
    }

    /// Release all currently held `inputs`.
    pub fn release_all(&mut self) {
        self.just_released.extend(self.pressed.drain());
    }

    /// Clear all struct fields.
    pub fn reset(&mut self) {
        self.pressed.clear();
        self.just_pressed.clear();
        self.just_released.clear();
    }

    /// Clear `just_pressed` and `just_released` `inputs`.
    pub fn clear(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }

    /// Returns `true` if the `input` is pressed.
    pub fn pressed(&self, input: T) -> bool {
        self.pressed.contains(&input)
    }

    /// Returns `true` if any of the `inputs` are pressed.
    pub fn any_pressed(&self, inputs: impl IntoIterator<Item = T>) -> bool {
        inputs.into_iter().any(|t| self.pressed(t))
    }

    /// Returns `true` if all of the `inputs` are pressed.
    pub fn all_pressed(&self, inputs: impl IntoIterator<Item = T>) -> bool {
        inputs.into_iter().all(|t| self.pressed(t))
    }

    /// Returns `true` if the `input` was just pressed.
    pub fn just_pressed(&self, input: T) -> bool {
        self.just_pressed.contains(&input)
    }

    /// Returns `true` if any of the `inputs` have just been pressed.
    pub fn any_just_pressed(&self, inputs: impl IntoIterator<Item = T>) -> bool {
        inputs.into_iter().any(|t| self.just_pressed(t))
    }

    /// Reutns `true` if all of the `inputs` have just been pressed.
    pub fn all_just_pressed(&self, inputs: impl IntoIterator<Item = T>) -> bool {
        inputs.into_iter().all(|t| self.just_pressed(t))
    }

    /// Returns `true` if the `input` was just released.
    pub fn just_released(&self, input: T) -> bool {
        self.just_released.contains(&input)
    }

    /// Returns `true` if any of the `inputs` have just been released.
    pub fn any_just_released(&self, inputs: impl IntoIterator<Item = T>) -> bool {
        inputs.into_iter().any(|t| self.just_released(t))
    }

    /// Returns `true` if all of the `inputs` have just been released.
    pub fn all_just_released(&self, inputs: impl IntoIterator<Item = T>) -> bool {
        inputs.into_iter().all(|t| self.just_released(t))
    }

    /// Get every pressed `input`.
    pub fn get_pressed(&self) -> impl ExactSizeIterator<Item = &T> {
        self.pressed.iter()
    }

    /// Get every just pressed `input`.
    pub fn get_just_pressed(&self) -> impl ExactSizeIterator<Item = &T> {
        self.just_pressed.iter()
    }

    /// Get every just released `input`.
    pub fn get_just_released(&self) -> impl ExactSizeIterator<Item = &T> {
        self.just_released.iter()
    }
}
