#![no_std]
#![no_main]
// for embassy main
#![feature(type_alias_impl_trait)]

use defmt::{info, warn, trace, debug, error};
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::{bind_interrupts, i3c, peripherals};
use embassy_stm32::i3c::I3cTarget;
use embassy_stm32::time::Hertz;
use embassy_time::{Duration, Timer};

bind_interrupts!(struct Irqs {
    I3C1_EV => i3c::InterruptHandler<peripherals::I3C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello, world!");

    let mut led = Output::new(p.PA5, Level::High, Speed::Low);

    // cn10 pin 3, and cn5 pin 10 SCL/D15
    let pin_scl = p.PB6;
    // cn10 pin 5, and cn5 pin 9 SDA/D14
    let pin_sda = p.PB7;
    // cn10 pin 9 is gnd


    let mut i3c = I3cTarget::new(
        p.I3C1,
        pin_scl,
        pin_sda,
        Irqs,
        // TODO: are these channels arbitrary?
        p.GPDMA1_CH4,
        p.GPDMA1_CH5,
        Default::default(),
    );


    loop {
        // let i = embassy_stm32::pac::I3C1;
        // info!("hotjoin evr {:08x} devr0 {:08x} sr {:08x} ser {:08x}  cr {:08x}",
        //     i.evr().read().0,
        //     i.devr0().read().0,
        //     i.sr().read().0,
        //     i.ser().read().0,
        //     i.cr().read().0,
        //     );

        i3c.hotjoin();

        led.set_high();
        Timer::after(Duration::from_millis(2000)).await;

        led.set_low();
        Timer::after(Duration::from_millis(2000)).await;
    }
}
