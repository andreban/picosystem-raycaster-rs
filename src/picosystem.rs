use cortex_m::{delay::Delay, prelude::_embedded_hal_digital_InputPin};
use display_interface_spi::SPIInterface;
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::Rgb565, prelude::*};
use embedded_hal::{
    digital::v2::{InputPin, OutputPin},
    spi::MODE_3,
};

use fugit::RateExtU32;
use mipidsi::{models::ST7789, Display, DisplayOptions};
// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;
use pimoroni_picosystem::{
    hal::{self, Clock},
    hal::{
        gpio::{
            bank0::{Gpio4, Gpio5, Gpio9, Gpio18, Gpio23, Gpio20, Gpio22, Gpio21},
            Input, Output, Pin, PinId, PinMode, PushPull, ValidPinMode, PullDown,
        },
        spi::Enabled,
        Spi, Timer,
    },
    pac, XOSC_CRYSTAL_FREQ,
};

type PicoSystemDisplay = Display<
    SPIInterface<
        Spi<Enabled, pac::SPI0, 8>,
        Pin<Gpio9, Output<PushPull>>,
        Pin<Gpio5, Output<PushPull>>,
    >,
    Pin<Gpio4, Output<PushPull>>,
    ST7789,
>;

pub struct Button<PIN> {
    pin: PIN,
}

impl<PIN> Button<PIN>
where
    PIN: InputPin,
{
    pub fn is_pressed(&self) -> bool {
        self.pin.is_low().unwrap_or(false)
    }
}

pub struct PicoSystem {
    pub display: PicoSystemDisplay,
    pub button_up: Button<Pin<Gpio23, Input<PullDown>>>,
    pub button_down: Button<Pin<Gpio20, Input<PullDown>>>,
    pub button_left: Button<Pin<Gpio22, Input<PullDown>>>,
    pub button_right: Button<Pin<Gpio21, Input<PullDown>>>,
    pub button_a: Button<Pin<Gpio18, Input<PullDown>>>,
    pub timer: Timer,
}

impl PicoSystem {
    pub fn take() -> Option<Self> {
        // Grab our singleton objects
        let mut pac = pac::Peripherals::take().unwrap();
        let core = pac::CorePeripherals::take().unwrap();

        // Set up the watchdog driver - needed by the clock setup code
        let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

        // Configure the clocks
        //
        // The default is to generate a 125 MHz system clock
        let clocks = hal::clocks::init_clocks_and_plls(
            XOSC_CRYSTAL_FREQ,
            pac.XOSC,
            pac.CLOCKS,
            pac.PLL_SYS,
            pac.PLL_USB,
            &mut pac.RESETS,
            &mut watchdog,
        )
        .ok()
        .unwrap();

        // The single-cycle I/O block controls our GPIO pins
        let sio = hal::Sio::new(pac.SIO);

        // Set the pins up according to their function on this particular board
        let pins = pimoroni_picosystem::Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );

        // Configure ST7789
        let lcd_dc = pins.lcd_dc.into_push_pull_output();
        let lcd_cs = pins.lcd_cs.into_push_pull_output();
        let lcd_reset = pins.lcd_reset.into_push_pull_output();

        pins.lcd_mosi.into_mode::<hal::gpio::FunctionSpi>();
        pins.lcd_sclk.into_mode::<hal::gpio::FunctionSpi>();

        let mut lcd_backlight = pins.lcd_backlight.into_push_pull_output();
        lcd_backlight.set_high().unwrap();

        let spi_screen = Spi::<_, _, 8>::new(pac.SPI0).init(
            &mut pac.RESETS,
            125_000_000u32.Hz(),
            16_000_000u32.Hz(),
            &MODE_3,
        );

        let lcd_spi_interface = SPIInterface::new(spi_screen, lcd_dc, lcd_cs);

        let display_options = DisplayOptions {
            ..Default::default()
        };

        let mut display = Display::st7789(lcd_spi_interface, lcd_reset);
        let mut lcd_delay = Delay::new(core.SYST, clocks.system_clock.freq().raw());
        let timer = Timer::new(pac.TIMER, &mut pac.RESETS);

        display.init(&mut lcd_delay, display_options).unwrap();

        display.clear(Rgb565::RED).unwrap();

        let button_up = Button {
            pin: pins.button_up.into_pull_down_input(),
        };

        let button_down = Button {
            pin: pins.button_down.into_pull_down_input(),
        };

        let button_left = Button {
            pin: pins.button_left.into_pull_down_input(),
        };
        
        let button_right = Button {
            pin: pins.button_right.into_pull_down_input(),
        };

        let button_a = Button {
            pin: pins.button_a.into_pull_down_input(),
        };

        Some(Self {
            display,
            button_up,
            button_down,
            button_left,
            button_right,
            button_a,
            timer,
        })
    }
}
