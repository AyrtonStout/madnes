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

    pub fn receive_input(&self, key_code: u8) {
    }
}

