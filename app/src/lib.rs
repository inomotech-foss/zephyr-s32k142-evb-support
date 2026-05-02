#![no_std]

use core::mem::MaybeUninit;
use zephyr::raw::{self, ZR_GPIO_OUTPUT_ACTIVE};
use zephyr::time::{Duration, sleep};

const ADC_CHANNEL: u8 = 12;
const ADC_RESOLUTION: u8 = 12;

/// Set up ADC channel 12 (potentiometer on PTC14).
unsafe fn adc_setup(adc: &zephyr::device::adc::Adc) -> i32 {
    unsafe {
        let mut cfg: raw::adc_channel_cfg = MaybeUninit::zeroed().assume_init();
        cfg.gain = raw::adc_gain_ADC_GAIN_1;
        cfg.reference = raw::adc_reference_ADC_REF_INTERNAL;
        cfg.acquisition_time = 0; // ADC_ACQ_TIME_DEFAULT
        cfg.set_channel_id(ADC_CHANNEL);
        cfg.set_differential(0);
        adc.channel_setup(&cfg)
    }
}

/// Read a single sample from ADC channel 12.
unsafe fn adc_read_pot(adc: &zephyr::device::adc::Adc) -> i16 {
    unsafe {
        let mut buf: i16 = 0;
        let mut seq: raw::adc_sequence = MaybeUninit::zeroed().assume_init();
        seq.channels = 1u32 << ADC_CHANNEL;
        seq.buffer = &mut buf as *mut i16 as *mut core::ffi::c_void;
        seq.buffer_size = core::mem::size_of::<i16>();
        seq.resolution = ADC_RESOLUTION;

        let ret = adc.read(&seq);
        if ret != 0 {
            return -1;
        }
        buf
    }
}

#[unsafe(no_mangle)]
extern "C" fn rust_main() {
    zephyr::printk!("Hello from S32K142!\n");

    let mut gpio_token = unsafe { zephyr::device::gpio::GpioToken::get_instance().unwrap() };

    let mut red = zephyr::devicetree::aliases::led0::get_instance().unwrap();
    let mut green = zephyr::devicetree::aliases::led1::get_instance().unwrap();
    let mut blue = zephyr::devicetree::aliases::led2::get_instance().unwrap();

    unsafe {
        red.configure(&mut gpio_token, ZR_GPIO_OUTPUT_ACTIVE);
        green.configure(&mut gpio_token, ZR_GPIO_OUTPUT_ACTIVE);
        blue.configure(&mut gpio_token, ZR_GPIO_OUTPUT_ACTIVE);

        red.set(&mut gpio_token, false);
        green.set(&mut gpio_token, false);
        blue.set(&mut gpio_token, false);
    }

    // Get ADC0 device from devicetree and set up channel 12 (potentiometer)
    let adc = zephyr::devicetree::soc::adc_4003b000::get_instance()
        .expect("ADC0 already taken");
    let ret = unsafe { adc_setup(&adc) };
    if ret != 0 {
        zephyr::printk!("ADC setup failed: {}\n", ret);
    }

    // Main loop: read potentiometer and use it to control LED cycle speed
    let leds = [&mut red, &mut green, &mut blue];
    let names = ["Red", "Green", "Blue"];
    let mut i = 0;

    loop {
        // Read potentiometer (0-4095 for 12-bit)
        let pot_value = unsafe { adc_read_pot(&adc) };

        // Map pot value to delay: 50ms (fast) to 1000ms (slow)
        let delay_ms = if pot_value < 0 {
            500 // fallback if read fails
        } else {
            50 + ((pot_value as u32) * 950 / 4095)
        };

        let prev = if i == 0 { 2 } else { i - 1 };
        unsafe {
            leds[prev].set(&mut gpio_token, false);
            leds[i].set(&mut gpio_token, true);
        }
        zephyr::printk!("{} LED (pot={}, delay={}ms)\n", names[i], pot_value, delay_ms);
        i = (i + 1) % 3;

        sleep(Duration::millis_at_least(delay_ms as u64));
    }
}
