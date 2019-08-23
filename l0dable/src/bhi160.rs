use core::mem::uninitialized;
use core::marker::PhantomData;
use super::bindings::*;

use core::fmt::Write;

pub trait SensorType {
    /// sensor_type in C, sensor_id in Python
    fn sensor_type() -> u32;
    fn convert_single(value: i16) -> f32;
}

pub struct Accelerometer;
impl SensorType for Accelerometer {
    fn sensor_type() -> u32 {
        0
    }
    fn convert_single(value: i16) -> f32 {
        2.0 * f32::from(value) / 32768.0
    }
}

pub struct Gyroscope;
impl SensorType for Gyroscope {
    fn sensor_type() -> u32 {
        3
    }
    fn convert_single(value: i16) -> f32 {
        360.0 * f32::from(value) / 32768.0
    }
}

pub struct Orientation;
impl SensorType for Orientation {
    fn sensor_type() -> u32 {
        2
    }
    fn convert_single(value: i16) -> f32 {
        360.0 * f32::from(value) / 32768.0
    }
}

const DATA_MAX: usize = 10;

pub struct Sensor<S: SensorType> {
    stream_id: i32,
    _kind: PhantomData<S>,
}

impl<S: SensorType> Sensor<S> {
    pub fn start() -> Self {
        let mut cfg = bhi160_sensor_config {
            sample_buffer_len: 200,
            sample_rate: 4,
            dynamic_range: 2,
            _padding: [0u8; 8],
        };
        let stream_id = unsafe {
            epic_bhi160_enable_sensor(S::sensor_type(), &mut cfg)
        };
        Sensor {
            stream_id,
            _kind: PhantomData,
        }
    }

    pub fn read(&self) -> SensorData<S> {
        let mut buf: [bhi160_data_vector; DATA_MAX] = unsafe {
            uninitialized()
        };
        let n = unsafe {
            epic_stream_read(self.stream_id, buf.as_mut_ptr() as *mut _, buf.len())
        };
        if n < 0 {
            panic!("epic_stream_read fail");
        }
        let n = n as usize;
        SensorData {
            buf, n,
            _kind: PhantomData,
        }
    }
}

impl<S: SensorType> Drop for Sensor<S> {
    fn drop(&mut self) {
        unsafe { epic_bhi160_disable_sensor(S::sensor_type()); }
    }
}

pub struct SensorData<S> {
    buf: [bhi160_data_vector; DATA_MAX],
    n: usize,
    _kind: PhantomData<S>,
}

impl<'a, S: SensorType> IntoIterator for &'a SensorData<S> {
    type Item = SensorDataItem;
    type IntoIter = SensorDataIter<'a, S>;
    fn into_iter(self) -> Self::IntoIter {
        SensorDataIter {
            data: self,
            pos: 0,
        }
    }
}

pub struct SensorDataIter<'a, S> {
    data: &'a SensorData<S>,
    pos: usize,
}

impl<'a, S: SensorType> Iterator for SensorDataIter<'a, S> {
    type Item = SensorDataItem;
    fn next(&mut self) -> Option<Self::Item> {
        while self.pos < self.data.n {
            let vec = &self.data.buf[self.pos];
            self.pos += 1;
            if vec.data_type == DATA_TYPE_VECTOR {
                let item = SensorDataItem {
                    x: S::convert_single(vec.x),
                    y: S::convert_single(vec.y),
                    z: S::convert_single(vec.z),
                    status: vec.status,
                };
                return Some(item);
            } else {
                writeln!(crate::UART, "Sensor: skip type {}\r", vec.data_type).unwrap();
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct SensorDataItem {
    x: f32,
    y: f32,
    z: f32,
    status: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bhi160_data_vector {
    /// This one is wrongly defined by buildgen
    pub data_type: u8,
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub status: u8,
}

const DATA_TYPE_VECTOR: u8 = 0;
