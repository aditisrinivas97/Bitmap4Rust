extern crate bitmap as bitmap;

#[test]
fn t_create_bitmap(){
    let mut size: u64 = 30;
    let mut _bitmap = bitmap::bitmap_create(&mut size);
    assert_eq!(size, 3);
}

#[test]
fn t_check_bit(){
    let mut size: u64 = 30;
    let mut bitmap = bitmap::bitmap_create(&mut size);
    bitmap::check_bit(&mut bitmap, 3);
}

#[test]
fn t_set_bit(){
    let mut size: u64 = 30;
    let mut bitmap = bitmap::bitmap_create(&mut size);
    bitmap::set_bit(&mut bitmap, 3);
    let check = bitmap::check_bit(&mut bitmap, 3);
    assert_eq!(check, 1);
}

#[test]
fn t_unset_bit(){
    let mut size: u64 = 30;
    let mut bitmap = bitmap::bitmap_create(&mut size);
    bitmap::set_bit(&mut bitmap, 3);
    bitmap::clear_bit(&mut bitmap, 3);
    let check = bitmap::check_bit(&mut bitmap, 3);
    assert_eq!(check, 0);
}

#[test]
fn t_get_first_set_bit(){
    let mut size: u64 = 30;
    let mut bitmap = bitmap::bitmap_create(&mut size);
    bitmap::set_bit(&mut bitmap, 3);
    bitmap::set_bit(&mut bitmap, 5);
    let check = bitmap::get_first_set_bit(&mut bitmap);
    assert_eq!(check, 3);
}

#[test]
fn t_get_first_unset_bit(){
    let mut size: u64 = 30;
    let mut bitmap = bitmap::bitmap_create(&mut size);
    bitmap::set_bit(&mut bitmap, 0);
    bitmap::set_bit(&mut bitmap, 1);
    bitmap::set_bit(&mut bitmap, 5);
    let check = bitmap::get_first_unset_bit(&mut bitmap);
    assert_eq!(check, 2);
}