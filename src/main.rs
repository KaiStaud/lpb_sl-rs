#[macro_use]
extern crate nalgebra as na;
mod inverse_kinematics;

use na::{Vector3, Rotation3};
fn main() {
    let axis  = Vector3::x_axis();
    let angle = 1.57;
    let b     = Rotation3::from_axis_angle(&axis, angle);
    let t1=Vector3::new(0.0, 0.0, 0.0);
    let t2=Vector3::new(5.0, 5.0, 5.0);
    let t3=Vector3::new(0.0, 5.0, 0.0);
    let t4=Vector3::new(5.0, 0.0, 0.0);
    let t5=Vector3::new(2.5, 2.5, 0.0);
    let v = inverse_kinematics::inverse_kinematics::simple_ik(t2);
    print!("{:?}",v);
}