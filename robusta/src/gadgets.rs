use std::collections::HashSet;
use std::mem;

use serde::{Deserialize, Serialize};

#[derive(specta::Type, Clone, Serialize, Deserialize, Debug)]
pub enum MrXGadget {
    AlternativeFacts { stop_id: String },
    Midjourney { image: Vec<u8> },
    NotFound,
    Teleport,
    Shifter,
}

#[derive(specta::Type, Clone, Serialize, Deserialize, Debug)]
pub enum DetectiveGadget {
    Stop { stop_id: String },
    OutOfOrder,
    Shackles,
}

#[derive(Debug)]
pub struct GadgetState<T> {
    can_be_used: bool,
    used: HashSet<mem::Discriminant<T>>,
}

impl<T> GadgetState<T> {
    pub fn new() -> Self {
        Self {
            can_be_used: false,
            used: HashSet::new(),
        }
    }

    pub fn try_use(&mut self, gadget: &T) -> bool {
        if self.can_be_used && self.used.insert(mem::discriminant(gadget)) {
            self.can_be_used = false;
            true
        } else {
            false
        }
    }

    pub fn allow_use(&mut self) {
        self.can_be_used = true;
    }
}
