#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use mint::{ColumnMatrix4, RowMatrix4};
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


impl From<mint::Vector3<f32>> for vec3 {
    fn from(val: mint::Vector3<f32>) -> Self {
        Self {
            x: val.x,
            y: val.y,
            z: val.z
        }
    }
}

impl From<mint::Vector2<f32>> for vec2 {
    fn from(val: mint::Vector2<f32>) -> Self {
        Self {
            x: val.x,
            y: val.y
        }
    }
}

impl From<mint::Vector4<f32>> for vec4 {
    fn from(val: mint::Vector4<f32>) -> Self {
        Self {
            x: val.x,
            y: val.y,
            z: val.z,
            w: val.w
        }
    }
}

impl From<mint::ColumnMatrix4<f32>> for matrix {
    fn from(m: mint::ColumnMatrix4<f32>) -> Self {
        matrix {
            row: [
                m.x.into(),
                m.y.into(),
                m.z.into(),
                m.w.into(),
            ],
        }
    }
}

impl Into<mint::ColumnMatrix4<f32>> for matrix {
    fn into(self) -> mint::ColumnMatrix4<f32> {
        unsafe {
            //This should not work, but it does work, do not ask me why, god only knows.
            match self {
                matrix { m: ma} => { ColumnMatrix4::from(ma) }
                matrix { row: r} => { ColumnMatrix4::from(RowMatrix4::from([
                    r[0].x, r[0].y, r[0].z, r[0].w, r[1].x, r[1].y, r[1].z, r[1].w, r[2].x, r[2].y,
                    r[2].z, r[2].w, r[3].x, r[3].y, r[3].z, r[3].w,
                ])) }
            }
        }
    }
}

impl From<mint::Quaternion<f32>> for quat {
    fn from(val: mint::Quaternion<f32>) -> Self {
        quat {
            x: val.v.x,
            y: val.v.y,
            z: val.v.z,
            w: val.s
        }
    }
}


impl color128 {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r,
            g,
            b,
            a,
        }
    }
    pub const fn new_rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new(r, g, b, 1.0)
    }
}

impl color32 {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r,
            g,
            b,
            a
        }
    }
    pub const fn new_rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b,
            a: 255
        }
    }
}

impl From<color128> for color32 {
    fn from(a: color128) -> Self {
        Self::new((a.r*255.0) as u8, (a.g*255.0) as u8, (a.b*255.0) as u8, (a.a*255.0) as u8)
    }
}

impl From<color32> for color128 {
    fn from(a: color32) -> Self {
        Self::new(a.r as f32 / 255.0, a.g as f32 / 255.0, a.b as f32 / 255.0, a.a as f32 / 255.0)
    }
}

#[cfg(feature = "prisma")]
pub mod prisma_specific {
    use prisma::Rgba;
    use crate::{color128, color32};

    impl From<Rgba<f32>> for color128 {
        fn from(color: Rgba<f32>) -> Self {
            Self {
                r: color.red(),
                g: color.green(),
                b: color.blue(),
                a: color.alpha()
            }
        }
    }
    impl From<Rgba<u8>> for color32 {
        fn from(color: Rgba<u8>) -> Self {
            Self {
                r: color.red(),
                g: color.green(),
                b: color.blue(),
                a: color.alpha(),
            }
        }
    }
}

#[cfg(feature="palette")]
pub mod pallet_specific {
    use crate::{color128, color32};

    impl From<palette::Rgba<f32>> for color128 {
        fn from(val: palette::Rgba<f32>) -> Self {
            Self {
                r: val.red(),
                g: val.green(),
                b: val.blue(),
                a: val.alpha()
            }
        }
    }

    impl From<palette::Rgba<u8>> for color32 {
        fn from(val: palette::Rgba<u8>) -> Self {
            Self {
                r: val.red(),
                g: val.green(),
                b: val.blue(),
                a: val.alpha()
            }
        }
    }

    impl<T: palette::RgbStandard> From<palette::Alpha<palette::Rgb<T>, f32>> for color128 {
        fn from(color: palette::Alpha<palette::Rgb<T>, f32>) -> Self {
            Self {
                r: color.red,
                g: color.green,
                b: color.blue,
                a: color.alpha
            }
        }
    }
}