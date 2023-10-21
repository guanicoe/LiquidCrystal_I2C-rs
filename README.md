Port of the liquide crystall I2C lirary found for arduino in rust. 
Tested on Raspberry PI and ESP32-WROOM-32. 

Example of use with RasberryPI:
```rust
use liquidcrystal_i2c_rs::{Backlight, Display, Lcd};
use rppal::i2c::I2c;
use std::thread::sleep;
use std::time::Duration;

static LCD_ADDRESS: u16 = 0x3f;

struct HW<'d> {
  i2c: I2c,
  delay: Delay,
}
  
// Implement the `Hardware` trait to give access to I2C pins
impl lcd::I2C for HW<'_> {
  fn write(&mut self, data: u8) -> Result<usize, lcd::I2cError> {
    self.i2c.write(self.address, &[data])
  }
}
  
// Implement the `Delay` trait to allow library to sleep for the given amount of time
impl lcd::Delay for HW<'_> {
  fn delay_us(&mut self, delay_usec: u32) {
    sleep(Duration::from_millis(delay_usec));
  }
}

fn main() {
  let hw = HW { 
    i2c: I2c::new().unwrap(),
    delay: Delay::new(),
  };

  hw.i2c.set_slave_address(LCD_ADDRESS);

  let mut lcd = screen::Lcd::new(hw).unwrap();

  lcd.set_display(screen::Display::On).unwrap();
  lcd.set_backlight(screen::Backlight::On).unwrap();
  lcd.print("Hello world!").unwrap();
}
```

Example of use with ESP32-WROOM-32:
```rust
use esp_idf_hal::units::KiloHertz;
use esp_idf_sys as _; If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use std::thread;
use std::time::Duration;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::i2c::I2cDriver;
use esp_idf_hal::i2c;

static  LCD_ADDRESS: u8 = 0x27;

struct HW<'d> {
  i2c: I2cDriver<'d>,
  address: u8,
}
  
// Implement the `Hardware` trait to give access to I2C pins
impl lcd::I2C for HW<'_> {
  fn write(&mut self, data: u8) -> Result<usize, lcd::I2cError> {
    if let Ok(()) = self.i2c.write(self.address, &[data], 1000) {
      return Ok(1);
    }
  
    return Err(lcd::I2cError::Io);
  }
}
  
// Implement the `Delay` trait to allow library to sleep for the given amount of time
impl lcd::Delay for HW<'_> {
  fn delay_us(&mut self, delay_usec: u32) {
    thread::sleep(Duration::from_millis(delay_usec.into()));
  }
}
  
fn main() {
  // It is necessary to call this function once. Otherwise some patches to the runtime
  // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
  esp_idf_sys::link_patches();
  // Bind the log crate to the ESP Logging facilities
  esp_idf_svc::log::EspLogger::initialize_default();
  
  let peripherals = Peripherals::take().unwrap();
  
  let scl = peripherals.pins.gpio22;
  let sda = peripherals.pins.gpio21;
  
  let i2c_config = i2c::config::Config::new()
    .baudrate(KiloHertz(100).into())
    .scl_enable_pullup(true)
    .sda_enable_pullup(true);
  
  let i2c_driver = i2c::I2cDriver::new(peripherals.i2c0, sda, scl, &i2c_config).unwrap();
  
  let hw = HW { 
    i2c: i2c_driver,
    address: LCD_ADDRESS,
  };
   
  let mut lcd_screen = lcd::Lcd::new(hw).unwrap();
  
  lcd_screen.print("Hello!").unwrap();
  
  loop {
    thread::sleep(Duration::from_millis(1000));
    
    println!("It's works!");
  }
}
```
