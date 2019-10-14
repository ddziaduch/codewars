// https://www.codewars.com/kata/51ba717bb08c1cd60f00002f/train/rust

fn range_extraction(numbers: &[i32]) -> String {
    let mut vector_of_numbers: Vec<i32> = numbers.to_vec();

    vector_of_numbers.sort();

    let mut ranges: Vec<Vec<&i32>> = Vec::new();
    let mut range: Vec<&i32> = Vec::new();

    for (index, number) in vector_of_numbers.iter().enumerate() {
        range.push(number);
        let next_number = vector_of_numbers.get(index + 1);
        if next_number.is_none() || next_number.is_some() && next_number.unwrap() - *number != 1  {
            ranges.push(range);
            range = Vec::new();
        }
    }

    ranges.iter().map(|range| {
        match range.len() {
            0 => String::new(),
            1 => format!("{}", range.first().unwrap()),
            2 => format!("{},{}", range.first().unwrap(), range.last().unwrap()),
            _ => format!("{}-{}", range.first().unwrap(), range.last().unwrap())
        }
    }).collect::<Vec<String>>().join(",")
}

pub fn verify() {
    assert_eq!("-6,-3-1,3-5,7-11,14,15,17-20", range_extraction(&[-6,-3,-2,-1,0,1,3,4,5,7,8,9,10,
        11,14,15,17,18,19,20]));
    assert_eq!("-3--1,2,10,15,16,18-20", range_extraction(&[-3,-2,-1,2,10,15,16,18,19,20]));
}