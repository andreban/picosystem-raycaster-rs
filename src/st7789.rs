use cortex_m::delay::Delay;
use display_interface::{DataFormat, DisplayError, WriteOnlyDataCommand};
use embedded_hal::digital::v2::OutputPin;

const ST7789_SWRESET: u8 = 0x01;
const ST7789_SLPIN: u8 = 0x10;
const ST7789_SLPOUT: u8 = 0x11;
const ST7789_NORON: u8 = 0x13;
const ST7789_INVOFF: u8 = 0x20;
const ST7789_INVON: u8 = 0x21;
const ST7789_DISPON: u8 = 0x29;
const ST7789_CASET: u8 = 0x2A;
const ST7789_RASET: u8 = 0x2B;
const ST7789_RAMWR: u8 = 0x2C;
// const ST7789_RAMRD: u8 = 0x2E;
const ST7789_TEON: u8 = 0x35;
const ST7789_MADCTL: u8 = 0x36;
const ST7789_COLMOD: u8 = 0x3A;

// Color Modes
// const COLOR_MODE_65K: u8 = 0x50;
// const COLOR_MODE_262K: u8 = 0x60;
// const COLOR_MODE_12BIT: u8 = 0x03;
const COLOR_MODE_16BIT: u8 = 0x05;
// const COLOR_MODE_18BIT: u8 = 0x06;
// const COLOR_MODE_16M: u8 = 0x07;

pub struct St7789<DI: WriteOnlyDataCommand, RST: OutputPin> {
    display_interface: DI,
    pin_rst: Option<RST>,
    delay: Delay,
}

#[derive(Debug)]
pub enum St7789Error<PE> {
    DisplayError,
    Pin(PE),
}

impl<PE> From<DisplayError> for St7789Error<PE> {
    fn from(_: DisplayError) -> Self {
        St7789Error::DisplayError
    }
}

impl<DI: WriteOnlyDataCommand, RST: OutputPin> St7789<DI, RST> {
    pub fn new(display_interface: DI, pin_rst: Option<RST>, delay: Delay) -> Self {
        Self {
            display_interface,
            pin_rst,
            delay,
        }
    }

    pub fn init(&mut self) -> Result<(), St7789Error<RST::Error>> {
        if self.pin_rst.is_some() {
            self.hard_reset()?;
        }
        self.soft_reset()?;

        self.sleep_mode(false)?;
        self.delay.delay_ms(50);

        // TODO(andreban): always using 240x240. Prepare for 240x135
        //   if(width == 240 && height == 240) {
        self.send_command(ST7789_MADCTL, Some(&[0x04]))?; // row/column addressing order - rgb pixel order
        self.send_command(ST7789_TEON, Some(&[0x00]))?; // enable frame sync signal if used
                                                        //

        self.set_color_mode(COLOR_MODE_16BIT)?;
        self.set_invert_mode(true)?;
        self.delay.delay_ms(10);

        self.send_command(ST7789_NORON, None)?;
        self.delay.delay_ms(10);

        self.send_command(ST7789_DISPON, None)?;

        // setup correct addressing window
        // if(width == 240 && height == 240) {
        self.send_command(ST7789_CASET, Some(&[0x00, 0x00, 0x00, 0xef]))?; // 0 .. 239 columns
        self.send_command(ST7789_RASET, Some(&[0x00, 0x00, 0x00, 0xef]))?; // 0 .. 239 rows
                                                                           // }
        Ok(())
    }

    fn send_command(&mut self, command: u8, params: Option<&[u8]>) -> Result<(), DisplayError> {
        self.display_interface
            .send_commands(DataFormat::U8(&[command]))?;
        if let Some(params) = params {
            self.display_interface.send_data(DataFormat::U8(params))?;
        }
        Ok(())
    }

    fn soft_reset(&mut self) -> Result<(), DisplayError> {
        self.send_command(ST7789_SWRESET, None)?;
        self.delay.delay_ms(150);
        Ok(())
    }

    fn hard_reset(&mut self) -> Result<(), St7789Error<RST::Error>> {
        if let Some(rst) = &mut self.pin_rst {
            rst.set_high().map_err(St7789Error::Pin)?;
            self.delay.delay_ms(50);
            rst.set_low().map_err(St7789Error::Pin)?;
            self.delay.delay_ms(50);
            rst.set_high().map_err(St7789Error::Pin)?;
            self.delay.delay_ms(150);
        }
        Ok(())
    }

    fn sleep_mode(&mut self, mode: bool) -> Result<(), DisplayError> {
        let command = match mode {
            true => ST7789_SLPIN,
            false => ST7789_SLPOUT,
        };
        self.send_command(command, None)?;
        Ok(())
    }

    fn set_invert_mode(&mut self, mode: bool) -> Result<(), DisplayError> {
        let command = match mode {
            true => ST7789_INVON,
            false => ST7789_INVOFF,
        };
        self.send_command(command, None)?;
        Ok(())
    }

    fn set_color_mode(&mut self, mode: u8) -> Result<(), DisplayError> {
        self.send_command(ST7789_COLMOD, Some(&[mode]))?;
        Ok(())
    }

    pub fn set_pixels(&mut self, data: &[u16; 240 * 240]) -> Result<(), DisplayError> {
        self.display_interface
            .send_commands(DataFormat::U8(&[ST7789_RAMWR]))?;
        self.display_interface.send_data(DataFormat::U16(data))?;
        Ok(())
    }
}
