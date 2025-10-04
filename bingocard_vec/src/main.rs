use rand::seq::SliceRandom;
fn main() {
    // mutable 벡터 생성
    // mutable은 변경 가능한 벡터
    // Rust에서는 벡터의 크기를 동적으로 변경할 수 있습니다.
    // 벡터는 동일한 타입의 값들을 저장할 수 있는 동적 배열
    let mut nums=vec![];
    for i in 1..75{
        nums.push(i);
    }

    // print!("{:?}",nums);
    // println!();

    let mut rng=rand::thread_rng();
    
    nums.shuffle(&mut rng);

      print!("{:?}",nums);
    println!();

    for i in 0..25
    {
        if i==12
        {
            print!("  *,");//와일드카드
        }
        else
        {
            print!("{:3},",nums[i]);
    
        if i%5==4{
        println!("");
        }}
    }
}

