pub mod inverse_kinematics {

    extern crate libm;
    extern crate nalgebra as na;

    use std::f64::{consts::PI, NAN};

    use libm::{atan, cos, pow, sin, sqrt};
    use na::Vector3;

    #[derive(Debug, PartialEq, Eq)]
    pub enum Direction {
        Clockwise,
        Counterclockwise,
    }

    fn calculate_vector_fx(v1: Vector3<f32>, v2: Vector3<f32>) -> (f32, f32) {
        let m = (v1[1] - v2[1]) / (v1[0] - v2[0]);
        let b = v2[1] - v2[0] * m;
        return (m, b);
    }

    fn calculate_intersection(a: f32, b: f32, m: f32, u: f32) -> Vector3<f32> {
        let mut i = Vector3::new(1.0, 1.0, 1.0);
        i[0] = (u - b) / (a - m);
        i[1] = a * i[0] + b;
        i[2] = 0.0;
        return i;
    }

    pub fn simple_ik(v1: Vector3<f64>) -> Vec<Vector3<f64>> {
        let mut vec = Vec::new();
        let num_effectors = 2.0;
        let sizeof_arm = 4.0;
        let e_r = v1.clone().to_owned(); //Vector3::new(5.0, 5.0, 0.0);
                                         // Fehlerfaelle pruefen:
        let magnitude =
            libm::sqrt(libm::pow(e_r[0], 2.0) + libm::pow(e_r[1], 2.0) + libm::pow(e_r[2], 2.0));
        if magnitude > num_effectors * sizeof_arm {
            return vec;
        } else {
            // Haelfte vom ZielVektor berechnen:
            let alpha = atan(e_r[1] / e_r[0]);

            // Normalvektor berechnen:
            let sizeof_n = sqrt(pow(sizeof_arm, 2.0) - pow(e_r.norm() / 2.0, 2.0));
            // Jetzt den Winkel zwischen effektor1 und x Achse berechen

            let beta = alpha + (sizeof_n / (e_r.norm() / 2.0));
            // Koordinaten vom Schnittpunkt des normalenvektors und effektor1:
            let mut e_1 = Vector3::new(1.0, 1.0, 1.0);
            e_1[0] = cos(beta) * sizeof_arm;
            e_1[1] = sin(beta) * sizeof_arm;
            vec.push(e_1);

            // Vom Schnittpunkt aus kann der nächste Effektor bestimmt werden:
            let effektor2 = e_r - e_1;
            vec.push(effektor2);
            // TCP wurde bereits übergeben! Nix mehr zu tun!
        }
        return vec;
    }

    /*
       Calculate no of steps to advance to v2 from v2.
       Provide micro-steps per revolution ( param: spr)
    */
    pub fn coordinates_to_steps(v1: Vector3<f64>, v2: Vector3<f64>, spr: i64) -> (i64, Direction) {
        // Map vector into circle, where zero radians equals to v1.
        let theta = libm::acos(v1.dot(&v2) / (v1.len() * v2.len()) as f64);
        let mut dir = Direction::Clockwise;
        if theta == NAN {
            panic!("Vectors are not connected");
        } else if theta < PI {
            dir = Direction::Clockwise;
        } else {
            dir = Direction::Counterclockwise;
        }
        println!(
            "theta={},dot={},len={}",
            theta,
            v1.dot(&v2),
            (v1.len() * v2.len()) as f64
        );
        let steps = theta / (2.0 * PI) * spr as f64;
        (steps as i64, dir)
    }

    #[cfg(test)]
    #[test]
    fn test_calculate_vector_fx() {
        let t1 = Vector3::new(0.0, 0.0, 0.0);
        let t2 = Vector3::new(5.0, 5.0, 5.0);
        let t3 = Vector3::new(0.0, 5.0, 0.0);
        let t4 = Vector3::new(5.0, 0.0, 0.0);

        assert_eq!(calculate_vector_fx(t1, t2), (1.0, 0.0));
        assert_eq!(calculate_vector_fx(t3, t4), (-1.0, 5.0));
        // Also catch inf and NaN!
        //assert_eq!(calculate_vector_fx(t3,t1),(-1.0,5.0));
    }

    #[test]
    fn test_calculate_intersection() {
        let t1 = Vector3::new(0.0, 0.0, 0.0);
        let t2 = Vector3::new(5.0, 5.0, 5.0);
        let t3 = Vector3::new(0.0, 5.0, 0.0);
        let t4 = Vector3::new(5.0, 0.0, 0.0);
        let t5 = Vector3::new(2.5, 2.5, 0.0);

        let fx1 = calculate_vector_fx(t1, t2);
        let fx2 = calculate_vector_fx(t3, t4);

        assert_eq!(calculate_intersection(fx1.0, fx1.1, fx2.0, fx2.1), t5);
    }
    #[test]
    fn test_simple_ik() {
        let t1 = Vector3::new(0.0, 0.0, 0.0);
        let t2 = Vector3::new(5.0, 5.0, 0.0);
        let t3 = Vector3::new(0.0, 5.0, 0.0);
        let t4 = Vector3::new(5.0, 0.0, 0.0);
        let t5 = Vector3::new(2.5, 2.5, 0.0);
        let v = simple_ik(t2);
        assert_eq!(v[0] + v[1], t2);
    }

    #[test]
    fn test_coordinates_to_steps() {
        let t1 = Vector3::new(0.0, 5.0, 0.0);
        let t2 = Vector3::new(5.0, 0.0, 0.0);

        let s1 = coordinates_to_steps(t1, t2, 500);
        assert_eq!(s1, (125, Direction::Clockwise));
    }

    #[test]
    #[should_panic]
    fn test_coordinates_to_steps_panic() {
        let t1 = Vector3::new(0.0, 5.0, 0.0);
        let t2 = Vector3::new(5.0, 5.0, 0.0);
        let s1 = coordinates_to_steps(t1, t2, 500);
    }
}
