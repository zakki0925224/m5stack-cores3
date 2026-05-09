use core::panic::PanicInfo;
use esp_println::println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "panicked at {}:{}:{}:",
            location.file(),
            location.line(),
            location.column()
        );
    } else {
        println!("panicked at ?:?:?:");
    }

    println!("{:?}", info.message());

    loop {}
}
