#![no_std]
#![no_main]

use core::fmt::Write;
use core::writeln;

use hal::adc::Adc;
use hal::gpio::{Input, Level, Output, OutputDrive, Pull};
use hal::uart::UartTx;
use {ch57x_hal as hal, panic_halt as _};

#[qingke_rt::entry]
fn main() -> ! {
    let mut config = hal::Config::default();
    config.clock.use_pll_60mhz().enable_lse();

    let p = hal::init(config);

    // LED PA8
    let mut blue_led = Output::new(p.PA8, Level::Low, OutputDrive::_5mA);

    let mut serial = UartTx::new(p.Uart1, p.PA9, Default::default()).unwrap();

    let download_button = Input::new(p.PB22, Pull::Up);
    let reset_button = Input::new(p.PB23, Pull::Up);

    serial.blocking_flush().unwrap();

    writeln!(serial, "\n\n\nHello World!").unwrap();
    writeln!(serial, "Clocks: {}", hal::sysctl::clocks().hclk).unwrap();
    writeln!(serial, "ChipID: {:02x}", hal::signature::get_chip_id()).unwrap();

    // ADC part
    let mut adc = Adc::new(p.ADC, hal::adc::Config::default());

    let _vbat_channel = adc.enable_vbat();
    //let mut temp_sensor = adc.enable_temperature();

    let mut pin = p.PA4;

    loop {
        blue_led.toggle();

        let data = adc.read(&mut pin);
        writeln!(serial, "adc raw data: {}", data).unwrap();
        let vi = adc.read_as_millivolts(&mut pin);
        writeln!(serial, "Vbat voltage: {}mV", vi).unwrap();

        writeln!(
            serial,
            "BUTTON state: download={} reset={}",
            // now.isoweekday(),
            download_button.is_low(),
            reset_button.is_low()
        )
        .unwrap();

        hal::delay_ms(1000);
    }
}
