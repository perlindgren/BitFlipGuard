use bitflip_guard::BitFlipGuard2;

fn main() {
    let mut guard = BitFlipGuard2::new(42i32);
    guard.set(100);

    println!("guard {:?}", guard);

    if let Some(value) = guard.get() {
        println!("val {:?}", value);
        guard.set(100 + value);
    }

    println!("guard {:?}", guard.get());

    guard.mock_all_one_v();
    println!("mock one, guard {:?}", guard);

    println!("val {:?}", guard.get());
}
