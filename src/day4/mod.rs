use std::{collections::HashMap, fs, vec};


pub fn solution(path_to_input: String) {
    let input = fs::read_to_string(path_to_input).expect("Failed to read input");

    let mut part1 = 0;
    let mut part2 = 0;

    let mat: Vec<Vec<char>> = input
        .split("\n")
        .map(|x| x.chars().collect())
        .filter(|y: &Vec<char>| y.len() != 0)
        .collect();

    ////////////
    // PART 1 //
    ////////////
    
    // All directions should be searched for the first time
    let offsets: Vec<(i32, i32)> = vec![
        // Top Row
        (-1, -1),
        (-1,  0),
        (-1,  1),
        // Middle Row
        ( 0, -1),
        ( 0,  1),
        // Bottom Row
        ( 1, -1),
        ( 1,  0),
        ( 1,  1),
    ];

    let mut row = 0;
    let mut col;
    while row < mat.len() {
        col = 0;
        while col < mat[row].len() {
            part1 += find_xmas(mat.clone(), (row, col), offsets.clone(), 0);
            col += 1;
        }
        row += 1;
    }

    ////////////
    // Part 2 //
    ////////////
    
    let mut center_coords: Vec<Vec<(usize, usize, bool)>> = Vec::new();

    row = 0;
    while row < mat.len() {
        col = 0;
        while col < mat[row].len() {
            center_coords.push(find_x_mas(mat.clone(), (row, col), offsets.clone(), 0));
            col += 1;
        }
        row += 1;
    }

    let coords: Vec<(usize, usize, bool)> = center_coords.iter().filter(|x| x.len() != 0).flatten().map(|x| *x ).collect();
    let mut coord_table: HashMap<(usize, usize, bool), usize> = HashMap::new();
    for ent in coords {
        coord_table.entry(ent).and_modify(|i| *i += 1).or_insert(1);
    }

    for ((_,_,diag), num) in coord_table {
        if diag {
            part2 += num / 2;
        }
    }
    
    println!("Part 1: {:?}\nPart 2: {:?}", part1, part2);
}

// Check if the surrounding tiles contain the passed char
// Variables represent positions as follows:
//   a b c      (-1, -1) (-1, 0) (-1, +1)
//   d * e      ( 0, -1) ( 0, 0) ( 0, +1)
//   f g h      (+1, -1) (+1, 0) (+1, +1)
fn check_surrounding(matrix: Vec<Vec<char>>, pos: (i32, i32), directions: Vec<(i32, i32)>, digit: char) -> Vec<(usize, usize)> {
    
    let check_locations: Vec<(i32, i32)> = directions.iter().map(|x| (pos.0 + x.0, pos.1 + x.1)).collect();

    let mut valid: Vec<(usize, usize)> = Vec::new();
    for point in check_locations {
        // Bounds checking
        if point.0 < 0
            || (point.0 as usize) > (matrix.len() - 1)
            || point.1 < 0
            || (point.1 as usize) > (matrix[point.0 as usize].len() - 1)
        {
            continue;
        }

        if matrix[point.0 as usize][point.1 as usize] == digit {
            valid.push((point.0 as usize, point.1 as usize));
        }
    }

    return valid;
}


// Find any XMAS that can be spelled out from the current position in matrix
fn find_xmas(matrix: Vec<Vec<char>>, pos: (usize, usize), directions: Vec<(i32, i32)>, depth: usize) -> u32 {
    let digits: Vec<char> = vec!['X', 'M', 'A', 'S'];
    let mut sum = 0;

    // BASE CASE
    if matrix[pos.0][pos.1] == *digits.last().unwrap() && depth == digits.len() - 1 {
        return 1;
    }

    // RECURSIVE CASE
    if matrix[pos.0][pos.1] == digits[depth] {
        // Search all surrounding slots for the next digit and recurse for all hits
        let other_coords = check_surrounding(matrix.clone(), (pos.0 as i32, pos.1 as i32), directions, digits[depth + 1]);
        for coord in other_coords {
            let new_directions = vec![(coord.0 as i32 - pos.0 as i32, coord.1 as i32 - pos.1 as i32)];
            sum += find_xmas(matrix.clone(), coord, new_directions, depth + 1);
        }
    }

    sum

}

// Find any two MAS that form an X in matrix
// Returns the center coordinates of all mas with a bool, true is a diagonal while false is horizontal/vertical
fn find_x_mas(matrix: Vec<Vec<char>>, pos: (usize, usize), directions: Vec<(i32, i32)>, depth: usize) -> Vec<(usize, usize, bool)> {
    let digits: Vec<char> = vec!['M', 'A', 'S'];
    let mut a_locs: Vec<(usize, usize, bool)> = Vec::new();
    let mut diag = false;

    // BASE CASE
    if matrix[pos.0][pos.1] == digits[1] && depth == digits.len() - 2 {
        let coords = vec![((pos.0 as i32 + directions[0].0) as usize, (pos.1 as i32 + directions[0].1) as usize)];
        if check_surrounding(matrix.clone(), (pos.0 as i32, pos.1 as i32), directions.clone(), *digits.last().unwrap()) == coords {
            if directions[0].0 != 0 && directions[0].1 != 0 {
                diag = true;
            }
            return vec![(pos.0, pos.1, diag)];
        }
    }

    // RECURSIVE CASE
    if matrix[pos.0][pos.1] == digits[depth] {
        // Search all surrounding slots for the next digit and recurse for all hits
        let other_coords = check_surrounding(matrix.clone(), (pos.0 as i32, pos.1 as i32), directions, digits[depth + 1]);
        for coord in other_coords {
            let new_directions = vec![(coord.0 as i32 - pos.0 as i32, coord.1 as i32 - pos.1 as i32)];
            for center in find_x_mas(matrix.clone(), coord, new_directions, depth + 1) {
                a_locs.push(center);
            }
        }
    }

    a_locs

}
