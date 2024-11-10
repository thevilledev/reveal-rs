use std::ops::{Mul, Sub};
use num_traits::{Float, FromPrimitive};

// Helper function to convert HSV to RGB
pub fn hsv_to_rgb<T>(h: T, s: T, v: T) -> (u8, u8, u8)
where
    T: Float + FromPrimitive + Mul<Output = T> + Sub<Output = T>
{
    let six = T::from_f64(6.0).unwrap();
    let one = T::from_f64(1.0).unwrap();
    let n255 = T::from_f64(255.0).unwrap();

    let h = h * six;
    let i = h.floor();
    let f = h - i;
    let p = v * (one - s);
    let q = v * (one - s * f);
    let t = v * (one - s * (one - f));

    let (r, g, b) = match i.to_i32().unwrap() % 6 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    };

    ((r * n255).to_u8().unwrap(),
     (g * n255).to_u8().unwrap(),
     (b * n255).to_u8().unwrap())
}