// https://www.codewars.com/kata/5d23d89906f92a00267bb83d/train/rust

fn get_order(input: String ) -> String {
    let menu = vec![
        "burger",
        "fries",
        "chicken",
        "pizza",
        "sandwich",
        "onionrings",
        "milkshake",
        "coke"
    ];
    let mut order = Vec::new();
    for item in menu {
        let x: Vec<_> = input.match_indices(item).collect();
        for y in x {
            order.push(y.1);
        }
    }
    let mut result = String::new();
    // for item in order {
        // result.push_str()
    // }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!( get_order( "milkshakepizzachickenfriescokeburgerpizzasandwichmilkshakepizza".to_string() ),
                    "Burger Fries Chicken Pizza Pizza Pizza Sandwich Milkshake Milkshake Coke".to_string() );
    }

    #[test]
    fn test_2() {
        assert_eq!( get_order( "pizzachickenfriesburgercokemilkshakefriessandwich".to_string() ),
                    "Burger Fries Fries Chicken Pizza Sandwich Milkshake Coke".to_string() );
    }
}

fn main() {
    println!("Hello, world!");
}
