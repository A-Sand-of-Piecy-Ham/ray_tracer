use std::cell::RefCell;

use rand::{rngs::SmallRng, Rng};



#[derive(Debug, Clone, Default)]
pub enum Material {
    /// Colors based off shape normals
    #[default]
    Debug,
    RandomDiffuse(RefCell<SmallRng>)
}

