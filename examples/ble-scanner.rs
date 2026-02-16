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
    use embassy_sync::channel::Channel;
    use embassy_time::{Duration, Timer};
    use hal::ble::ffi::*;
    use hal::ble::gap::*;
    use hal::ble::MacAddress;
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
            led.set_high();
            Timer::after(Duration::from_millis(150)).await;
            led.set_low();
            Timer::after(Duration::from_millis(150)).await;
        }
    }

    type CS = embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
    static EVENTS: Channel<CS, bool, 10> = Channel::new();

    const DEFAULT_DISCOVERY_MODE: u8 = DEVDISC_MODE_ALL;
    const DEFAULT_DISCOVERY_ACTIVE_SCAN: u8 = 0;
    const DEFAULT_DISCOVERY_WHITE_LIST: u8 = 0;

    unsafe extern "C" fn observer_event_callback(event: &gapRoleEvent_t) {
        match event.gap.opcode {
            GAP_DEVICE_INIT_DONE_EVENT => {
                println!("Discovering...");
                GAPRole_ObserverStartDiscovery(
                    DEFAULT_DISCOVERY_MODE,
                    DEFAULT_DISCOVERY_ACTIVE_SCAN,
                    DEFAULT_DISCOVERY_WHITE_LIST,
                )
                .unwrap();
            }
            GAP_DEVICE_DISCOVERY_EVENT => {
                println!("Complete");
                EVENTS.try_send(true).unwrap();
            }
            GAP_DEVICE_INFO_EVENT => {
                let event = event.deviceInfo;
                println!("Device => {}, rssi={}", MacAddress::from_raw(event.addr), event.rssi);
            }
            _ => {
                println!("unknown event opcode: {}", event.gap.opcode);
            }
        }
    }

    #[embassy_executor::task]
    async fn observer() {
        static CALLBACK: gapRoleObserverCB_t = gapRoleObserverCB_t {
            eventCB: Some(observer_event_callback),
        };
        unsafe {
            GAPRole_ObserverStartDevice(&CALLBACK).unwrap();
        }

        loop {
            EVENTS.receiver().receive().await;
            println!("Restarting discovery...");
            unsafe {
                GAPRole_CancelSync().unwrap();
                GAPRole_ObserverStartDiscovery(
                    DEFAULT_DISCOVERY_MODE,
                    DEFAULT_DISCOVERY_ACTIVE_SCAN,
                    DEFAULT_DISCOVERY_WHITE_LIST,
                )
                .unwrap();
            }
        }
    }

    #[embassy_executor::main(entry = "qingke_rt::entry")]
    #[highcode]
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

        println!();
        println!("Hello World from ch57x-hal!");
        println!("System Clocks: {}", hal::sysctl::clocks().hclk);
        println!("ChipID: 0x{:02x}", hal::signature::get_chip_id());
        println!("RTC datetime: {}", rtc.now());

        let (task_id, _) = ble::init(ble::Config::default()).unwrap();
        println!("BLE init task id: {:?}", task_id);
        println!("MemFree: {}K", hal::stack_free() / 1024);

        unsafe {
            GAPRole_ObserverInit().unwrap();
            GAP_SetParamValue(TGAP_DISC_SCAN, 4800).unwrap();
            GAP_SetParamValue(TGAP_DISC_SCAN_PHY, GAP_PHY_BIT_LE_1M).unwrap();
        }

        spawner.spawn(observer()).unwrap();
        spawner.spawn(blink(p.PA8.degrade())).unwrap();

        loop {
            Timer::after(Duration::from_micros(300)).await;
            unsafe {
                TMOS_SystemProcess();
            }
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
