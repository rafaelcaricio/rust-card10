/// Accelerometer, Gyroscope, Orientation

use core::{
    fmt::{self, Display, Write},
    marker::PhantomData,
    mem::MaybeUninit,
};

use card10_sys::*;

pub trait SensorType {
    /// sensor_type in C, sensor_id in Python
    fn sensor_type() -> u32;
    fn convert_single(value: i16) -> f32;
}

#[derive(Debug)]
pub struct Accelerometer;
impl SensorType for Accelerometer {
    fn sensor_type() -> u32 {
        0
    }
    fn convert_single(value: i16) -> f32 {
        2.0 * f32::from(value) / 32768.0
    }
}

#[derive(Debug)]
pub struct Gyroscope;
impl SensorType for Gyroscope {
    fn sensor_type() -> u32 {
        3
    }
    fn convert_single(value: i16) -> f32 {
        360.0 * f32::from(value) / 32768.0
    }
}

#[derive(Debug)]
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
    /// Use one of:
    /// - `BHI160::<Accelerometer>::start()`
    /// - `BHI160::<Gyroscope>::start()`
    /// - `BHI160::<Orientation>::start()`
    fn new(stream_id: i32) -> Self {
        Self {
            stream_id,
            _kind: PhantomData,
        }
    }

    pub fn start() -> Result<Self, Error> {
        let mut cfg = bhi160_sensor_config {
            sample_buffer_len: 200,
            sample_rate: 4,
            dynamic_range: 2,
            _padding: [0u8; 8],
        };

        let stream_id = unsafe { epic_bhi160_enable_sensor(S::sensor_type(), &mut cfg) };
        if stream_id < 0 {
            let error = match -stream_id {
                errno::EBUSY => Error::DriverBusy,
                _ => Error::Unknown(stream_id),
            };

            return Err(error);
        }

        Ok(Sensor::new(stream_id))
    }

    pub fn read(&self) -> Result<SensorData<S>, Error> {
        let mut buffer = MaybeUninit::<[bhi160_data_vector; DATA_MAX]>::zeroed();
        let buffer_pointer = buffer.as_mut_ptr() as *mut _;

        let packet_count = unsafe { epic_stream_read(self.stream_id, buffer_pointer, DATA_MAX) };
        if packet_count < 0 {
            let error = match -packet_count {
                errno::ENODEV => Error::SensorUnavailable,
                errno::EBADF => Error::SensorDescriptorUnknown,
                errno::EINVAL => Error::InvalidSampleCount,
                errno::EBUSY => Error::CouldNotAcquireLock,
                _ => Error::Unknown(packet_count),
            };

            return Err(error);
        }

        Ok(SensorData {
            buf: unsafe { buffer.assume_init() },
            n: packet_count as usize,
            _kind: PhantomData,
        })
    }
}

impl<S: SensorType> Drop for Sensor<S> {
    fn drop(&mut self) {
        unsafe {
            epic_bhi160_disable_sensor(S::sensor_type());
        }
    }
}

pub struct SensorData<S> {
    buf: [bhi160_data_vector; DATA_MAX],
    n: usize,
    _kind: PhantomData<S>,
}

impl<'a, S: SensorType> IntoIterator for &'a SensorData<S> {
    type Item = SensorDataItem<S>;
    type IntoIter = SensorDataIter<'a, S>;

    fn into_iter(self) -> Self::IntoIter {
        SensorDataIter { data: self, pos: 0 }
    }
}

pub struct SensorDataIter<'a, S> {
    data: &'a SensorData<S>,
    pos: usize,
}

impl<'a, S: SensorType> Iterator for SensorDataIter<'a, S> {
    type Item = SensorDataItem<S>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.pos < self.data.n {
            let vec = &self.data.buf[self.pos];
            if vec.data_type != DATA_TYPE_VECTOR {
                writeln!(crate::UART, "Sensor: skip type {}\r", vec.data_type).ok();
                continue;
            }

            let item = SensorDataItem::<S> {
                x_raw: vec.x,
                y_raw: vec.y,
                z_raw: vec.z,
                status: vec.status,
                _kind: PhantomData,
            };

            self.pos += 1;

            return Some(item);
        }

        None
    }
}

#[derive(Debug, Clone)]
pub struct SensorDataItem<S: SensorType> {
    pub x_raw: i16,
    pub y_raw: i16,
    pub z_raw: i16,
    pub status: u8,
    _kind: PhantomData<S>,
}

impl<S: SensorType> SensorDataItem<S> {
    pub fn get_x(&self) -> f32 {
        S::convert_single(self.x_raw)
    }

    pub fn get_y(&self) -> f32 {
        S::convert_single(self.y_raw)
    }

    pub fn get_z(&self) -> f32 {
        S::convert_single(self.z_raw)
    }
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

// -----------------------------------------------------------------------------
// BHI160 Error
// -----------------------------------------------------------------------------

#[derive(Debug)]
pub enum Error {
    /// The descriptor table lock could not be acquired.
    CouldNotAcquireLock,
    /// The BHI160 driver is currently busy with other tasks and could not be
    /// acquired for enabling a sensor.
    DriverBusy,
    /// The requested sample `count` is not a multiple of the sensor's sample
    /// size.
    InvalidSampleCount,
    /// The given sensor descriptor is unknown.
    SensorDescriptorUnknown,
    /// Sensor is not currently available.
    SensorUnavailable,
    /// Not yet documented and therefore unknown error types.
    Unknown(i32),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::CouldNotAcquireLock => writeln!(f, "Could not acquire BHI160 lock."),
            Error::DriverBusy => writeln!(f, "The BHI160 Driver is busy."),
            Error::InvalidSampleCount => writeln!(f, "Sample couldn't invalid."),
            Error::SensorDescriptorUnknown => writeln!(f, "Unknown BHI160 sensor descriptor."),
            Error::SensorUnavailable => writeln!(f, "The BHI160 sensor is currently unavailable."),
            Error::Unknown(id) => writeln!(f, "Unknown error id: {}", id),
        }
    }
}
