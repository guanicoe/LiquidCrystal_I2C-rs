// Port of the liquide crystall I2C lirary found for arduino in rust.
// Tested on raspberry pi and ESP32-WROOM-32.
#[derive(Debug)]
pub enum I2cError {
    Io
}

/// Controls the visibilty of the non-blinking cursor, which is basically an _ **after** the cursor position.
/// The cursor position represents where the next character will show up.
#[derive(Copy, Clone, Debug)]
pub enum Cursor {
    /// Display the non-blinking cursor
    On = 0x02,
    /// Hide the non-blinking cursor
    Off = 0x00,
}

/// Controls the visibility of the blinking block cursor.
#[derive(Copy, Clone, Debug)]
pub enum Blink {
    /// Turn the blinking block cursor on
    On = 0x01,
    /// Turn the blinking block cursor off
    Off = 0x00,
}

/// Determines whether the entire LCD is on or off.
#[derive(Copy, Clone, Debug)]
pub enum Display {
    /// Turn the LCD display on
    On = 0x04,
    /// Turn the LCD display off
    Off = 0x00,
}

/// Determines whether the blaclight is on or off.
#[derive(Copy, Clone, Debug)]
pub enum Backlight {
    /// Turn the backlight on
    On = 0x08,
    /// Turn the backlight off
    Off = 0x00,
}

/// Commands
#[derive(Copy, Clone, Debug)]
pub enum Mode {
    COMMAND = 0x00,
    CLEARDISPLAY = 0x01,
    RETURNHOME = 0x02,
    ENTRYMODESET = 0x04,
    DISPLAYCONTROL = 0x08,
    CURSORSHIFT = 0x10,
    FUNCTIONSET = 0x20,
    SETCGRAMADDR = 0x40,
    SETDDRAMADDR = 0x80,
}

/// flags for display entry mode
#[derive(Copy, Clone, Debug)]
pub enum Entries {
    RIGHT = 0x00,
    LEFT = 0x02,
}

/// Flag for selection the display of cursor
#[derive(Copy, Clone, Debug)]
pub enum MoveSelect {
    DISPLAY = 0x08,
    CURSOR = 0x00,
}

// flags for selection the direction to wite in.
#[derive(Copy, Clone, Debug)]
pub enum Direction {
    RIGHT = 0x04,
    LEFT = 0x00,
}

#[derive(Copy, Clone, Debug)]
pub enum Shift {
    INCREMENT = 0x01,
    DECREMENT = 0x00,
}

#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
pub enum BitMode {
    Bit4 = 0x00,
    Bit8 = 0x10,
}

#[derive(Copy, Clone, Debug)]
pub enum Dots {
    Dots5x8 = 0x00,
    Dots5x10 = 0x04,
}

#[derive(Copy, Clone, Debug)]
pub enum Lines {
    OneLine = 0x00,
    TwoLine = 0x08,
}

#[derive(Copy, Clone, Debug)]
pub enum BitAction {
    Command = 0x00,
    Enable = 0x04,
    ReadWrite = 0x02,
    RegisterSelect = 0x01,
}
pub trait Delay {
    /// Delay for given amount of time (in microseconds).
    fn delay_us(&mut self, delay_usec: u32);
}

pub trait I2C {
    fn write(&mut self, data: u8) -> Result<usize, I2cError>;
}

pub struct DisplayControl {
    pub cursor: Cursor,
    pub display: Display,
    pub blink: Blink,
    pub backlight: Backlight,
    pub direction: Direction,
}

impl DisplayControl {
    pub fn new() -> Self {
        DisplayControl {
            cursor: Cursor::Off,
            display: Display::Off,
            blink: Blink::Off,
            backlight: Backlight::On,
            direction: Direction::LEFT,
        }
    }

    pub fn value(&self) -> u8 {
        self.blink as u8 | self.cursor as u8 | self.display as u8 | self.backlight as u8
    }
}

pub struct Lcd<HW: Delay + I2C> {
    hw: HW,
    control: DisplayControl,
}

impl<HW: Delay + I2C> Lcd<HW> {
    pub fn new(hw: HW) -> Result<Self, I2cError> {
        let mut display = Self {
            hw,
            control: DisplayControl::new(),
        };
        display.init()?;
        Ok(display)
    }

    // Initialize the display for the first time after power up
    fn init(&mut self) -> Result<(), I2cError> {
        // SEE PAGE 45/46 FOR INITIALIZATION SPECIFICATION!
        // according to datasheet, we need at least 40ms after power rises above 2.7V
        // before sending commands. Arduino can turn on way before 4.5V so we'll wait 50
        self.hw.delay_us(50);

        self.expander_write(self.control.backlight as u8)?;
        self.hw.delay_us(1);

        // Send the initial command sequence according to the HD44780 datasheet
        let mode_8bit = Mode::FUNCTIONSET as u8 | BitMode::Bit8 as u8;
        self.write4bits(mode_8bit)?;
        self.hw.delay_us(5);

        self.write4bits(mode_8bit)?;
        self.hw.delay_us(5);

        self.write4bits(mode_8bit)?;
        self.hw.delay_us(5);

        let mode_4bit = Mode::FUNCTIONSET as u8 | BitMode::Bit4 as u8;
        self.write4bits(mode_4bit)?;
        self.hw.delay_us(5);

        let lines_font = Mode::FUNCTIONSET as u8
            | BitMode::Bit4 as u8
            | Dots::Dots5x8 as u8
            | Lines::TwoLine as u8;
        self.command(lines_font)?;

        self.clear()?;

        let entry_mode = Mode::ENTRYMODESET as u8 | Entries::LEFT as u8 | Shift::DECREMENT as u8;
        self.command(entry_mode)?;

        Ok(())
    }

    /********** high level commands, for the user! */
    /**
    Clear the display. The LCD display driver requires a 2ms delay after clearing, which
    is why this method requires a `delay` object.

    # Errors

    Returns a `Result` that will report I2C errors, if any.
    */
    pub fn clear(&mut self) -> Result<(), I2cError> {
        self.command(Mode::CLEARDISPLAY as u8)?;
        self.hw.delay_us(2);
        Ok(())
    }

    /**
    Home

    # Errors

    Returns a `Result` that will report I2C errors, if any.
    */
    pub fn home(&mut self) -> Result<(), I2cError> {
        self.command(Mode::RETURNHOME as u8)?;
        self.hw.delay_us(2);
        Ok(())
    }

    /**
    Set the position of the cursor

    # Errors

    Returns a `Result` that will report I2C errors, if any.
    */
    pub fn set_cursor_position(&mut self, col: u8, row: u8) -> Result<(), I2cError> {
        self.command(Mode::SETDDRAMADDR as u8 | (col + row * 0x40))?;
        Ok(())
    }

    /**
    Control whether the display is on or off

    # Errors

    Returns a `Result` that will report I2C errors, if any.
    */
    pub fn set_display(&mut self, display: Display) -> Result<(), I2cError> {
        self.control.display = display;
        self.write_display_control()
    }

    /**
    Sets the visibility of the cursor, which is a non-blinking _

    # Errors

    Returns a `Result` that will report I2C errors, if any.
    */
    pub fn set_cursor(&mut self, cursor: Cursor) -> Result<(), I2cError> {
        self.control.cursor = cursor;
        self.write_display_control()
    }

    /**
    Turns on the blinking block cursor

    # Errors

    Returns a `Result` that will report I2C errors, if any.
    */
    pub fn set_blink(&mut self, blink: Blink) -> Result<(), I2cError> {
        self.control.blink = blink;
        self.write_display_control()
    }

    pub fn set_backlight(&mut self, backlight: Backlight) -> Result<usize, I2cError> {
        self.control.backlight = backlight;
        Ok(self.expander_write(0)?)
    }

    /*********** mid level commands, for sending data/cmds */

    /**
    Adds a string to the current position. The cursor will advance
    after this call to the next column
    # Errors
    Returns a `Result` that will report I2C errors, if any.
    */
    pub fn print(&mut self, s: &str) -> Result<(), I2cError> {
        for c in s.chars() {
            self.write(c as u8)?;
        }

        Ok(())
    }

    // Set one of the display's control options and then send the updated set of options to the display
    fn write_display_control(&mut self) -> Result<(), I2cError> {
        self.command(Mode::DISPLAYCONTROL as u8 | self.control.value())
    }

    // Send two bytes to the display
    fn write(&mut self, value: u8) -> Result<(), I2cError> {
        self.send(value, BitAction::RegisterSelect)
    }

    fn command(&mut self, value: u8) -> Result<(), I2cError> {
        self.send(value, BitAction::Command)
    }

    /************ low level data pushing commands **********/

    fn send(&mut self, data: u8, mode: BitAction) -> Result<(), I2cError> {
        let high_bits: u8 = data & 0xf0;
        let low_bits: u8 = (data << 4) & 0xf0;
        self.write4bits(high_bits | mode as u8)?;
        self.write4bits(low_bits | mode as u8)?;
        Ok(())
    }

    fn write4bits(&mut self, value: u8) -> Result<(), I2cError> {
        self.expander_write(value)?;
        self.pulse_enable(value)?;
        Ok(())
    }

    fn expander_write(&mut self, data: u8) -> Result<usize, I2cError> {
        self
            .hw
            .write(data | self.control.backlight as u8)
    }

    fn pulse_enable(&mut self, data: u8) -> Result<(), I2cError> {
        self.expander_write(data | BitAction::Enable as u8)?; // En high
        self.hw.delay_us(1);

        self.expander_write(data & !(BitAction::Enable as u8))?; // En low
        self.hw.delay_us(1);

        Ok(())
    }
}
