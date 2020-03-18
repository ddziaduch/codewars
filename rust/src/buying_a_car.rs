fn nb_months_recursive(old: f64, new: f64, saving: i32, perc: f64, month: i32) -> (i32, i32) {
    let available = old - new + (saving * month) as f64;

    if available > 0.0 {
        (month, available.round() as i32)
    } else {
        nb_months_recursive(
            old * (1.0 - perc / 100.0),
            new * (1.0 - perc / 100.0),
            saving,
            if month % 2 == 0 { perc + 0.5 } else { perc },
            month + 1
        )

    }
}

fn nb_months(old: i32, new: i32, saving: i32, perc: f64) -> (i32, i32) {
    nb_months_recursive(old as f64, new as f64, saving, perc, 0)
}

fn testing(old: i32, new: i32, saving: i32, perc: f64, exp: (i32, i32)) -> () {
    assert_eq!(nb_months(old, new, saving, perc), exp)
}

pub fn verify() {
    testing(2000, 8000, 1000, 1.5, (6, 766));
    testing(12000, 8000, 1000, 1.5 , (0, 4000));
    testing(8000, 12000, 500, 1.0, (8, 597));
    testing(18000, 32000, 1500, 1.25, (8, 332));
    testing(7500, 32000, 300, 1.55, (25, 122));
}
