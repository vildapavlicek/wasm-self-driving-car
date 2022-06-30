use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, serde::Serialize, serde::Deserialize, Copy, Clone)]
pub enum ControlType {
    Keyboard,
    NoControl,
    Ai,
}

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
#[derive(Debug, Clone, Copy)]
pub struct Controls {
    pub(crate) control_type: ControlType,
    pub up: bool,
    pub left: bool,
    pub right: bool,
    pub down: bool,
}

impl Controls {
    pub fn new(control_type: ControlType) -> Self {
        Controls {
            control_type,
            up: false,
            right: false,
            down: false,
            left: false,
        }
    }

    pub fn handle_key_input(&mut self, event: KeyEvent) {
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

    pub fn set_up_pressed(&mut self) {
        self.up = true
    }
    pub fn set_up_released(&mut self) {
        self.up = false
    }
    pub fn set_right_pressed(&mut self) {
        self.right = true
    }
    pub fn set_right_released(&mut self) {
        self.right = false
    }
    pub fn set_down_pressed(&mut self) {
        self.down = true
    }
    pub fn set_down_released(&mut self) {
        self.down = false
    }
    pub fn set_left_pressed(&mut self) {
        self.left = true
    }
    pub fn set_left_released(&mut self) {
        self.left = false
    }

    // getters
    pub fn up(&self) -> bool {
        self.up
    }

    pub fn right(&self) -> bool {
        self.right
    }
    pub fn down(&self) -> bool {
        self.down
    }
    pub fn left(&self) -> bool {
        self.left
    }
}

impl std::default::Default for Controls {
    fn default() -> Self {
        Self {
            control_type: ControlType::NoControl,
            up: true,
            right: false,
            down: false,
            left: false,
        }
    }
}
