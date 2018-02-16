fn main() {
    let mut size = 68;
    let mut _vec = bitmap_create(&mut size);
    set_bit(&mut _vec, 8);
    println!("{:?}", _vec);
    clear_bit(&mut _vec, 8);
    println!("{:?}", _vec);
}

fn bitmap_create(bitmap_size: &mut u64) -> Vec<u8> {
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

fn clear_bit(bitmap: &mut Vec<u8>, bitno: u64){
    let index: u64 = bitno / 8;
    let bit_index: u32 = (bitno % 8) as u32;
    let val: u8 = u8::pow(2, bit_index);
    (* bitmap)[index as usize] = (* bitmap)[index as usize] & !(val);
    return;
}

fn set_bit(bitmap: &mut Vec<u8>, bitno: u64){
    let index: u64 = bitno / 8;
    let bit_index: u32 = (bitno % 8) as u32;
    let val: u8 = u8::pow(2, bit_index);
    (* bitmap)[index as usize] = (* bitmap)[index as usize] | (val);
    return;
}
