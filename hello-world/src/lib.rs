
#![no_std] // Opt out of std library
#![no_main] // No main function in kernel mode

use core::panic::PanicInfo;

#[panic_handler]
const fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Entry point for the driver binary (DriverEntry)
use wdk_sys::{NTSTATUS, PCUNICODE_STRING, PDRIVER_OBJECT, STATUS_SUCCESS};

// Does same as #[no_mangle] but also sets the name of the symbol to DriverEntry
#[export_name = "DriverEntry"]
pub unsafe extern "system" fn driver_entry(
    driver_object: PDRIVER_OBJECT,
    registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
    wdk::println!("Hello, world!");

    STATUS_SUCCESS
}