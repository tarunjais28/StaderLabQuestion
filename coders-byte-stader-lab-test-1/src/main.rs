fn main() {
    let input = "aabbcde";
    println!("Question1: a. {} = {}", input, run_len(input));
    let input = "wwwbbbw";
    println!("Question1: b. {} = {}", input, run_len(input));
    let input = "";
    println!("Question1: c. {}(empty) = {}(empty)", input, run_len(input));
    let input = "wwwggopp";
    println!("Question1: d. {}(empty) = {}(empty)", input, run_len(input));

    let arr = [1, 5, 4, 6, 9, 3, 0, 0, 1, 3];
    println!(
        "Question2: a. {:?} = {}",
        arr,
        array_min_jump(&arr, arr.len())
    );
    let arr = [3, 4, 2, 1, 1, 100];
    println!(
        "Question2: b. {:?} = {}",
        arr,
        array_min_jump(&arr, arr.len())
    );
    let arr = [1, 3, 6, 8, 2, 7, 1, 2, 1, 2, 6, 1, 2, 1, 2];
    println!(
        "Question2: c. {:?} = {}",
        arr,
        array_min_jump(&arr, arr.len())
    );
    let arr = [];
    println!(
        "Question2: d. {:?}(empty) = {}",
        arr,
        array_min_jump(&arr, arr.len())
    );
    let arr = [1];
    println!(
        "Question2: e. {:?} = {}",
        arr,
        array_min_jump(&arr, arr.len())
    );
}

fn run_len(str: &str) -> String {
    let mut output = String::new();
    let str_bytes = str.as_bytes();
    let mut prev_byte = str_bytes.get(0).unwrap_or(&0);
    let mut count = 0;
    output.push(*prev_byte as char);
    for byte in str_bytes.iter() {
        if byte.eq(prev_byte) {
            count += 1;
        } else {
            output.push_str(&format!("{}{}", count, *byte as char));
            prev_byte = byte;
            count = 1;
        }
    }
    output.retain(|out| out != '\n');
    
    output
}

fn array_min_jump(arr: &[i32], len: usize) -> i32 {
    if arr.is_empty() {
        return -1;
    }

    if len == 1 {
        return 0;
    }

    let mut iterations = i32::MAX;
    for i in (0..(len - 1)).rev() {
        if arr[i] + i as i32 >= len as i32 - 1 {
            let mid_iter = array_min_jump(&arr, i + 1);
            if mid_iter != i32::MAX {
                iterations = iterations.min(mid_iter + 1);
            }
        }
    }
    
    iterations
}
