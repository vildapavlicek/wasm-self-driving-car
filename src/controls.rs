use std::sync::atomic::{AtomicBool, Ordering};
use wasm_bindgen::prelude::*;

use crate::log;

#[wasm_bindgen]
#[derive(Debug)]
pub enum KeyEvent {
    UpPressed,
    UpReleased,
    RightPressed,
    RightReleased,
    DownPressed,
    DownReleased,
    LeftPressed,
    LeftReleased,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Controls {
    up: AtomicBool,
    right: AtomicBool,
    down: AtomicBool,
    left: AtomicBool,
}

impl Controls {
    pub fn handle_key_input(&self, event: KeyEvent) {
        match event {
            KeyEvent::UpPressed => self.set_up_pressed(),
            KeyEvent::UpReleased => self.set_up_released(),
            KeyEvent::RightPressed => self.set_right_pressed(),
            KeyEvent::RightReleased => self.set_right_released(),
            KeyEvent::DownPressed => self.set_down_pressed(),
            KeyEvent::DownReleased => self.set_down_released(),
            KeyEvent::LeftPressed => self.set_left_pressed(),
            KeyEvent::LeftReleased => self.set_left_released(),
        }
    }

    pub fn set_up_pressed(&self) {
        self.up.store(true, Ordering::Relaxed);
        log!("up pressed: {:?}", self);
    }
    pub fn set_up_released(&self) {
        self.up.store(false, Ordering::Relaxed);
        log!("up released: {:?}", self);
    }
    pub fn set_right_pressed(&self) {
        self.right.store(true, Ordering::Relaxed);
        log!("right pressed: {:?}", self);
    }
    pub fn set_right_released(&self) {
        self.right.store(false, Ordering::Relaxed);
        log!("right released: {:?}", self);
    }
    pub fn set_down_pressed(&self) {
        self.down.store(true, Ordering::Relaxed);
        log!("down pressed: {:?}", self);
    }
    pub fn set_down_released(&self) {
        self.down.store(false, Ordering::Relaxed);
        log!("down released: {:?}", self);
    }
    pub fn set_left_pressed(&self) {
        self.left.store(true, Ordering::Relaxed);
        log!("left pressed: {:?}", self);
    }
    pub fn set_left_released(&self) {
        self.left.store(false, Ordering::Relaxed);
        log!("left release: {:?}", self);
    }

    // getters
    pub fn up(&self) -> bool {
        self.up.load(Ordering::Relaxed)
    }

    pub fn right(&self) -> bool {
        self.right.load(Ordering::Relaxed)
    }
    pub fn down(&self) -> bool {
        self.down.load(Ordering::Relaxed)
    }
    pub fn left(&self) -> bool {
        self.left.load(Ordering::Relaxed)
    }
}

impl std::default::Default for Controls {
    fn default() -> Self {
        Self {
            up: AtomicBool::new(false),
            right: AtomicBool::new(false),
            down: AtomicBool::new(false),
            left: AtomicBool::new(false),
        }
    }
}
