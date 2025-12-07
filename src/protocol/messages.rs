use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub struct Channel1;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Message1(pub usize);
