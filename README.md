# PCF8591 driver

I2C driver for the PCF8591 8-bit A/D converter

## [Examples](examples)
```rust
use linux_embedded_hal::I2cdev;
use pcf8591::PCF8591;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut driver = PCF8591::new(dev);

    let data0 = driver.query_ain0().unwrap();
    println!("Potentiometer: {}", data0);

    let data1 = driver.query_ain1().unwrap();
    println!("Photoresistor: {}", data1);

    let data2 = driver.query_ain2().unwrap();
    println!("Thermistor: {}", data2);
}

```