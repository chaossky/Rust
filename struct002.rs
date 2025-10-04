struct SeaCreature {
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}

fn main() {
    //seacrature 데이터는 스택에 있다.
    let feris=SeaCreature{
        //String struct는 stack에 있으나,
        //heap에 있는 데이터에 대한 참조를 가지고 있다.
        animal_type: String::from("게"),
        name: String::from("Ferris"),
        arms:2,
        legs:4,
        weapon:String::from("집게"),
    };

    let sarah=SeaCreature{  
        animal_type:String::from("문어"),
        name:String::from("Sarah"),
        arms:8,
        legs:0,
        weapon:String::from("먹물"),
    };

    println!("{}는 {}이며 {}개의 팔과 {}개의 다리를 가지고 있으며, {}가 무기이다.",
    feris.name,feris.animal_type,feris.arms,feris.legs,feris.weapon
);
    println!("{}는 {}이며 {}개의 팔과 {}개의 다리를 가지고 있으며, {}이 무기이다.",
    sarah.name,sarah.animal_type,sarah.arms,sarah.legs,sarah.weapon
);
}
