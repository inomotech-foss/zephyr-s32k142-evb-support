#![no_std]

use zephyr as _;

#[unsafe(no_mangle)]
extern "C" fn rust_main() {
    zephyr::printk!("Hello from S32K142!\n");
}
