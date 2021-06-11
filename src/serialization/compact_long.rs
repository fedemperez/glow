use num_integer::Integer;

pub struct CompactLong {
    pub longs: Vec<i64>,
    bits: u8,
    length: usize,
}

impl CompactLong {
    pub fn new(longs: Vec<i64>, bits: u8) -> Self {
        let length = longs.len() * 64 / bits as usize;
        Self { longs, bits, length }
    }

    pub fn from_values(values: &[u16], bits: u8) -> Self {
        let mut longs = Vec::with_capacity(values.len() / (64 / bits as usize));
        let mut next_long = 0;
        let mut bits_written = 0;
        let mask = (1 << bits) - 1;
        for value in values {
            let value = (value & mask) as i64;
            next_long |= value << bits_written;
            bits_written += bits;
            if bits_written + bits > 64 {
                longs.push(next_long);
                next_long = 0;
                bits_written = 0;
            }
        }
        if bits_written > 0 {
            longs.push(next_long);
        }
        Self {
            longs, bits, length: values.len(),
        }
    }

    pub fn get(&self, index: usize) -> i64 {
        let (index, displace) = self.location(index);
        (self.longs[index] >> displace) & self.mask()
    }

    pub fn set(&mut self, index: usize, value: i64) {
        let (index, displace) = self.location(index);
        let value = value & self.mask();
        self.longs[index] &= !(self.mask() << displace);
        self.longs[index] |= value << displace;
    }

    pub fn set_bits(&mut self, bits: u8) {
        if bits != self.bits {
            let mask = self.mask();
            let values_per_long = 64 / self.bits as usize;
            let mut new = vec![];
            let mut next_long = 0;
            let mut bits_written = 0;
            let mut values_written = 0;
            'main: for long in &self.longs {
                let mut old_long = *long;
                for _ in 0..values_per_long {
                    next_long |= (old_long & mask) << bits_written;
                    old_long >>= self.bits;
                    bits_written += bits;
                    values_written += 1;
                    if values_written >= self.length {
                        if bits_written > 0 {
                            new.push(next_long);
                        }
                        break 'main;
                    }
                    if bits_written + bits > 64 {
                        new.push(next_long);
                        next_long = 0;
                        bits_written = 0;
                    }
                }
            }
            self.longs = new;
            self.bits = bits;
        }
    }

    fn location(&self, index: usize) -> (usize, usize) {
        let items_per_long = 64 / self.bits as usize;
        let (index, displace) = index.div_rem(&items_per_long);
        (index, displace * self.bits as usize)
    }

    fn mask(&self) -> i64 {
        (1 << self.bits) - 1
    }
}

#[cfg(test)]
mod tests {
    use super::CompactLong;
    
    #[test]
    fn from_values_test() {
        let input = vec![1, 2, 2, 3, 4, 4, 5, 6, 6, 4, 8, 0, 7, 4, 3, 13, 15, 16, 9, 14, 10, 12, 0, 2];
        let complong = CompactLong::from_values(&input, 5);
        let expected = vec![0x0020863148418841, 0x01018A7260F68C87];
        assert_eq!(complong.longs, expected);
    }

    #[test]
    fn set_bits_test() {
        let input = vec![1, 2, 2, 3, 4, 4, 5, 6, 6, 4, 8, 0, 7, 4, 3, 13, 15, 11, 9, 14, 10, 12, 0, 2];
        let mut complong = CompactLong::from_values(&input, 4);
        complong.set_bits(5);
        let expected = vec![0x20863148418841, 0x1018A7256F68C87];
        assert_eq!(complong.longs, expected);
    }

    #[test]
    fn get_test() {
        let complong = CompactLong::new(
            vec![0x0020863148418841, 0x01018A7260F68C87], 
            5);
        let expected = vec![1, 2, 2, 3, 4, 4, 5, 6, 6, 4, 8, 0, 7, 4, 3, 13, 15, 16, 9, 14, 10, 12, 0, 2];
        for (i, n) in expected.iter().enumerate() {
            assert_eq!(*n, complong.get(i));
        }
    }

    #[test]
    fn set_test() {
        let mut complong = CompactLong::new(
            vec![0x0020863148418841, 0x01018A7260F68C87], 
            5);
        complong.set(8, 0);
        complong.set(0, 5);
        complong.set(23, 21);
        let expected = vec![5, 2, 2, 3, 4, 4, 5, 6, 0, 4, 8, 0, 7, 4, 3, 13, 15, 16, 9, 14, 10, 12, 0, 21];
        for (i, n) in expected.iter().enumerate() {
            assert_eq!(*n, complong.get(i));
        }
    }
}
