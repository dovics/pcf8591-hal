use embedded_hal::blocking::i2c::{Operation as I2cOperation, Transactional};

const ADDR: u8 = 0x48;
pub struct PCF8591<I2C> {
    i2c: I2C,
}

enum ADCNum {
    AIN0,
    AIN1,
    AIN2,
    AIN3,
}

impl<I2C> PCF8591<I2C>
where
    I2C: Transactional,
{
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    pub fn query_ain0(&mut self) -> Result<u8, I2C::Error> {
        self.query(ADCNum::AIN0)
    }

    pub fn query_ain1(&mut self) -> Result<u8, I2C::Error> {
        self.query(ADCNum::AIN1)
    }

    pub fn query_ain2(&mut self) -> Result<u8, I2C::Error> {
        self.query(ADCNum::AIN2)
    }

    pub fn query_ain3(&mut self) -> Result<u8, I2C::Error> {
        self.query(ADCNum::AIN3)
    }

    fn query(&mut self, num: ADCNum) -> Result<u8, I2C::Error> {
        let write_buffer = match num {
            ADCNum::AIN0 => [0x40],
            ADCNum::AIN1 => [0x41],
            ADCNum::AIN2 => [0x42],
            ADCNum::AIN3 => [0x43],
        };

        let mut read_buffer = [0];
        let mut ops = [
            I2cOperation::Write(&write_buffer),
            I2cOperation::Read(&mut read_buffer),
        ];

        self.i2c.exec(ADDR, &mut ops).and(Ok(read_buffer[0]))
    }
}
