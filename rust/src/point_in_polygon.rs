// https://www.codewars.com/kata/530265044b7e23379d00076a/train/rust

// @TODO fix me in the future :-D

type Point = (f32, f32);

fn point_in_poly(poly: &[Point], point: Point) -> bool {
println!("{:?}", poly);
println!("{:?}", point);
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
            None => break
        };
        edges.push((current_point, next_point));
    }

//    println!("{:?}", edges);

    let count = edges.iter().fold(0, |acc: i32, (a, b): &(&Point, &Point)| {
        let does_intersects = ray_intersects_segment(&point, &a, &b);
        println!("acc: {:?}", acc);
        println!("point: {:?}, edge: ({:?}, {:?}), result: {:?}", point, a, b, does_intersects);
        if does_intersects {
            acc + 1
        } else {
            acc
        }
    });

    println!("{:?}", count % 2 == 0);

    count % 2 == 0
}

fn ray_intersects_segment(p: &Point, candidate_for_a: &Point, candidate_for_b: &Point) -> bool {
    let (a, b): (&Point, &Point) = if candidate_for_a.1 > candidate_for_b.1 {
        (candidate_for_b, candidate_for_a)
    } else {
        (candidate_for_b, candidate_for_a)
    };
    if p.1 < a.1 || p.1 > b.1 {
        println!("1");
        return false;
    } else if p.0 >= f32::max(a.0, b.0) {
        println!("{:?} >= {:?} = {:?}", p.0, f32::max(a.0, b.0), p.0 >= f32::max(a.0, b.0));
        println!("2");
        return false;
    } else {
        if p.0 < f32::min(a.0, b.0) {
            println!("{:?} < {:?}", p.0, f32::min(a.0, b.0));
            println!("3");
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
            println!("4");
            return true;
        } else {
            println!("5");
            return false;
        }
    }
}

pub fn verify() {
//    let poly1 = [(3.4753733, 0.051969312), (3.9030876, 1.386015), (2.8046527, 2.1531386), (1.3699969, 3.2690222), (1.4758166, 4.5434055), (-0.46345565, 4.115956), (-2.2031665, 4.3177347), (-2.9215863, 2.8945498), (-3.8659158, 2.4807734), (-3.647556, 0.99233085), (-4.316395, -0.70497954), (-3.0350296, -1.7127984), (-2.8724017, -3.1114857), (-2.3000085, -4.349689), (-0.58425885, -3.4157448), (0.36373234, -3.7423024), (1.43385, -3.0622816), (2.5702152, -2.3453069), (2.7365725, -1.8047758), (4.7745223, -1.4577917), (4.3008046, 0.26547885)];
//    assert_eq!(point_in_poly(&poly1, (1.0, 1.0)), true);
//    let poly2 = [(3.2049444, 0.29150805), (4.239688, 1.379339), (2.6314414, 2.1349516), (1.9661627, 3.80815), (0.4746115, 4.3984904), (-0.5676507, 3.792591), (-1.3150723, 2.9935586), (-3.0781221, 2.586996), (-4.031927, 2.847552), (-3.0026538, 0.5462968), (-3.9003053, -0.8104153), (-4.1608834, -2.5757208), (-2.9246836, -3.0282784), (-1.9597085, -3.7872243), (-0.3049916, -3.3599823), (0.9727091, -4.1978536), (1.4286779, -3.1409984), (2.5917325, -2.385314), (3.0967581, -1.8540423), (3.267939, -0.9816226), (3.9553068, 0.06835371)];
//    assert_eq!(point_in_poly(&poly2, (2.0, 0.0)), true);
//    let poly3 = [(3.2049444, 0.29150805), (4.239688, 1.379339), (2.6314414, 2.1349516), (1.9661627, 3.80815), (0.4746115, 4.3984904), (-0.5676507, 3.792591), (-1.3150723, 2.9935586), (-3.0781221, 2.586996), (-4.031927, 2.847552), (-3.0026538, 0.5462968), (-3.9003053, -0.8104153), (-4.1608834, -2.5757208), (-2.9246836, -3.0282784), (-1.9597085, -3.7872243), (-0.3049916, -3.3599823), (0.9727091, -4.1978536), (1.4286779, -3.1409984), (2.5917325, -2.385314), (3.0967581, -1.8540423), (3.267939, -0.9816226), (3.9553068, 0.06835371)];
//    assert_eq!(point_in_poly(&poly3, (-3.6235142, 2.55911)), false);
    let poly4 = [(4.364402, 0.6138406), (3.9723313, 1.4574578), (2.2279303, 2.3453453), (1.5992278, 2.7790885), (1.0563306, 4.4745984), (-0.6923928, 4.938121), (-1.8085438, 3.3300903), (-2.8849387, 3.3724074), (-3.0205736, 1.3045429), (-4.5655155, 0.9881144), (-3.6658928, -0.24545273), (-3.852487, -2.2719302), (-3.1415997, -2.499733), (-2.1871955, -4.354479), (-0.906585, -3.3160586), (0.14234553, -3.0850787), (1.5223219, -3.3299353), (2.32076, -2.091913), (3.3552115, -1.7117407), (3.0171533, -0.31947133), (4.118953, 0.7885876)];
    assert_eq!(point_in_poly(&poly4, (3.5143738, -0.37211952)), false);
}
