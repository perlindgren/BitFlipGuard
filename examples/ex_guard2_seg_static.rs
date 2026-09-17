// #![no_std]
use bitflip_guard::BitFlipGuard2Seg;

// Replace to other section with low susceptibility to bit flips
#[unsafe(link_section = ".data")]
static mut LOCAL_VALUE: i32 = 0;

// Replace to other section, with low susceptibility to bit flips and
// low electrical and geometrical coupling to LOCAL VALUE

#[unsafe(link_section = ".data")]
static mut LOCAL_VALUE_GHOST: i32 = 0;

fn main() {
    #[allow(static_mut_refs)]
    let mut guard = unsafe { BitFlipGuard2Seg::new(&mut LOCAL_VALUE, &mut LOCAL_VALUE_GHOST) };

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
