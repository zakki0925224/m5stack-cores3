pub fn delay_ms(ms: u32) {
    for _ in 0..(ms * 50_000) {
        core::hint::black_box(());
    }
}
