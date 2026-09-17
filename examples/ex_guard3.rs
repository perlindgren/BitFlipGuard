use bitflip_guard::BitFlipGuard3;

fn main() {
    let mut guard = BitFlipGuard3::new(42i32);

    println!("guard {:?}", guard);

    if let Some(value) = guard.get() {
        println!("val {:?}", value);
        guard.set(100 + value);
    }

    println!("guard {:?}", guard);

    println!("{:?}", guard.get());

    guard.mock_all_one_v();
    println!("mock one, guard {:?}", guard);

    println!("{:?}", guard.get());
    println!("{:?}", guard.get_majority());
}
