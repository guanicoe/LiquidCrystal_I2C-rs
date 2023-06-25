Port of the liquide crystall I2C lirary found for arduino in rust. 
Tested on raspberry pi. 

Example of use:

```rust
use liquidcrystal_i2c_rs::{Backlight, Display, Lcd};

static LCD_ADDRESS: u16 = 0x3f;

fn main() {
    let i2c = rppal::i2c::I2c::new().unwrap();

    let mut lcd = Lcd::new(i2c, LCD_ADDRESS).unwrap();

    lcd.set_display(Display::On).unwrap();
    lcd.set_backlight(Backlight::On).unwrap();

    lcd.clear().unwrap();
    lcd.print("Hello World!").unwrap();
}

```