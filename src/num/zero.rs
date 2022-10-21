pub trait Zero {
    fn zero() -> Self;
}

impl Zero for i8 {
    fn zero() -> Self {
        0 as i8
    }
}

impl Zero for i16 {
    fn zero() -> Self {
        0 as i16
    }
}

impl Zero for i32 {
    fn zero() -> Self {
        0 as i32
    }
}

impl Zero for i64 {
    fn zero() -> Self {
        0 as i64
    }
}

impl Zero for i128 {
    fn zero() -> Self {
        0 as i128
    }
}

impl Zero for f32 {
    fn zero() -> Self {
        0.0 as f32
    }
}

impl Zero for f64 {
    fn zero() -> Self {
        0.0 as f64
    }
}
