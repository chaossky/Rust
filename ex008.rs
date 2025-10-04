struct SeaCreature{
    animal_type:String,
    name:String,
    arms:i32,
    legs:i32,
    weapon:String,
}

fn main(){
    let fish=SeaCreature{
        animal_type:"fish".to_string(),
        name:"Ovon".to_string(),
        arms:0,
        legs:0,
        weapon:"poison".to_string(),
    };

    let squid=SeaCreature;
    squid.animal_type="squid".to_string();
    squid.name="squid".to_string();
    squid.arms=0;
    squid.legs=0;
    squid.weapon="poison".to_string();

    println!("{}",fish.animal_type);
    println!("{}",fish.name);
    println!("{}",fish.arms);
    println!("{}",fish.legs);
    println!("{}",fish.weapon);

    println!("{}",squid.animal_type);
    println!("{}",squid.name);
    println!("{}",squid.arms);
    println!("{}",squid.legs);
    println!("{}",squid.weapon);

    }
    