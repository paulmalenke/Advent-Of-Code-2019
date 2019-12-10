fn main() {
    let int_codes: [i64; 137] = get_puzzle_input();

    let int_codes_restored_state: [i64; 137] = restore_last_state(int_codes);
    
    let processed_int_codes: [i64; 137] = process_int_codes(int_codes_restored_state);
    println!("Value [0]: {}", processed_int_codes[0]);
}

fn process_int_codes(mut int_codes: [i64; 137]) -> [i64; 137] {
    let mut index: usize = 0;
    loop {
        let op_code: i64 = int_codes[index];
        if op_code == 99 { break; }

        let position_two: usize = int_codes[index + 1] as usize;
        let position_three: usize = int_codes[index + 2] as usize;
        let position_four: usize = int_codes[index + 3] as usize;

        if op_code == 1 {
            let result: i64 = int_codes[position_two] + int_codes[position_three];
            int_codes[position_four] = result;
        };

        if op_code == 2 {
            let result: i64 = int_codes[position_two] * int_codes[position_three];
            int_codes[position_four] = result;
        };

        index += 4;
    }
    return int_codes;
}

fn restore_last_state(mut int_codes: [i64; 137]) -> [i64; 137] {
    int_codes[1] = 12;
    int_codes[2] = 2;
    return int_codes;
}

fn get_puzzle_input() -> [i64; 137] {
    return [
        1,
        0,
        0,
        3,
        1,
        1,
        2,
        3,
        1,
        3,
        4,
        3,
        1,
        5,
        0,
        3,
        2,
        6,
        1,
        19,
        2,
        19,
        13,
        23,
        1,
        23,
        10,
        27,
        1,
        13,
        27,
        31,
        2,
        31,
        10,
        35,
        1,
        35,
        9,
        39,
        1,
        39,
        13,
        43,
        1,
        13,
        43,
        47,
        1,
        47,
        13,
        51,
        1,
        13,
        51,
        55,
        1,
        5,
        55,
        59,
        2,
        10,
        59,
        63,
        1,
        9,
        63,
        67,
        1,
        6,
        67,
        71,
        2,
        71,
        13,
        75,
        2,
        75,
        13,
        79,
        1,
        79,
        9,
        83,
        2,
        83,
        10,
        87,
        1,
        9,
        87,
        91,
        1,
        6,
        91,
        95,
        1,
        95,
        10,
        99,
        1,
        99,
        13,
        103,
        1,
        13,
        103,
        107,
        2,
        13,
        107,
        111,
        1,
        111,
        9,
        115,
        2,
        115,
        10,
        119,
        1,
        119,
        5,
        123,
        1,
        123,
        2,
        127,
        1,
        127,
        5,
        0,
        99,
        2,
        14,
        0,
        0
    ];
}
