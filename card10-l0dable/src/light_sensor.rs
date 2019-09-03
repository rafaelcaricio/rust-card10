use card10_sys::*;

pub struct LightSensor {
    // Prevent creation of this struct by all but this module.
    _private: (),
}

impl LightSensor {
    pub fn start() -> Self {
        if unsafe { epic_light_sensor_run() } != 0 {
            panic!("Cannot start light sensor");
        }
        LightSensor { _private: () }
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
        unsafe {
            epic_light_sensor_stop();
        }
    }
}
