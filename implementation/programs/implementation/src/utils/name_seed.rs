pub fn name_seed(name: &str) -> &[u8] {
    let b_name = name.as_bytes();
    if b_name.len() > 32 { &b_name[0..32] } else { b_name } // the max length for any seed is 32 bytes
}
