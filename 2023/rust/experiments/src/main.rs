mod popcnt;

#[cfg(feature = "heapprofile")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

pub fn main() {
    #[cfg(feature = "heapprofile")]
    let _profiler = dhat::Profiler::new_heap();

    println!("Hello experiments!");
    let popcnt1 = popcnt::popcnt(0b1010);
    println!("popcnt(1010) = {:?}", popcnt1);
}
