use sdl2::keyboard::Keycode;

// bit:   	 7     6     5     4     3     2     1     0
// button:	 A     B  Select Start  Up   Down  Left  Right
pub struct Controlletron {
    strobe: u8, // Alternates. When it goes from 1 to 0, the current value of the controller is saved to the latch to be read from
    controller1: u8, // A bitmask representing what buttons player1 has held down
    controller1_latched: u8, // A stored snapshot of controller input. Read from by the game
    controller1_latch_position: u8, // The index of the next latched bit position to be read from. Only one button is read from at a time
    controller2: u8,
    controller2_latched: u8,
    controller2_latch_position: u8
}

static CONTROLLER1_MEMORY: u16 = 0x4016;

impl Controlletron {
    pub fn new() -> Controlletron {
        return Controlletron {
            strobe: 0,
            controller1: 0,
            controller1_latched: 0,
            controller1_latch_position: 8,
            controller2: 0,
            controller2_latched: 0,
            controller2_latch_position: 8
        }
    }

    pub fn receive_memory_write(&mut self, write_value: u8) {
        if write_value == 1 {
            self.strobe = 1;
        } else if write_value == 0 && self.strobe == 1 {
            self.controller1_latched = self.controller1;
            self.controller2_latched = self.controller2;
            self.controller1_latch_position = 8;
            self.controller2_latch_position = 8;
            self.strobe = 0;
        }
    }

    pub fn read_controller_value(&mut self, controller_memory: u16) -> u8 {
        let read_controller_1 = controller_memory == CONTROLLER1_MEMORY;

        // If strobe is set to 1, just read the currently active value of 'A'. Not a standard thing for a game to do
        if self.strobe == 1 {
            if read_controller_1 {
                return (self.controller1 & 0b1000_0000) >> 7;
            } else {
                return (self.controller2 & 0b1000_0000) >> 7;
            }
        }

        // Strobe is 0. Read the NEXT latched value. Each read will read the NEXT button. It takes 8 total reads to read them all!
        // TODO what to do if a game decides to read more than 8 times? Currently will experience an underflow error and crash!
        if read_controller_1 {
            self.controller1_latch_position -= 1;
            return (self.controller1_latched & 1 << self.controller1_latch_position) >> self.controller1_latch_position;
        } else {
            self.controller2_latch_position -= 1;
            return (self.controller2_latched & 1 << self.controller2_latch_position) >> self.controller2_latch_position;
        }
    }

    // These are all currently hard coded. But eventually, they should be remappable
    pub fn receive_key_input(&mut self, keycode: Keycode, is_keydown: bool) {
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
    use controlletron::Controlletron;
    use sdl2::keyboard::Keycode;

    #[test]
    fn set_and_unset_controller1_b_button() {
        let mut controlletron = Controlletron::new();

        controlletron.receive_key_input(Keycode::X, true);
        assert_eq!(controlletron.controller1, 0b0100_0000);

        controlletron.receive_key_input(Keycode::X, false);
        assert_eq!(controlletron.controller1, 0b0000_0000);
    }

    #[test]
    fn changing_multiple_bits_does_not_affect_other_bits() {
        let mut controlletron = Controlletron::new();

        controlletron.receive_key_input(Keycode::X, true);
        assert_eq!(controlletron.controller1, 0b0100_0000);

        controlletron.receive_key_input(Keycode::Up, true);
        assert_eq!(controlletron.controller1, 0b0100_1000);

        controlletron.receive_key_input(Keycode::X, false);
        assert_eq!(controlletron.controller1, 0b0000_1000);
    }

    #[test]
    fn changing_strobe_from_1_to_0_latches_controller_values() {
        let mut controlletron = Controlletron::new();

        controlletron.receive_key_input(Keycode::Z, true);
        controlletron.receive_memory_write(1);
        controlletron.receive_memory_write(0);
        controlletron.receive_key_input(Keycode::Z, false);

        assert_eq!(controlletron.controller1, 0b0000_0000);
        assert_eq!(controlletron.controller1_latched, 0b1000_0000);
        assert_eq!(controlletron.controller1_latch_position, 8);
    }

    #[test]
    fn reading_reads_through_the_latch_one_at_a_time() {
        let mut controlletron = Controlletron::new();
        controlletron.controller1_latched = 0b1001_0010;
        controlletron.controller2_latched = 0b0101_0010;

        assert_eq!(controlletron.read_controller_value(0x4016), 1);
        assert_eq!(controlletron.read_controller_value(0x4016), 0);
        assert_eq!(controlletron.read_controller_value(0x4017), 1);
    }
}


