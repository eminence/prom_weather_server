

pub mod temperature {
    pub struct Celsius(pub f32);
}

pub mod pressure {
    pub struct Hectopascal(pub f32);
}

pub mod speed {
    pub struct MetersPerSec(pub f32);
}

pub mod distance {
    pub struct Kilometers(pub u32);
    pub struct Meters(pub u32);
}

pub mod humidity {
    pub struct Percent(pub u8);
}
