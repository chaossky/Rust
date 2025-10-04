#![allow(dead_code)]

enum Species{
    Crab,
    Octopus,
    Fish,
    Clam,
}

struct SeaCreature{
    species:Species,
    name:String,
    arms:i32,
    legs:i32,
    weapon:String,
}

fn main(){
    let ferris=SeaCreature{
        species:Species::Crab,
        name:String::from("Ferris"),
        arms:2,
        legs:4,
        weapon:String::from("집게"),
    };

    match ferris.species{
        Species::Crab=>println!("{} 는 게 이다.",ferris.name),
        Species::Octopus=>println!("{} 는 문어 이다.",ferris.name),
        Species::Fish=>println!("{} 는 물고기 이다.",ferris.name),
        Species::Clam=>println!("{} 는 조개 이다.",ferris.name),
    }



}
