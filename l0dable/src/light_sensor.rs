use super::bindings::*;

pub struct LightSensor;

impl LightSensor {
    pub fn start() -> Self {
        if unsafe { epic_light_sensor_run() } != 0 {
            panic!("Cannot start light sensor");
        }
        LightSensor
    }

    pub fn get(&self) -> Option<u16> {
        let mut result = 0;
        if unsafe { epic_light_sensor_get(&mut result) } == 0 {
            Some(result)
        } else {
            None
        }
    }
}

impl Drop for LightSensor {
    fn drop(&mut self) {
        unsafe { epic_light_sensor_stop(); }
    }
}
