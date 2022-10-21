pub trait One {
    fn one() -> Self;
}

impl One for i8 {
    fn one() -> Self {
        1 as i8
    }
}

impl One for i16 {
    fn one() -> Self {
        1 as i16
    }
}

impl One for i32 {
    fn one() -> Self {
        1 as i32
    }
}

impl One for i64 {
    fn one() -> Self {
        1 as i64
    }
}

impl One for i128 {
    fn one() -> Self {
        1 as i128
    }
}

impl One for f32 {
    fn one() -> Self {
        1.0 as f32
    }
}

impl One for f64 {
    fn one() -> Self {
        1.0 as f64
    }
}
