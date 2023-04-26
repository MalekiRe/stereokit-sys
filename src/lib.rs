#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

unsafe impl Sync for _material_t {}
unsafe impl Send for _material_t {}
unsafe impl Sync for _model_t {}
unsafe impl Send for _model_t {}
unsafe impl Sync for _tex_t {}
unsafe impl Send for _tex_t {}
unsafe impl Sync for _sound_t {}
unsafe impl Send for _sound_t {}
unsafe impl Sync for _material_buffer_t {}
unsafe impl Send for _material_buffer_t {}
unsafe impl Sync for _sprite_t {}
unsafe impl Send for _sprite_t {}
unsafe impl Sync for _font_t {}
unsafe impl Send for _font_t {}
unsafe impl Sync for _gradient_t {}
unsafe impl Send for _gradient_t {}
unsafe impl Sync for _shader_t {}
unsafe impl Send for _shader_t {}

impl From<glam::Vec3> for vec3 {
	fn from(val: glam::Vec3) -> Self {
		Self {
			x: val.x,
			y: val.y,
			z: val.z,
		}
	}
}
impl Into<glam::Vec3> for vec3 {
	fn into(self) -> glam::Vec3 {
		match self {
			vec3 { x, y, z } => glam::Vec3{ x, y, z}
		}
	}
}

impl From<glam::Vec2> for vec2 {
	fn from(val: glam::Vec2) -> Self {
		Self { x: val.x, y: val.y }
	}
}
impl Into<glam::Vec2> for vec2 {
	fn into(self) -> glam::Vec2 {
		match self {
			vec2 { x, y } => glam::Vec2{ x, y }
		}
	}
}

impl From<glam::Vec4> for vec4 {
	fn from(val: glam::Vec4) -> Self {
		Self {
			x: val.x,
			y: val.y,
			z: val.z,
			w: val.w,
		}
	}
}
impl Into<glam::Vec4> for vec4 {
	fn into(self) -> glam::Vec4 {
		match self {
			vec4 { x, y, z, w } => glam::Vec4::new(x, y, z, w)
		}
	}
}

impl From<glam::Mat4> for matrix {
	fn from(m: glam::Mat4) -> Self {
		matrix {
			row: [m.x_axis.into(), m.y_axis.into(), m.z_axis.into(), m.w_axis.into()],
		}
	}
}

impl Into<glam::Mat4> for matrix {
	fn into(self) -> glam::Mat4 {
		unsafe {
			//This should not work, but it does work, do not ask me why, god only knows.
			#[allow(unreachable_patterns)]
			// match self {
			// 	matrix { row: r } => ColumnMatrix4::from(RowMatrix4::from([
			// 		r[0].x, r[0].y, r[0].z, r[0].w, r[1].x, r[1].y, r[1].z, r[1].w, r[2].x, r[2].y,
			// 		r[2].z, r[2].w, r[3].x, r[3].y, r[3].z, r[3].w,
			// 	])),
			// 	matrix { m: ma } => ColumnMatrix4::from(ma),
			// }
			match self {
				matrix { row: r } => {
					glam::Mat4::from_cols(r[0].into(), r[1].into(), r[2].into(), r[3].into()).transpose()
				}
			}
		}
	}
}

impl From<glam::Quat> for quat {
	fn from(val: glam::Quat) -> Self {
		quat {
			x: val.x,
			y: val.y,
			z: val.z,
			w: val.w,
		}
	}
}

impl color128 {
	pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
		Self { r, g, b, a }
	}
	pub const fn new_rgb(r: f32, g: f32, b: f32) -> Self {
		Self::new(r, g, b, 1.0)
	}
}

impl color32 {
	pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
		Self { r, g, b, a }
	}
	pub const fn new_rgb(r: u8, g: u8, b: u8) -> Self {
		Self { r, g, b, a: 255 }
	}
}

impl From<color128> for color32 {
	fn from(a: color128) -> Self {
		Self::new(
			(a.r * 255.0) as u8,
			(a.g * 255.0) as u8,
			(a.b * 255.0) as u8,
			(a.a * 255.0) as u8,
		)
	}
}
impl From<[f32; 4]> for color128 {
	fn from(s: [f32; 4]) -> Self {
		Self::new(s[0], s[1], s[2], s[3])
	}
}

impl From<color32> for color128 {
	fn from(a: color32) -> Self {
		Self::new(
			a.r as f32 / 255.0,
			a.g as f32 / 255.0,
			a.b as f32 / 255.0,
			a.a as f32 / 255.0,
		)
	}
}
impl From<[u8; 4]> for color32 {
	fn from(s: [u8; 4]) -> Self {
		Self::new(s[0], s[1], s[2], s[3])
	}
}

#[cfg(feature = "prisma")]
pub mod prisma_specific {
	use crate::{color128, color32};
	use prisma::Rgba;

	impl From<Rgba<f32>> for color128 {
		fn from(color: Rgba<f32>) -> Self {
			Self {
				r: color.red(),
				g: color.green(),
				b: color.blue(),
				a: color.alpha(),
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

#[cfg(feature = "palette")]
pub mod pallet_specific {
	use crate::{color128, color32};

	impl From<palette::Rgba<f32>> for color128 {
		fn from(val: palette::Rgba<f32>) -> Self {
			Self {
				r: val.red(),
				g: val.green(),
				b: val.blue(),
				a: val.alpha(),
			}
		}
	}

	impl From<palette::Rgba<u8>> for color32 {
		fn from(val: palette::Rgba<u8>) -> Self {
			Self {
				r: val.red(),
				g: val.green(),
				b: val.blue(),
				a: val.alpha(),
			}
		}
	}

	impl<T: palette::RgbStandard> From<palette::Alpha<palette::Rgb<T>, f32>> for color128 {
		fn from(color: palette::Alpha<palette::Rgb<T>, f32>) -> Self {
			Self {
				r: color.red,
				g: color.green,
				b: color.blue,
				a: color.alpha,
			}
		}
	}
}
