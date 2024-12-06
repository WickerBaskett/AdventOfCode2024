use std::fs;
use regex::Regex;

pub fn solution(path_to_input: String) {
    let input = fs::read_to_string(path_to_input)
        .expect("Failed to read input");

    let mut part1: u32 = 0;
    let mut part2: u32 = 0;

    ////////////
    // PART 1 //
    ////////////

    let re1 = Regex::new(r"mul\((\d{1,3},\d{1,3}\))").unwrap();
    let _ = re1.find_iter(&input)
    .map(|x| {

        let pair: Vec<&str> = x.as_str()
            .split(['(', ',', ')'])
            .filter(|inp| *inp != "mul" && *inp != "")
            .collect();

        return (pair[0].parse::<u32>().unwrap(), pair[1].parse::<u32>().unwrap())
    })
    .for_each(|mult| {
        part1 += mult.0 * mult.1;
    });

    ////////////
    // PART 2 //
    ////////////
    
    let mut doing: bool = true; // Tracks if we are in a do() or don't() section

    let re2 = Regex::new(r"mul\((\d{1,3},\d{1,3}\))|do\(\)|don't\(\)").unwrap();
    let _ = re2.find_iter(&input)
    .map(|x| x.as_str())
    .for_each(|x| match x {
        "do()" => { doing = true }
        "don't()" => { doing = false }
        n @ _ => {
            let pair: Vec<u32> = n.split(['(', ',', ')'])
            .filter(|inp| *inp != "mul" && *inp != "")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
            
            if doing {
                part2 += pair[0] * pair[1];
            }
        }
    });



    println!("Part 1: {:?}\nPart 2: {:?}", part1, part2);
}
