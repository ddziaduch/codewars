// https://www.codewars.com/kata/556deca17c58da83c00002db/train/rust

fn tribonacci(signature: &[f64; 3], n: usize) -> Vec<f64> {
    let mut vector = new_vector_from_signature(signature, n);

    append_tail_to_vector(n, &mut vector);

    vector
}

fn append_tail_to_vector(n: usize, vector: &mut Vec<f64>) {
    for i in 3..n {
        let first = vector.get(i - 3).unwrap();
        let second = vector.get(i - 2).unwrap();
        let third = vector.get(i - 1).unwrap();
        let fourth = first + second + third;
        vector.push(fourth)
    }
}

fn new_vector_from_signature(signature: &[f64; 3], n: usize) -> Vec<f64> {
    let mut vector = Vec::<f64>::new();
    let number_of_elements_to_push = if n > 3 { 3 } else { n };

    for i in 0..number_of_elements_to_push {
        vector.push(signature[i]);
    }

    vector
}

pub fn verify() {
    assert_eq!(tribonacci(&[300., 200., 100.], 0), vec![]);
    assert_eq!(tribonacci(&[1., 1., 1.], 1), vec![1.]);
    assert_eq!(tribonacci(&[1., 1., 1.], 2), vec![1., 1.]);
    assert_eq!(tribonacci(&[1., 1., 1.], 3), vec![1., 1., 1.]);
    assert_eq!(tribonacci(&[0., 1., 1.], 10), vec![0., 1., 1., 2., 4., 7., 13., 24., 44., 81.]);
    assert_eq!(tribonacci(&[1., 0., 0.], 10), vec![1., 0., 0., 1., 1., 2., 4., 7., 13., 24.]);
    assert_eq!(tribonacci(&[0., 0., 0.], 10), vec![0., 0., 0., 0., 0., 0., 0., 0., 0., 0.]);
    assert_eq!(tribonacci(&[1., 2., 3.], 10), vec![1., 2., 3., 6., 11., 20., 37., 68., 125., 230.]);
    assert_eq!(tribonacci(&[3., 2., 1.], 10), vec![3., 2., 1., 6., 9., 16., 31., 56., 103., 190.]);
    assert_eq!(tribonacci(&[0.5, 0.5, 0.5], 30), vec![0.5, 0.5, 0.5, 1.5, 2.5, 4.5, 8.5, 15.5, 28.5, 52.5, 96.5, 177.5, 326.5, 600.5, 1104.5, 2031.5, 3736.5, 6872.5, 12640.5, 23249.5, 42762.5, 78652.5, 144664.5, 266079.5, 489396.5, 900140.5, 1655616.5, 3045153.5, 5600910.5, 10301680.5]);;
}