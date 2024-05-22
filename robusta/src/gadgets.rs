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
    cooldown: Option<f32>,
    used: HashSet<mem::Discriminant<T>>,
}

impl<T> GadgetState<T> {
    pub fn new() -> Self {
        Self {
            can_be_used: false,
            cooldown: None,
            used: HashSet::new(),
        }
    }

    pub fn update_time(&mut self, delta: f32) {
        if let Some(cooldown) = self.cooldown.as_mut() {
            *cooldown -= delta;
            if *cooldown < 0.0 {
                self.cooldown = None;
            }
        }
    }

    pub fn remaining(&self) -> Option<f32> {
        self.cooldown
    }

    pub fn try_use(&mut self, gadget: &T, cooldown: f32) -> bool {
        if self.can_be_used && self.cooldown.is_none() && self.used.insert(mem::discriminant(gadget)) {
            self.cooldown = Some(cooldown);
            true
        } else {
            false
        }
    }

    pub fn allow_use(&mut self) {
        self.can_be_used = true;
    }
}
