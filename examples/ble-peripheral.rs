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
    use core::ffi::c_void;
    use core::mem::size_of_val;

    use ch57x_hal as hal;
    use embassy_executor::Spawner;
    use embassy_time::{Duration, Timer};
    use hal::ble::ffi::*;
    use hal::ble::gap::*;
    use hal::ble::gattservapp::*;
    use hal::gpio::{AnyPin, Input, Level, Output, OutputDrive, Pin, Pull};
    use hal::peripherals;
    use hal::rtc::Rtc;
    use hal::uart::UartTx;
    use hal::{ble, println};
    use qingke_rt::highcode;

    static mut SCAN_RSP_DATA: &[u8] = &[
        0x12,
        GAP_ADTYPE_LOCAL_NAME_COMPLETE,
        b'S',
        b'i',
        b'm',
        b'p',
        b'l',
        b'e',
        b' ',
        b'P',
        b'e',
        b'r',
        b'i',
        b'p',
        b'h',
        b'e',
        b'r',
        b'a',
        b'l',
        0x02,
        GAP_ADTYPE_POWER_LEVEL,
        0,
    ];
    static mut ADVERT_DATA: &[u8] = &[
        0x02,
        GAP_ADTYPE_FLAGS,
        GAP_ADTYPE_FLAGS_BREDR_NOT_SUPPORTED,
        0x04,
        GAP_ADTYPE_MANUFACTURER_SPECIFIC,
        0xD7,
        0x07,
        0x01,
    ];
    static ATT_DEVICE_NAME: &[u8] = b"Simple Peripheral";

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

    fn peripheral_init() {
        unsafe {
            const MIN_INTERVAL: u16 = 6;
            const MAX_INTERVAL: u16 = 100;

            GAPRole_SetParameter(GAPROLE_ADVERT_ENABLED, 1, &true as *const _ as _);
            GAPRole_SetParameter(GAPROLE_SCAN_RSP_DATA, SCAN_RSP_DATA.len() as _, SCAN_RSP_DATA.as_ptr() as _);
            GAPRole_SetParameter(GAPROLE_ADVERT_DATA, ADVERT_DATA.len() as _, ADVERT_DATA.as_ptr() as _);
            GAPRole_SetParameter(GAPROLE_MIN_CONN_INTERVAL, 2, &MIN_INTERVAL as *const _ as _);
            GAPRole_SetParameter(GAPROLE_MAX_CONN_INTERVAL, 2, &MAX_INTERVAL as *const _ as _);

            GGS_SetParameter(
                GGS_DEVICE_NAME_ATT,
                ATT_DEVICE_NAME.len() as _,
                ATT_DEVICE_NAME.as_ptr() as _,
            );

            GAP_SetParamValue(TGAP_DISC_ADV_INT_MIN, 80);
            GAP_SetParamValue(TGAP_DISC_ADV_INT_MAX, 80);
            GAP_SetParamValue(TGAP_ADV_SCAN_REQ_NOTIFY, 1);

            let passkey: u32 = 0;
            let pair_mode = GAPBOND_PAIRING_MODE_WAIT_FOR_REQ;
            let mitm = true;
            let bonding = true;
            let io_cap = GAPBOND_IO_CAP_DISPLAY_ONLY;
            GAPBondMgr_SetParameter(
                GAPBOND_PERI_DEFAULT_PASSCODE,
                size_of_val(&passkey) as _,
                &passkey as *const _ as _,
            );
            GAPBondMgr_SetParameter(GAPBOND_PERI_PAIRING_MODE, 1, &pair_mode as *const _ as _);
            GAPBondMgr_SetParameter(GAPBOND_PERI_MITM_PROTECTION, 1, &mitm as *const _ as _);
            GAPBondMgr_SetParameter(GAPBOND_PERI_IO_CAPABILITIES, 1, &io_cap as *const _ as _);
            GAPBondMgr_SetParameter(GAPBOND_PERI_BONDING_ENABLED, 1, &bonding as *const _ as _);

            GGS_AddService(GATT_ALL_SERVICES).unwrap();
            GATTServApp::add_service(GATT_ALL_SERVICES).unwrap();

            static CB: gapRolesBroadcasterCBs_t = gapRolesBroadcasterCBs_t {
                pfnScanRecv: None,
                pfnStateChange: None,
            };
            GAPRole_BroadcasterSetCB(&CB);
        }
    }

    #[embassy_executor::task]
    async fn peripheral(task_id: u8) {
        unsafe extern "C" fn on_state_change(new_state: gapRole_States_t, event: *mut gapRoleEvent_t) {
            println!("in on_state_change: {}", new_state);
            let event = &*event;
            match new_state {
                GAPROLE_STARTED => println!("initialized.."),
                GAPROLE_ADVERTISING => println!("advertising.."),
                GAPROLE_WAITING => {
                    if event.gap.opcode == GAP_END_DISCOVERABLE_DONE_EVENT {
                        println!("waiting for advertising..");
                    } else if event.gap.opcode == GAP_LINK_TERMINATED_EVENT {
                        println!("disconnected .. reason {:x}", event.linkTerminate.reason);
                        GAPRole_SetParameter(GAPROLE_ADVERT_ENABLED, 1, &true as *const _ as _).unwrap();
                    }
                }
                GAPROLE_CONNECTED => println!("connected.. !!"),
                GAPROLE_ERROR => println!("error.."),
                _ => println!("unknown state: {}", new_state),
            }
        }
        unsafe extern "C" fn on_rssi_read(conn_handle: u16, rssi: i8) {
            println!("RSSI -{} dB Conn {:x}", -rssi, conn_handle);
        }
        unsafe extern "C" fn on_param_update(conn_handle: u16, interval: u16, _slave_latency: u16, timeout: u16) {
            println!(
                "on_param_update Conn handle: {} inverval: {} timeout: {}",
                conn_handle, interval, timeout
            );
        }

        unsafe {
            static BOND_MGR_CB: gapBondCBs_t = gapBondCBs_t {
                passcodeCB: None,
                pairStateCB: None,
                oobCB: None,
            };
            static APP_CB: gapRolesCBs_t = gapRolesCBs_t {
                pfnStateChange: Some(on_state_change),
                pfnRssiRead: Some(on_rssi_read),
                pfnParamUpdate: Some(on_param_update),
            };
            GAPRole_PeripheralStartDevice(task_id, &BOND_MGR_CB, &APP_CB).unwrap();
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

        let _boot_btn = Input::new(p.PB22, Pull::Up);
        let rtc = Rtc::new(p.RTC);

        println!();
        println!("Hello World from ch57x-hal!");
        println!("System Clocks: {}", hal::sysctl::clocks().hclk);
        println!("ChipID: 0x{:02x}", hal::signature::get_chip_id());
        println!("RTC datetime: {}", rtc.now());

        spawner.spawn(blink(p.PA8.degrade())).unwrap();

        println!("BLE Lib Version: {}", ble::lib_version());
        let (task_id, _sub) = hal::ble::init(Default::default()).unwrap();
        println!("BLE task id: {}", task_id);

        unsafe {
            let r = GAPRole_PeripheralInit();
            println!("GAPRole_PeripheralInit: {:?}", r);
        }

        peripheral_init();
        spawner.spawn(peripheral(task_id)).unwrap();
        mainloop().await
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
