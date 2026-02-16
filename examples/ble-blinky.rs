#![no_std]
#![no_main]
#![cfg_attr(feature = "nightly", feature(type_alias_impl_trait))]

#[cfg(not(feature = "nightly"))]
#[qingke_rt::entry]
fn main() -> ! {
    loop {}
}

#[cfg(not(feature = "nightly"))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(feature = "nightly")]
mod app {
    use ch57x_hal as hal;
    use embassy_executor::Spawner;
    use embassy_time::{Duration, Timer};
    use hal::ble::ffi::*;
    use hal::ble::gap::*;
    use hal::gpio::{AnyPin, Level, Output, OutputDrive, Pin};
    use hal::peripherals;
    use hal::rtc::Rtc;
    use hal::uart::UartTx;
    use hal::{ble, println};
    use qingke_rt::highcode;

    #[embassy_executor::task]
    async fn blink(pin: AnyPin) {
        let mut led = Output::new(pin, Level::Low, OutputDrive::_5mA);
        loop {
            led.toggle();
            Timer::after(Duration::from_millis(150)).await;
        }
    }

    #[highcode]
    #[embassy_executor::task]
    async fn tmos_mainloop() {
        loop {
            Timer::after(Duration::from_micros(300)).await;
            unsafe {
                TMOS_SystemProcess();
            }
        }
    }

    #[embassy_executor::main(entry = "qingke_rt::entry")]
    async fn main(spawner: Spawner) -> ! {
        let mut config = hal::Config::default();
        config.clock.use_pll_60mhz().enable_lse();
        let p = hal::init(config);
        hal::embassy::init();

        let uart = UartTx::new(p.Uart1, p.PA9, Default::default()).unwrap();
        unsafe {
            hal::set_default_serial(uart);
        }

        let rtc = Rtc::new(p.RTC);
        println!("Hello World from ch57x-hal!");
        println!("System Clocks: {}", hal::sysctl::clocks().hclk);
        println!("ChipID: 0x{:02x}", hal::signature::get_chip_id());
        println!("RTC datetime: {}", rtc.now());
        println!("BLE Lib Version: {}", ble::lib_version());

        let (task_id, _sub) = hal::ble::init(Default::default()).unwrap();
        println!("BLE hal task id: {}", task_id);

        unsafe {
            let _ = GAPRole::peripheral_init().unwrap();
        }

        spawner.spawn(blink(p.PA8.degrade())).unwrap();
        spawner.spawn(tmos_mainloop()).unwrap();

        loop {
            Timer::after(Duration::from_secs(1)).await;
        }
    }

    #[panic_handler]
    fn panic(info: &core::panic::PanicInfo) -> ! {
        use core::fmt::Write;

        let pa9 = unsafe { peripherals::PA9::steal() };
        let uart1 = unsafe { peripherals::Uart1::steal() };
        let mut serial = UartTx::new(uart1, pa9, Default::default()).unwrap();
        let _ = writeln!(&mut serial, "\n\n\n{}", info);
        loop {}
    }
}
