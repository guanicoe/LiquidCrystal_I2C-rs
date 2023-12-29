
### I2C LCD
Port of the liquide crystall I2C lirary found for arduino in rust.
*Supports 4x16 Displays.*
Tested on raspberry pi and esp32. 

Example of use:

```rust
use rppal::{gpio::Gpio, i2c::I2c};

static  LCD_ADDRESS: u8 = 0x27;

fn main() {
    let mut i2c = I2c::new().unwrap();
    let mut delay = rppal::hal::Delay;

    let mut lcd = i2c_lcd::Lcd::new(&mut i2c, LCD_ADDRESS, &mut delay).unwrap();

    lcd.set_display(screen::Display::On).unwrap();
    lcd.set_backlight(screen::Backlight::On).unwrap();
    lcd.print("Hello world!").unwrap();
}

```
