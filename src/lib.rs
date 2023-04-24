use std::{thread, time};

fn collatz_condition(x: i32) -> i32 {
    let one_millis = time::Duration::from_millis(1);
    thread::sleep(one_millis);
    if x % 2 == 0 {
        return x / 2
    } else {
        return 3 * x + 1
    }
}

fn find_collatz_sequence(x: i32) -> Vec<i32> {
    let mut collatz_sequence = Vec::new();
    let mut p = x;

    while p > 1 {
        collatz_sequence.push(p);
        p = collatz_condition(p);
    }

    return collatz_sequence;
}

fn find_collatz_sequence_dynamically(x: i32, collatz_sequences: Vec<Vec<i32>>) -> Vec<i32> {
    let mut collatz_sequence = Vec::new();
    let mut p = x;

    while p > 1 {
        // println!("{}: {}", x, p);
        if p < collatz_sequences.len().try_into().unwrap() {
            collatz_sequence.extend(collatz_sequences[(p - 1) as usize].clone());
            break;
        }
        collatz_sequence.push(p);
        p = collatz_condition(p);
    }

    return collatz_sequence;
}

pub fn run_collatz() {
    let mut collatz_sequences = Vec::new();
    for x in 1..100 {
        // let collatz_sequence = find_collatz_sequence(x);
        let collatz_sequence = find_collatz_sequence_dynamically(x, collatz_sequences.clone());
        collatz_sequences.push(collatz_sequence.clone());
        // println!("{}: {:?}\n", x, collatz_sequence);
    }
}
