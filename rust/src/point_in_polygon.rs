// https://www.codewars.com/kata/530265044b7e23379d00076a/train/rust

type Point = (f32, f32);

fn point_in_poly(poly: &[Point], point: Point) -> bool {
    let max_x: f32 = poly.iter().map(|point| point.0).fold(0. / 0., f32::max );
    let max_y: f32 = poly.iter().map(|point| point.1).fold(0. / 0., f32::max );
    let min_x: f32 = poly.iter().map(|point| point.0).fold(0. / 0., f32::min );
    let min_y: f32 = poly.iter().map(|point| point.1).fold(0. / 0., f32::min );

    if point.0 > max_x || point.0 < min_x || point.1 > max_y || point.1 < min_y {
        return false;
    }

    let mut edges = Vec::new();

    for (index, current_point) in poly.iter().enumerate() {
        let next_point = match poly.get(index + 1) {
            Some(next_point) => next_point,
            None => match poly.first() {
                Some(first_point) => first_point,
                None => break
            }
        };
        edges.push((current_point, next_point));
    }

//    println!("{:?}", edges);

    let count = edges.iter().fold(0, |acc: i32, (a, b): &(&Point, &Point)| {
        let does_intersects = ray_intersects_segment(&point, &a, &b);
        println!("point: {:?}, edge: ({:?}, {:?}), result: {:?}", point, a, b, does_intersects);
        if does_intersects {
            acc + 1
        } else {
            acc
        }
    });

    count % 2 == 0
}

fn ray_intersects_segment(p: &Point, candidate_for_a: &Point, candidate_for_b: &Point) -> bool {
    let (a, b): (&Point, &Point) = if candidate_for_a.1 > candidate_for_b.1 {
        (candidate_for_b, candidate_for_a)
    } else {
        (candidate_for_b, candidate_for_a)
    };
    if p.1 < a.1 || p.1 > b.1 {
//        println!("1");
        return false;
    }
    if p.0 >= f32::max(a.0, b.0) {
//        println!("{:?} >= {:?} = {:?}", p.0, f32::max(a.0, b.0), p.0 >= f32::max(a.0, b.0));
//        println!("2");
        return false;
    }
    if p.0 < f32::min(a.0, b.0) {
//        println!("3");
        return true;
    }
    let m_red: f32 = if a.0 != b.0 {
        (b.1 - a.1) / (b.0 - a.0)
    } else {
        std::f32::MAX
    };
    let m_blue: f32 = if a.0 != p.0 {
        (p.1 - a.1) / (p.0 / a.0)
    } else {
        std::f32::MAX
    };
    if m_blue >= m_red {
//        println!("4");
        return true;
    } else {
//        println!("5");
        return false;
    }
}

pub fn verify() {
    let poly = [(-5., -5.), (5., -5.),
        (5., 5.), (-5., 5.)];
    assert_eq!(point_in_poly(&poly, (-6., 0.)), false);
    assert_eq!(point_in_poly(&poly, (-1., 1.)), true);
}
