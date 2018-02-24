
pub fn bitmap_create(bitmap_size: &mut u64) -> Vec<u8> {
    if *bitmap_size % 8 != 0 {
        *bitmap_size = (*bitmap_size + 1) / 8;
    }
    else {
        *bitmap_size = *bitmap_size / 8;
    }
    let mut _bitmap: Vec<u8> = Vec::with_capacity(*bitmap_size as usize);
    for _i in 0..(*bitmap_size) {
        _bitmap.push(0);
    }
    return _bitmap;
}

pub fn clear_bit(bitmap: &mut Vec<u8>, bitno: u64){
    let index: u64 = bitno / 8;
    let bit_index: u32 = (bitno % 8) as u32;
    let val: u8 = u8::pow(2, bit_index);
    (* bitmap)[index as usize] = (* bitmap)[index as usize] & !(val);
    return;
}

pub fn set_bit(bitmap: &mut Vec<u8>, bitno: u64){
    let index: u64 = bitno / 8;
    let bit_index: u32 = (bitno % 8) as u32;
    let val: u8 = u8::pow(2, bit_index);
    (* bitmap)[index as usize] = (* bitmap)[index as usize] | (val);
    return;
}

pub fn get_first_set_bit(bitmap: &mut Vec<u8>) -> i64{
    let mut _val: u8 = 0;
    for index in 0..bitmap.len() {
        for bit_index in 0..8 {
            _val = u8::pow(2, bit_index);
            if (bitmap[index] & _val) > 0 {
                return ((index as u64) * 8 + (bit_index as u64)) as i64;
            }
        }
    }
    return -1;
}

pub fn get_first_unset_bit(bitmap: &mut Vec<u8>) -> i64{
    let mut _val: u8 = 0;
    for index in 0..bitmap.len() {
        for bit_index in 0..8 {
            _val = u8::pow(2, bit_index);
            if (bitmap[index] & _val) == 0{
                return ((index as u64) * 8 + (bit_index as u64)) as i64;
            }
        }
    }
    return -1;
}

pub fn check_bit(bitmap: &mut Vec<u8>, bitno: u64) -> u32 {
    let index: u64 = bitno / 8;
    let bit_index: u32 = (bitno % 8) as u32;
    return ((bitmap[index as usize] >> bit_index) as u32) & 1;
}