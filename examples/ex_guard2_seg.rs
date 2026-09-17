use bitflip_guard::BitFlipGuard2Seg;

fn main() {
    let mut local_value = 0;
    let mut local_value_ghost = 0;
    // If possible replace stack allocations by
    // using allocator with low susceptibility to bit flips, and
    // low electrical and geometrical coupling between the values

    let mut guard = BitFlipGuard2Seg::new(&mut local_value, &mut local_value_ghost);

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
