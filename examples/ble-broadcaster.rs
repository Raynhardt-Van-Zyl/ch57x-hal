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

    static mut SCAN_RSP_DATA: [u8; 16] = [
        0x0c,
        GAP_ADTYPE_LOCAL_NAME_COMPLETE,
        b'B',
        b'r',
        b'o',
        b'a',
        b'd',
        b'c',
        b'a',
        b's',
        b't',
        b'e',
        b'r',
        0x02,
        GAP_ADTYPE_POWER_LEVEL,
        0,
    ];
    static mut ADVERT_DATA: [u8; 22] = [
        0x02,
        GAP_ADTYPE_FLAGS,
        GAP_ADTYPE_FLAGS_BREDR_NOT_SUPPORTED,
        0x04,
        GAP_ADTYPE_MANUFACTURER_SPECIFIC,
        0xD7,
        0x07,
        0x01,
        0x0a,
        GAP_ADTYPE_LOCAL_NAME_SHORT,
        b'c',
        b'h',
        b'5',
        b'8',
        b'x',
        b'-',
        b'h',
        b'a',
        b'l',
        0x02,
        GAP_ADTYPE_POWER_LEVEL,
        0,
    ];

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

    unsafe extern "C" fn broadcaster_callback(new_state: u32) {
        println!("broadcast state: {}", new_state);
    }

    #[embassy_executor::task]
    async fn broadcaster() {
        static CALLBACKS: gapRolesBroadcasterCBs_t = gapRolesBroadcasterCBs_t {
            pfnStateChange: Some(broadcaster_callback),
            pfnScanRecv: None,
        };

        unsafe {
            println!("set up callback=> {:?}", CALLBACKS);
            let r = GAPRole_BroadcasterStartDevice(&CALLBACKS);
            println!("GAPRole_BroadcasterStartDevice: {:?}", r);
        }
    }

    #[highcode]
    async fn mainloop() -> ! {
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

        println!();
        println!("Hello World from ch57x-hal!");
        println!("System Clocks: {}", hal::sysctl::clocks().hclk);
        println!("ChipID: 0x{:02x}", hal::signature::get_chip_id());
        println!("RTC datetime: {}", rtc.now());
        println!("BLE Lib Version: {}", ble::lib_version());

        let (task_id, _) = hal::ble::init(Default::default()).unwrap();
        println!("init BLE task id: {}", task_id);

        unsafe {
            println!("Gen Addr: {:08x}", BLE_AccessAddressGenerate());
            let r = GAPRole_BroadcasterInit();
            println!("GAPRole_BroadcasterInit: {:?}", r);

            GAPRole_SetParameter(GAPROLE_ADVERT_ENABLED, 1, &true as *const _ as _).unwrap();
            GAPRole_SetParameter(GAPROLE_ADV_EVENT_TYPE, 1, &0x03 as *const _ as _).unwrap();
            GAPRole_SetParameter(
                GAPROLE_SCAN_RSP_DATA,
                SCAN_RSP_DATA.len() as _,
                SCAN_RSP_DATA.as_mut_ptr() as _,
            )
            .unwrap();
            GAPRole_SetParameter(
                GAPROLE_ADVERT_DATA,
                ADVERT_DATA.len() as _,
                ADVERT_DATA.as_mut_ptr() as _,
            )
            .unwrap();

            GAP_SetParamValue(TGAP_DISC_ADV_INT_MIN, 160).unwrap();
            GAP_SetParamValue(TGAP_DISC_ADV_INT_MAX, 160).unwrap();
        }

        spawner.spawn(broadcaster()).unwrap();
        spawner.spawn(blink(p.PA8.degrade())).unwrap();
        mainloop().await
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
