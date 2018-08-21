use sdl2::keyboard::Keycode;

// bit:   	 7     6     5     4     3     2     1     0
// button:	 A     B  Select Start  Up   Down  Left  Right
pub struct Controller {
     pub controller1: u8, // A bitmask representing what buttons player1 has held down
     pub controller2: u8
}

impl Controller {
    pub fn new() -> Controller {
        return Controller {
            controller1: 0,
            controller2: 0
        }
    }

    // These are all currently hard coded. But eventually, they should be remappable
    pub fn receive_input(&mut self, keycode: Keycode, is_keydown: bool) {
        match keycode {
            Keycode::Z => self.set_controller_bit(7, is_keydown, true),
            Keycode::X => self.set_controller_bit(6, is_keydown, true),
            Keycode::Backspace => self.set_controller_bit(5, is_keydown, true),
            Keycode::Return => self.set_controller_bit(4, is_keydown, true),
            Keycode::Up => self.set_controller_bit(3, is_keydown, true),
            Keycode::Down => self.set_controller_bit(2, is_keydown, true),
            Keycode::Left => self.set_controller_bit(1, is_keydown, true),
            Keycode::Right => self.set_controller_bit(0, is_keydown, true),
            _ => { }
        }
    }

    fn set_controller_bit(&mut self, bit_num: u8, is_set: bool, is_first_player: bool) {
        let mask = 1 << bit_num;
        if is_first_player {
            if is_set {
                self.controller1 |= mask;
            } else {
                self.controller1 &= !mask;
            }
        } else {
             if is_set {
                self.controller2 |= mask;
            } else {
                self.controller2 &= !mask;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use controller::Controller;
    use sdl2::keyboard::Keycode;

    #[test]
    fn set_and_unset_controller1_b_button() {
        let mut controller = Controller::new();

        controller.receive_input(Keycode::X, true);
        assert_eq!(controller.controller1, 0b0100_0000);

        controller.receive_input(Keycode::X, false);
        assert_eq!(controller.controller1, 0b0000_0000);
    }

    #[test]
    fn changing_multiple_bits_does_not_affect_other_bits() {
        let mut controller = Controller::new();

        controller.receive_input(Keycode::X, true);
        assert_eq!(controller.controller1, 0b0100_0000);

        controller.receive_input(Keycode::Up, true);
        assert_eq!(controller.controller1, 0b0100_1000);

        controller.receive_input(Keycode::X, false);
        assert_eq!(controller.controller1, 0b0000_1000);
    }
}


