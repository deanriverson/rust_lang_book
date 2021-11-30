#[cfg(test)]
mod tests {
    use crate::eat_at_restaurant;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn eat_at_restaurant_can_be_called() {
        eat_at_restaurant();
        assert_eq!(2 + 2, 4);
    }
}

mod front_of_house;

use std::collections::HashMap;

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("one", 1);

    println!("map contains 'one'? {}", map.contains_key("one"));
}
