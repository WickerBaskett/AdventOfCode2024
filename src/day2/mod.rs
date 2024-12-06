use std::fs;

pub fn solution(path_to_input: String) {
    let input = fs::read_to_string(path_to_input)
        .expect("Failed to read input");
    
    let mut part1 = 0;
    let mut part2 = 0;
    
    let _ = input.split('\n')
    .map(|line| {
        let values: Vec<u32> = line.split(' ')
        .filter(|x| *x != "")
        .map(|x| {
            x.parse::<u32>().expect("Failed to parse u32")
        })
        .collect::<Vec<u32>>();

        return values;
    })
    .for_each(|data| {
        if data.is_empty() {
            return;
        }
                
        ////////////
        // PART 1 //
        ////////////
        
        if check_safety(&data) {
            part1 += 1;
        }

        ////////////
        // PART 2 //
        ////////////
        let mut curr_data = data.clone();
        let mut index = 0;

        while index <= data.len()
        {
            if check_safety(&curr_data) {
                part2 += 1;
                break;
            }

            curr_data = data.clone();
            if index < data.len() {
                curr_data.remove(index);
            }
            index += 1;
        }
    });

    println!("Part 1: {:?}\nPart 2: {:?}", part1, part2);

}

// Checks if the passed report is safe
fn check_safety(data: &Vec<u32>) -> bool {
    if data.is_sorted() || data.is_sorted_by(|a, b| a >= b) {
        let mut safe = 0;

        for pair in data.windows(2){ 
            if pair[0].abs_diff(pair[1]) > 3 || pair[0].abs_diff(pair[1]) < 1 {
                safe += 1
            }
        };

        if safe == 0 {
            return true;
        }
    }

    false
}
