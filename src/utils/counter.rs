static mut NUM_COUNTER: u64 = 0;

/// Number Counter
/// Exit if the counter is full
pub unsafe fn num_counter(total_count: u64, desc: &str) {
    if &total_count == &NUM_COUNTER {
        println!("{}", desc);
        std::process::exit(1);
    } else {
        NUM_COUNTER = NUM_COUNTER + 1;
    }
}
