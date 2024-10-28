use steel::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct HelloWorld {
    pub message: [u8; 32],  // A fixed-size array to hold the message "Hello World!"
}

account!(HelloWorld);
