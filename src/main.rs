#![no_std]
#![no_main]

// The macro for our start-up function
use rp_pico::entry;

// GPIO traits
use embedded_hal::digital::v2::OutputPin;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Pull in any important traits
use rp_pico::hal::prelude::*;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::hal;

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals, then blinks the LED in an
/// infinite loop.
#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
        .ok()
        .unwrap();

    // The delay object lets us wait for specified amounts of time (in
    // milliseconds)
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Set the LED to be an output
    let mut led_pin = pins.led.into_push_pull_output();
    let mut pin_gp_15 = pins.gpio15.into_push_pull_output();

    // let mut pin_gp_a = pins.gpio1.into_push_pull_output();
    // let mut pin_gp_b = pins.gpio2.into_push_pull_output();
    // let mut pin_gp_c = pins.gpio3.into_push_pull_output();
    // let mut pin_gp_d = pins.gpio4.into_push_pull_output();
    // let mut pin_gp_e = pins.gpio5.into_push_pull_output();
    // let mut pin_gp_f = pins.gpio6.into_push_pull_output();

    let mut pin_gp_g = pins.gpio7.into_push_pull_output();
    let mut pin_gp_dot = pins.gpio8.into_push_pull_output();

    let mut pin1 = pins.gpio1.into_push_pull_output();
    let mut pin2 = pins.gpio2.into_push_pull_output();
    let mut pin3 = pins.gpio3.into_push_pull_output();
    let mut pin4 = pins.gpio4.into_push_pull_output();
    let mut pin5 = pins.gpio5.into_push_pull_output();
    let mut pin6 = pins.gpio6.into_push_pull_output();



    pin_gp_dot.set_high().unwrap();
    pin_gp_g.set_high().unwrap();
    loop {

        pin1.set_low().unwrap();
        delay.delay_ms(100);
        pin1.set_high().unwrap();

        pin2.set_low().unwrap();
        delay.delay_ms(100);
        pin2.set_high().unwrap();

        pin3.set_low().unwrap();
        delay.delay_ms(100);
        pin3.set_high().unwrap();

        pin4.set_low().unwrap();
        delay.delay_ms(100);
        pin4.set_high().unwrap();

        pin5.set_low().unwrap();
        delay.delay_ms(100);
        pin5.set_high().unwrap();

        pin6.set_low().unwrap();
        delay.delay_ms(100);
        pin6.set_high().unwrap();
    }
}


// End of file
