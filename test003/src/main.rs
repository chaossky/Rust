struct SeaCreature {
    animal_type: String,
    name: String,
    arms: i32,
    legs:i32,
    weapon: String,
}
fn main() {
    let sc1 = SeaCreature {
        animal_type: "Whale".to_string(),
        name: "barching Whale".to_string(),
        arms: 2,
        legs: 0,
        weapon: "Whale".to_string(),
    };
    println!("{} has {} arms and {} legs. It has a {} weapon.", sc1.name, sc1.arms, sc1.legs, sc1.weapon);
    println!("{}", sc1.animal_type);
}
