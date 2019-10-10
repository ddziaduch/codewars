// https://www.codewars.com/kata/51ba717bb08c1cd60f00002f/train/rust

fn range_extraction(numbers: &[i32]) -> String {
    let mut vector_of_numbers: Vec<i32> = numbers.to_vec();

    vector_of_numbers.sort();

    let mut ranges: Vec<Vec<&i32>> = Vec::new();
    let mut current_range: Vec<&i32> = Vec::new();

    for (index, number) in vector_of_numbers.iter().enumerate() {
        if index == 0 {
            current_range.push(number);
            ranges.push(current_range);
            current_range = Vec::new();
        } else {
            let prev_number = vector_of_numbers.get(index - 1);
            if prev_number.is_some() && prev_number.unwrap() + 1 == *number && ranges.last().is_some() {
                current_range.push(number);
            } else {
                current_range.push(number);
                ranges.push(current_range);
                current_range = Vec::new();
            }
        }
    }

    println!("{:?}", ranges);

    String::new()
}

pub fn verify() {
    assert_eq!("-6,-3-1,3-5,7-11,14,15,17-20", range_extraction(&[-6,-3,-2,-1,0,1,3,4,5,7,8,9,10,11,14,15,17,18,19,20]));
    assert_eq!("-3--1,2,10,15,16,18-20", range_extraction(&[-3,-2,-1,2,10,15,16,18,19,20]));
}