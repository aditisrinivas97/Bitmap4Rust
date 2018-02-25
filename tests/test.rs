extern crate bitmap4rust;

#[test]
fn t_create_bitmap(){
    let mut size: u64 = 30;
    let mut _bitmap = bitmap4rust::bitmap_create(&mut size);
    assert_eq!(size, 4);
}

#[test]
fn t_check_bit(){
    let mut size: u64 = 30;
    let mut bitmap = bitmap4rust::bitmap_create(&mut size);
    let check = bitmap4rust::check_bit(&mut bitmap, 14);
    assert_eq!(check, 0);
}

#[test]
fn t_set_bit(){
    let mut size: u64 = 30;
    let mut bitmap = bitmap4rust::bitmap_create(&mut size);
    bitmap4rust::set_bit(&mut bitmap, 28);
    let check = bitmap4rust::check_bit(&mut bitmap, 28);
    assert_eq!(check, 1);
}

#[test]
fn t_unset_bit(){
    let mut size: u64 = 30;
    let mut bitmap = bitmap4rust::bitmap_create(&mut size);
    bitmap4rust::set_bit(&mut bitmap, 10);
    bitmap4rust::clear_bit(&mut bitmap, 10);
    let check = bitmap4rust::check_bit(&mut bitmap, 10);
    assert_eq!(check, 0);
}

#[test]
fn t_get_first_set_bit(){
    let mut size: u64 = 30;
    let mut bitmap = bitmap4rust::bitmap_create(&mut size);
    bitmap4rust::set_bit(&mut bitmap, 29);
    bitmap4rust::set_bit(&mut bitmap, 30);
    let check = bitmap4rust::get_first_set_bit(&mut bitmap);
    assert_eq!(check, 29);
}

#[test]
fn t_get_first_unset_bit(){
    let mut size: u64 = 30;
    let mut bitmap = bitmap4rust::bitmap_create(&mut size);
    bitmap4rust::set_bit(&mut bitmap, 0);
    bitmap4rust::set_bit(&mut bitmap, 1);
    bitmap4rust::set_bit(&mut bitmap, 5);
    let check = bitmap4rust::get_first_unset_bit(&mut bitmap);
    assert_eq!(check, 2);
}