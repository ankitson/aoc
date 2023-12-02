mod popcnt;

#[cfg(feature = "heapprofile")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

pub fn main() {
    #[cfg(feature = "heapprofile")]
    let _profiler = dhat::Profiler::new_heap();

    println!("Hello experiments!");
    // let popcnt1 = popcnt::popcnt_naive(0b10101010);
    let popcnt1 = 0;
    let popcnt2 = popcnt::popcnt_split(0b10101010);
    println!("popcnt(10101010) = {:?} = {:?}", popcnt1, popcnt2);
}
