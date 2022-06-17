fn main() {
    assert_eq!("3.2", 3.2_f32.to_string());
    eprintln!("{}", 3.2_f32);

    assert_eq!("-inf", f32::NEG_INFINITY.to_string());
    eprintln!("{}", f32::NEG_INFINITY);
}
