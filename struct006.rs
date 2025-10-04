// 사용되지 않는 코드(dead code)에 대한 컴파일러 경고를 무시함
#![allow(dead_code)]

// 바다 생물의 종류를 나타내는 열거형(enum)
enum Species {
    Crab,     // 게
    Octopus,  // 문어
    Fish,     // 물고기
    Clam,     // 조개
}

// 독의 종류를 나타내는 열거형
enum PoisonType {
    Acidic,   // 산성 독
    Painful,  // 고통을 유발하는 독
    Lethal,   // 치명적인 독
}

// 무기의 크기를 나타내는 열거형
enum Size {
    Big,      // 큰
    Small,    // 작은
}

// 바다 생물이 사용할 수 있는 무기를 나타내는 열거형
enum Weapon {
    Claw(i32, Size),       // 집게: 개수와 크기
    Poison(PoisonType),    // 독: 독의 종류
    None,                  // 무기가 없음
}

// 바다 생물 구조체 정의
struct SeaCreature {
    species: Species,      // 생물의 종류
    name: String,          // 이름
    arms: i32,             // 팔 개수
    legs: i32,             // 다리 개수
    weapon: Weapon,        // 무기
}

// 프로그램의 진입점
fn main() {
    // SeaCreature 인스턴스 생성: Ferris라는 이름의 게
    let ferris = SeaCreature {
        species: Species::Crab,               // 게
        name: String::from("Ferris"),         // 이름
        arms: 2,                              // 팔 2개
        legs: 4,                              // 다리 4개
        weapon: Weapon::Claw(2, Size::Small), // 작은 집게 2개
    };

    // ferris의 종(species)에 따라 분기
    match ferris.species {
        Species::Crab => match ferris.weapon {
            // 무기가 Claw일 경우, 개수와 크기를 추출
            Weapon::Claw(num_claws, size) => {
                // 크기에 따라 설명 문자열 선택
                let size_description = match size {
                    Size::Big => " 큰 ",
                    Size::Small => " 작은 ",
                };
                // 최종 출력
                println!(
                    "ferris는 {} 개의 {} 집게를 가진 게이다.",
                    num_claws, size_description
                );
            }
            // Crab이지만 Claw가 아닌 다른 무기를 가진 경우
            _ => println!("ferris는 다른 무기를 가진 게이다."),
        },
        // Crab이 아닌 다른 종일 경우
        _ => println!("ferriss는 다른 동물이다."),
    }
}
/*
💡 요약
이 코드는 SeaCreature라는 구조체를 정의하고, 
enum을 활용해 다양한 속성(종, 무기, 크기 등)을 표현합니다.

match 문을 통해 ferris의 종과 무기를 조건에 따라 출력합니다.

Rust의 열거형과 패턴 매칭을 활용한 구조적이고 안전한 방식의 조건 분기 예제입니다.
*/