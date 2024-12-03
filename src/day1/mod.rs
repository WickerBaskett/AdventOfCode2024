use std::{collections::HashMap, fs, path::absolute};

pub fn solution(path_to_input: String) {
    let input = fs::read_to_string(path_to_input)
        .expect("Failed to read input");
    

    let mut llist: Vec<i32> = vec![];
    let mut rlist: Vec<i32> = vec![];

    let _ = input.split(&[' ', '\n'])
    .filter(|x| *x != "")
    .enumerate()
    .for_each(|x| {
        if (x.0 % 2) == 0 {
            llist.push(x.1.parse().expect("Failed to parse value for Left List"));
        }
        else {
            rlist.push(x.1.parse().expect("Failed to parse value for Right List"));
        }
    });


    llist.sort();
    rlist.sort();


    let mut diff_sum = 0;
    llist.iter().enumerate().for_each(|x| {
        diff_sum += (x.1 - rlist[x.0]).abs();
    });

    let mut num_occur: HashMap<i32, u32> = HashMap::new();
    rlist.iter().for_each(|x| {
        num_occur.entry(*x).and_modify(|i| *i += 1).or_insert(1);
    });

    let mut sum: u32 = 0;
    llist.iter().for_each(|x| {
        sum += *x as u32 * num_occur.get(x).or(Some(&0)).unwrap();
    });

    println!("Part 1: {:?}\nPart 2: {:?}", diff_sum, sum);
}

