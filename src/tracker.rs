extern crate nalgebra as na;
use crate::serialization::serde_helpers::*;
use error_stack::{IntoReport, ResultExt};
use na::Vector3;
use std::{error::Error, fmt};
#[derive(Debug)]
pub struct TrackerSetupError;

impl fmt::Display for TrackerSetupError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Could not setup tracker entity correctly")
    }
}

//impl Context for ParseConfigError {}
impl Error for TrackerSetupError {}

pub struct TrackingTarget {
    position: i32,
    id: i32,
    checkbox_size: Vec<f32>,
    target_vector: Vector3<f32>,
    target_rotation: Vector3<f32>,
}

impl TrackingTarget {
    pub fn new(p: i32, id: i32) -> TrackingTarget {
        TrackingTarget {
            position: (p),
            id: (0),
            checkbox_size: ([0.0, 0.0, 0.0].to_vec()),
            target_vector: (Vector3::new(0.0, 0.0, 0.0)),
            target_rotation: (Vector3::new(0.0, 0.0, 0.0)),
        }
    }
    pub fn new_from_file(path: &str) -> error_stack::Result<TrackingTarget, TrackerSetupError> {
        let p = deserialize_into_config(path)
            .change_context(TrackerSetupError)
            .attach_printable_lazy(|| format!("Couldn't deserialize json from {path:?}"))?;

        Ok(TrackingTarget {
            position: (0),
            id: (1),
            checkbox_size: ([0.0, 0.0, 0.0].to_vec()),
            target_vector: (Vector3::new(0.0, 0.0, 0.0)),
            target_rotation: (Vector3::new(0.0, 0.0, 0.0)),
        })
    }

    fn set_approx_radius_generic(&self, r: f32) -> bool {
        return true;
    }
    fn update_trackingdata(&mut self, v: Vector3<f32>, r: Vector3<f32>) -> bool {
        self.target_vector = v;
        self.target_rotation = r;
        return true;
    }

    fn is_target_reached(&self, y: Vector3<f32>, z: Vector3<f32>) -> bool {
        let mut ir = 0;
        let mut iv = 0;

        for n in 0..=2 {
            if (y[n] >= self.target_vector[n] - self.checkbox_size[n])
                && (y[n] <= self.target_vector[n] + self.checkbox_size[n])
            {
                iv = iv + 1;
            }
            if (z[n] >= self.target_vector[n] - self.checkbox_size[n])
                && (z[n] <= self.target_vector[n] + self.checkbox_size[n])
            {
                ir = ir + 1
            }
        }
        if (ir == 3) && (iv == 3) {
            return true;
        } else {
            return false;
        }
    }
}

#[cfg(test)]
#[test]
fn test_is_target_reached() {
    let v = Vector3::new(5.0, 5.0, 5.0);
    let r = Vector3::new(5.0, 5.0, 5.0);

    let mut t1 = TrackingTarget::new(0, 0);
    t1.update_trackingdata(v, r);
    assert_eq!(true, t1.is_target_reached(v, r));
}

#[test]
fn test_add_checkbox() {
    let v = Vector3::new(5.0, 5.0, 5.0);
    let r = Vector3::new(5.0, 5.0, 5.0);

    let mut t1 = TrackingTarget::new(0, 0);
    t1.set_approx_radius_generic(3.0);
    t1.update_trackingdata(v, r);
    assert_eq!(true, t1.is_target_reached(v, r));
}

#[test]
fn test_filebased_tracking() {
    let v = Vector3::new(5.0, 5.0, 5.0);
    let r = Vector3::new(5.0, 5.0, 5.0);

    let mut t1 = TrackingTarget::new_from_file(r"C:\Users\sta\source\rust\testbe-rs\settings.json")
        .expect("Test File corrupt!");
    t1.set_approx_radius_generic(3.0);
    t1.update_trackingdata(v, r);
    assert_eq!(true, t1.is_target_reached(v, r));
}
