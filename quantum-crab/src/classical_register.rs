#[derive(Debug, PartialEq, Clone)]
pub struct ClassicalRegister {
    bits: Vec<u8>,
}

impl ClassicalRegister {
    pub fn new(bits: Vec<u8>) -> ClassicalRegister {
        for bit in &bits {
            assert!(*bit == 0 || *bit == 1);
        }

        ClassicalRegister { bits }
    }

    pub fn zeroed(width: usize) -> ClassicalRegister {
        ClassicalRegister::new(vec![0; width])
    }

    pub fn from_bitstring(width: usize, bitstring: u32) -> ClassicalRegister {
        ClassicalRegister::from_value(width, bitstring)
    }

    pub fn from_value(width: usize, value: u32) -> ClassicalRegister {
        assert!(value < 2u32.pow(width as u32));

        let mut bits = Vec::new();
        let mut remaining = value;

        for i in 0..width {
            let position = (width - i - 1) as u32;
            let value = 2u32.pow(position);

            if value <= remaining {
                remaining -= value;
                bits.insert(0, 1);
            } else {
                bits.insert(0, 0);
            }
        }

        ClassicalRegister::new(bits)
    }

    pub fn width(&self) -> usize {
        self.bits.len()
    }

    pub fn value(&self) -> u32 {
        let mut value = 0;

        for (position, bit) in self.bits.iter().enumerate() {
            if *bit != 0 {
                value += 2u32.pow(position as u32)
            }
        }

        value
    }
}
