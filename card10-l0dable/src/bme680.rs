use card10_sys::*;
use core::mem::uninitialized;

pub struct BME680;
pub type SensorData = bme680_sensor_data;

impl BME680 {
    pub fn start() -> Self {
        if unsafe { epic_bme680_init() } != 0 {
            panic!("Cannot start BME680 sensor");
        }
        BME680
    }

    pub fn read(&self) -> Option<SensorData> {
        let mut result = unsafe { uninitialized() };
        if unsafe { epic_bme680_read_sensors(&mut result) } == 0 {
            Some(result)
        } else {
            None
        }
    }
}

impl Drop for BME680 {
    fn drop(&mut self) {
        unsafe {
            epic_bme680_deinit();
        }
    }
}
