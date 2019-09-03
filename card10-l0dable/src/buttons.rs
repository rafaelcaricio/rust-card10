use card10_sys::*;

/// Button inputs
pub struct Buttons {
    state: u32,
}

impl Buttons {
    /// Read the current button state
    pub fn read() -> Self {
        let mask = epic_button_BUTTON_LEFT_BOTTOM
            | epic_button_BUTTON_RIGHT_BOTTOM
            | epic_button_BUTTON_LEFT_TOP
            | epic_button_BUTTON_RIGHT_TOP
            | epic_button_BUTTON_RESET;
        let state = unsafe { epic_buttons_read(mask as u8) }.into();
        Buttons { state }
    }

    pub fn left_bottom(&self) -> bool {
        self.state & epic_button_BUTTON_LEFT_BOTTOM != 0
    }

    pub fn right_bottom(&self) -> bool {
        self.state & epic_button_BUTTON_RIGHT_BOTTOM != 0
    }

    pub fn left_top(&self) -> bool {
        self.state & epic_button_BUTTON_LEFT_TOP != 0
    }

    pub fn right_top(&self) -> bool {
        self.state & epic_button_BUTTON_RIGHT_TOP != 0
    }

    pub fn reset(&self) -> bool {
        self.state & epic_button_BUTTON_RESET != 0
    }
}
