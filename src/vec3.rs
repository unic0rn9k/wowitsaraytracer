use std::mem::transmute;
use std::ops::*;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const fn from_array(arr: [f32; 3]) -> Self {
        unsafe { transmute(arr) }
    }

    pub fn length_square(&self) -> f32 {
        self.iter().map(|n| n.powi(2)).sum()
    }

    pub fn length(&self) -> f32 {
        self.length_square().sqrt()
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.iter().zip(other.iter()).map(|(a, b)| a * b).sum()
    }

    pub fn cross(&self, other: &Self) -> Self {
        // C++ version
        //     return vec3(u.e[1] * v.e[2] - u.e[2] * v.e[1],
        //      u.e[2] * v.e[0] - u.e[0] * v.e[2],
        //      u.e[0] * v.e[1] - u.e[1] * v.e[0]);
        Self {
            x: self[1] * other[2] - self[2] * other[1],
            y: self[2] * other[0] - self[0] * other[2],
            z: self[0] * other[1] - self[1] * other[0],
        }
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub fn to_color(&self) -> u32 {
        (self.z * 255.) as u32 + (((self.y * 255.) as u32) << 8) + (((self.x * 255.) as u32) << 16)
    }
}

impl Deref for Vec3 {
    type Target = [f32; 3];

    fn deref(&self) -> &Self::Target {
        unsafe { transmute(self) }
    }
}

impl DerefMut for Vec3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute(self) }
    }
}

macro_rules! impl_op {
    ($op: tt $trait: ident $fn: ident) => {
        impl $trait<Vec3> for Vec3 {
            type Output = Vec3;

            fn $fn(self, other: Vec3) -> Vec3 {
                Vec3 {
                    x: self.x $op other.x,
                    y: self.y $op other.y,
                    z: self.z $op other.z,
                }
            }
        }

        impl $trait<f32> for Vec3 {
            type Output = Vec3;

            fn $fn(self, other: f32) -> Vec3 {
                Vec3 {
                    x: self.x $op other,
                    y: self.y $op other,
                    z: self.z $op other,
                }
            }
        }

        impl $trait<Vec3> for f32 {
            type Output = Vec3;

            fn $fn(self, other: Vec3) -> Vec3 {
                Vec3 {
                    x: self $op other.x,
                    y: self $op other.y,
                    z: self $op other.z,
                }
            }
        }
    };
}

macro_rules! impl_assign_op {
    ($op: tt $trait: ident $fn: ident) => {
        impl $trait<Vec3> for Vec3 {
            fn $fn(&mut self, other: Self){
                self.x $op other.x;
                self.y $op other.y;
                self.z $op other.z;
            }
        }

        impl $trait<f32> for Vec3 {
            fn $fn(&mut self, other: f32){
                self.x $op other;
                self.y $op other;
                self.z $op other;
            }
        }
	};
}

impl_op!(+ Add add);
impl_op!(- Sub sub);
impl_op!(/ Div div);
impl_op!(* Mul mul);

impl_assign_op!(+= AddAssign add_assign);
impl_assign_op!(-= SubAssign sub_assign);
impl_assign_op!(/= DivAssign div_assign);
impl_assign_op!(*= MulAssign mul_assign);

#[macro_export]
macro_rules! vec3 {
    ($x: expr, $y: expr, $z: expr $(,)?) => {
        Vec3::from_array([($x) as f32, ($y) as f32, ($z) as f32])
    };
    ($fill: expr) => {
        Vec3::from_array([($fill) as f32; 3])
    };
}
