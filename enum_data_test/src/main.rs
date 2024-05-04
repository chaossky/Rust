#![allow(dead_code)]//컴파일러 경고를 방지해준다.
enum Species{
    Crab,
    Octopus,
    Fish,
    Clam,
}
enum PoisonType {
    Acidic,
    Painful,
    Lethal,
}
enum Size {
    Big,
    Small,
}

enum Weapon {
    Claw(i32, Size),
    Poison(PoisonType),
    None,
}

struct SeaCreature {
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon: Weapon,
}
fn main() {
    let ferris=SeaCreature{
        species: Species::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: Weapon::Claw(2, Size::Small),
    };

    match ferris.species {
        Species::Crab => match ferris.weapon{
            Weapon::Claw(num_claws,size) => {
                let size_description=match size{
                    Size::Big => "큰",
                    Size::Small => "작은",
                };
                println!(
                    "ferris는 {} 개의 {} 집게를 가진 게이다.",
                    num_claws,
                    size_description
                )
            }
            _=> println!("ferris는 다른 무기를 가지는 게이다."),
        },
        _=> println!("ferris는 다른 동물이다."),
    }
}