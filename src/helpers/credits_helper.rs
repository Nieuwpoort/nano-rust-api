pub fn is_difficulty_higher_then_receive_block(
    difficulty_hex: &str,
) -> bool {
    const MIN_RECEIVE_DIFFICULTY: u64 = 0xfffffe0000000000;
    
    // Parse hex string (with or without "0x" prefix)
    let hex_str = difficulty_hex.trim_start_matches("0x");
    let difficulty = u64::from_str_radix(hex_str, 16).unwrap_or(0);
    
    difficulty >= MIN_RECEIVE_DIFFICULTY
}