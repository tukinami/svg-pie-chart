use std::f64::consts::TAU;

pub(crate) type Coord = (u32, u32);

#[inline]
pub(crate) fn normalize_angle(angle: f64) -> f64 {
    if angle < 0.0 {
        TAU + angle
    } else if angle >= TAU {
        angle - TAU
    } else {
        angle
    }
}

#[inline]
pub(crate) fn calc_angle_coord(angle: f64) -> (f64, f64) {
    let sin_cos = angle.sin_cos();
    (sin_cos.1, sin_cos.0)
}

#[inline]
pub(crate) fn rotate_perpendicular_positive((x, y): (f64, f64)) -> (f64, f64) {
    (-y, x)
}

pub(crate) fn calc_point(
    (angle_x, angle_y): (f64, f64),
    circle_center_x: u32,
    circle_center_y: u32,
    circle_radius: u32,
) -> (f64, f64) {
    let relative_x = angle_x * circle_radius as f64;
    let relative_y = -angle_y * circle_radius as f64;

    let absolute_x = relative_x + circle_center_x as f64;
    let absolute_y = relative_y + circle_center_y as f64;

    (absolute_x, absolute_y)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI, TAU};

    mod normalize_angle {
        use super::*;

        #[test]
        fn checking_value() {
            assert_eq!(normalize_angle(-FRAC_PI_4), TAU - FRAC_PI_4);
            assert_eq!(normalize_angle(TAU + FRAC_PI_4), FRAC_PI_4);
            assert_eq!(normalize_angle(FRAC_PI_4), FRAC_PI_4);
            assert_eq!(normalize_angle(TAU), 0.0);
            assert_eq!(
                normalize_angle(FRAC_PI_2 + (TAU * 0.5) + (TAU * 0.5 * 0.5)),
                0.0
            );
        }
    }

    mod calc_angle_coord {
        use super::*;

        #[test]
        fn checking_value() {
            assert_eq!(calc_angle_coord(0.0), (1.0, 0.0));

            let (x, y) = calc_angle_coord(FRAC_PI_2);
            assert!(x.abs() > 0.0 && x.abs() < f64::EPSILON);
            assert_eq!(y, 1.0);

            let (x, y) = calc_angle_coord(PI);
            assert_eq!(x, -1.0);
            assert!(y.abs() > 0.0 && y.abs() < f64::EPSILON);

            let (x, y) = calc_angle_coord(PI + FRAC_PI_2);
            assert!(x.abs() > 0.0 && x.abs() < f64::EPSILON);
            assert_eq!(y, -1.0);

            // 誤差大
            // let (x, y) = calc_angle_coord(TAU);
            // assert_eq!(x, 1.0);
            // assert!(y.abs() > 0.0 && y.abs() < f64::EPSILON);
        }
    }

    mod rotate_perpendicular_positive {
        use super::*;

        #[test]
        fn checking_value() {
            assert_eq!(rotate_perpendicular_positive((1.0, 0.0)), (0.0, 1.0));
        }
    }

    mod calc_point {
        use super::*;

        #[test]
        fn checking_value() {
            let angles = (0.0, 1.0);
            let circle_center_x = 3;
            let circle_center_y = 4;
            let circle_radius = 2;

            assert_eq!(
                calc_point(angles, circle_center_x, circle_center_y, circle_radius),
                (3.0, 2.0)
            );

            let angles = (-1.0, 0.0);
            let circle_center_x = 3;
            let circle_center_y = 4;
            let circle_radius = 2;

            assert_eq!(
                calc_point(angles, circle_center_x, circle_center_y, circle_radius),
                (1.0, 4.0)
            );

            let angles = (0.0, -1.0);
            let circle_center_x = 3;
            let circle_center_y = 4;
            let circle_radius = 2;

            assert_eq!(
                calc_point(angles, circle_center_x, circle_center_y, circle_radius),
                (3.0, 6.0)
            );

            let angles = (1.0, 0.0);
            let circle_center_x = 3;
            let circle_center_y = 4;
            let circle_radius = 2;

            assert_eq!(
                calc_point(angles, circle_center_x, circle_center_y, circle_radius),
                (5.0, 4.0)
            );
        }
    }
}
