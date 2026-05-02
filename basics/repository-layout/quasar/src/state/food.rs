/// A carnival food stand.
pub struct FoodStand {
    pub name: &'static str,
    pub food_type: &'static str,
    pub tickets: u32,
}

impl FoodStand {
    pub fn name_matches(&self, other: &str) -> bool {
        self.name.as_bytes() == other.as_bytes()
    }
}

/// Static list of food stands.
pub fn get_food_stands() -> &'static [FoodStand] {
    &[
        FoodStand { name: "Larry's Pizza", food_type: "pizza", tickets: 3 },
        FoodStand { name: "Taco Shack", food_type: "taco", tickets: 2 },
        FoodStand { name: "Dough Boy's", food_type: "fried dough", tickets: 1 },
    ]
}
