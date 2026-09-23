#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Vector3 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Imu {
    pub accel: Vector3,
    pub gyro: Vector3,
    pub mag: Vector3,
}
