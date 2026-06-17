use super_seq_macro::seq;

pub trait BitVec {
    const SIZE: u8;

    fn push(&mut self, bit: bool);

    fn get_at(&self, index: usize) -> bool;
}

seq!(N in (3..=7).collect().map(|i| 1 << i) {

    pub struct BitVec~N {
        data: u~N,
        index: u8,
    }

    impl BitVec~N {
        pub const fn new() -> Self {
            Self {
                data: 0,
                index: 0,
            }
        }
    }

    impl BitVec for BitVec~N {
        const SIZE: u8 = (size_of::<u~N>() * 8) as u8;

        fn push(&mut self, bit: bool) {
            if self.index == Self::SIZE {
                panic!("BitVec is full!");
            }
            if bit {
                self.data |= 1 << self.index;
            }
            self.index += 1;
        }

        fn get_at(&self, index: usize) -> bool {
            if index >= self.index as usize {
                panic!("Index out of range!");
            }
            self.data & (1 << index) != 0
        }
    }
});

