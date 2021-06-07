pub fn write_compacted_long(values: &[u16], bits_per_value: u32) -> Vec<i64> {
    let mut result = Vec::with_capacity(values.len() / (64 / bits_per_value as usize));
    let mut current_long = 0;
    let mut inserted_bits = 0;
    let mask = 2u16.pow(bits_per_value) - 1;
    for value in values {
        if inserted_bits + bits_per_value > 64 {
            result.push(current_long as i64);
            current_long = 0;
            inserted_bits = 0;
        }
        let mut value = (value & mask) as u64;
        value <<= inserted_bits;
        current_long |= value;
        inserted_bits += bits_per_value;
    }
    if inserted_bits > 0 {
        result.push(current_long as i64);
    }
    result
}

pub fn read_compacted_long(data: &[i64], bits_per_value: u32) -> Vec<u16> {
    let values_per_long = 64 / bits_per_value;
    let mask = 2i64.pow(bits_per_value) - 1;
    let mut result = Vec::with_capacity(values_per_long as usize * data.len());
    for long in data {
        let mut long = *long;
        for _ in 0..values_per_long {
            result.push((long & mask) as u16);
            long >>= bits_per_value;
        }
    }
    result
}

pub fn push_varint(mut value: u32, buffer: &mut Vec<u8>) {
    loop {
        let mut byte = value as u8 & 0b01111111;
        value >>= 7;
        if value != 0 {
            byte |= 0b10000000;
        }
        buffer.push(byte);
        if value == 0 {
            break
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        read_compacted_long,
        write_compacted_long};
    
    #[test]
    fn write_test() {
        let input = vec![1, 2, 2, 3, 4, 4, 5, 6, 6, 4, 8, 0, 7, 4, 3, 13, 15, 16, 9, 14, 10, 12, 0, 2];
        let result = write_compacted_long(&input, 5);
        let expected = vec![0x0020863148418841, 0x01018A7260F68C87];
        assert_eq!(result, expected);
    }

    #[test]
    fn read_test() {
        let input = vec![0x0020863148418841, 0x01018A7260F68C87];
        let result = read_compacted_long(&input, 5);
        let expected = vec![1, 2, 2, 3, 4, 4, 5, 6, 6, 4, 8, 0, 7, 4, 3, 13, 15, 16, 9, 14, 10, 12, 0, 2];
        assert_eq!(result, expected);
    }
}