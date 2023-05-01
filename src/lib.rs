// use std::{thread, time};
use std::collections::HashMap;

fn collatz_condition(x: i32) -> i32 {
    // let one_nanos = time::Duration::from_nanos(1);
    // thread::sleep(one_nanos);
    if x % 2 == 0 {
        return x / 2
    } else {
        return 3 * x + 1
    }
}

// ------------------------------
// Dynamic
// ------------------------------
// fn expand_sequence(p: i32, collatz_sequences: &HashMap<i32,Vec<i32>>) -> Vec<i32> {
//     if p == 1 {
//         return vec![2, 1];
//     }

//     if collatz_sequences.contains_key(&p) {
//         return collatz_sequences.get(&p).unwrap().to_vec();
//     }

//     let mut q = vec![p];
//     q.extend(expand_sequence(collatz_condition(p), collatz_sequences));
//     // let one_nanos = time::Duration::from_nanos(1);
//     // thread::sleep(one_nanos);
//     // println!("{}: {:?}", p, q);

//     return q;
// }


// pub fn run_collatz() {
//     let mut collatz_sequences: HashMap<i32,Vec<i32>> = HashMap::new();
//     for x in 1..100 {
//         let mut collatz_sequence = Vec::new();
//         let mut p = x;
    
//         while p > 1 && !collatz_sequences.contains_key(&p) {
//             let tmp_p = p;
//             collatz_sequence.push(p);
//             p = collatz_condition(p);
//             collatz_sequences.insert(tmp_p, expand_sequence(p, &collatz_sequences));
//         }
//     }

//     // println!("Collatz Sequences: {:?}", collatz_sequences);
//     // for i in 1..(collatz_sequences.len() as i32 + 1) {
//     //     let mut m: Vec<i32> = vec![];
//     //     if collatz_sequences.contains_key(&i) {
//     //         m = collatz_sequences.get(&i).unwrap().to_vec();
//     //     }
//     //     println!("{}: {:?}", i, m);
//     // }
// }

// ------------------------------
// Basic
// ------------------------------
fn expand_sequence(p: i32, collatz_sequences: &HashMap<i32,Vec<i32>>) -> Vec<i32> {
    if p == 1 {
        return vec![1];
    }

    let mut q = vec![p];
    q.extend(expand_sequence(collatz_condition(p), collatz_sequences));
    // let one_nanos = time::Duration::from_nanos(1);
    // thread::sleep(one_nanos);
    // println!("{}: {:?}", p, q);

    return q;
}


pub fn run_collatz() {
    let mut collatz_sequences: HashMap<i32,Vec<i32>> = HashMap::new();
    for x in 1..100 {
        let mut collatz_sequence = Vec::new();
        let mut p = x;
    
        while p > 1 {
            let tmp_p = p;
            collatz_sequence.push(p);
            p = collatz_condition(p);
            collatz_sequences.insert(tmp_p, expand_sequence(p, &collatz_sequences));
        }
    }

    // println!("Collatz Sequences: {:?}", collatz_sequences);
    // for i in 1..(collatz_sequences.len() as i32 + 1) {
    //     let mut m: Vec<i32> = vec![];
    //     if collatz_sequences.contains_key(&i) {
    //         m = collatz_sequences.get(&i).unwrap().to_vec();
    //     }
    //     println!("{}: {:?}", i, m);
    // }
}