use std::f64::consts::{PI, TAU};

use svg::node::element::{path::Data, Circle, ClipPath, Group, Path};

use crate::error::PieChartError;
use crate::util::{
    calc_angle_coord, calc_point, normalize_angle, rotate_perpendicular_positive, Coord,
};

pub(super) fn create_pie(
    circle_center: Coord,
    circle_radius: u32,
    start_angle: f64,
    end_angle: f64,
    center_angle: f64,
    target_angle_range: f64,
    color: &str,
    clip_path_id: &str,
) -> Result<Group, PieChartError> {
    let start_angle = normalize_angle(start_angle);
    let end_angle = normalize_angle(end_angle);
    let center_angle = normalize_angle(center_angle);

    // 角度が無い場合、空のGroupを返す
    if target_angle_range == 0.0 || target_angle_range.abs() < f64::EPSILON {
        return Ok(Group::new());
    }

    let mut circle = Circle::new()
        .set("cx", circle_center.0)
        .set("cy", circle_center.1)
        .set("r", circle_radius)
        .set("fill", color);

    // 円形以上の場合、円を返す。
    if target_angle_range.abs() >= TAU {
        return Ok(Group::new().add(circle));
    }

    // 通常の扇形
    let data_for_clip = create_data_for_clip(
        circle_center,
        circle_radius,
        start_angle,
        end_angle,
        center_angle,
        target_angle_range,
    )?;
    let path_for_clip = Path::new().set("d", data_for_clip);
    let clip_path = ClipPath::new().set("id", clip_path_id).add(path_for_clip);
    circle = circle.set("clip-path", format!("url(#{clip_path_id})"));

    Ok(Group::new().add(clip_path).add(circle))
}

fn create_data_for_clip(
    circle_center: Coord,
    circle_radius: u32,
    start_angle: f64,
    end_angle: f64,
    center_angle: f64,
    target_angle_range: f64,
) -> Result<Data, PieChartError> {
    if target_angle_range > PI {
        create_data_for_major_sector(
            circle_center,
            circle_radius,
            start_angle,
            end_angle,
            center_angle,
        )
    } else {
        create_data_for_minor_sector(
            circle_center,
            circle_radius,
            start_angle,
            end_angle,
            center_angle,
        )
    }
}

fn create_data_for_major_sector(
    (circle_center_x, circle_center_y): Coord,
    circle_radius: u32,
    start_angle: f64,
    end_angle: f64,
    center_angle: f64,
) -> Result<Data, PieChartError> {
    let center_angle_coord = calc_angle_coord(center_angle);

    // 扇形の空いているところを目標とする。
    let target_angle_coord = (-center_angle_coord.0, -center_angle_coord.1);

    // 扇形の空いているところの中心から、接線を計算する。
    let tangent_angle_coord = rotate_perpendicular_positive(target_angle_coord);
    let tangent_angle_expr = ExprKind::new(
        tangent_angle_coord.0,
        tangent_angle_coord.1,
        target_angle_coord.0,
        target_angle_coord.1,
    );

    // 開始点から目標の線と同じ方向に接線を設定する。
    let start_angle_coord = calc_angle_coord(start_angle);
    let start_tangent_angle_expr = ExprKind::new(
        target_angle_coord.0,
        target_angle_coord.1,
        start_angle_coord.0,
        start_angle_coord.1,
    );
    // 接線同士の交点を計算
    let start_tangent_cross_angle = tangent_angle_expr.cross_point(&start_tangent_angle_expr)?;

    // 終了点から目標の線と同じ方向に接線を設定する。
    let end_angle_coord = calc_angle_coord(end_angle);
    let end_tangent_angle_expr = ExprKind::new(
        target_angle_coord.0,
        target_angle_coord.1,
        end_angle_coord.0,
        end_angle_coord.1,
    );
    // 接線同士の交点を計算
    let end_tangent_cross_angle = tangent_angle_expr.cross_point(&end_tangent_angle_expr)?;

    // 開始点方向に接線を半径分伸ばす。
    let tangent_start_side_angle_coord = (
        tangent_angle_coord.0 + target_angle_coord.0,
        tangent_angle_coord.1 + target_angle_coord.1,
    );
    // 終了点方向に接線を半径分伸ばす。
    let tangent_end_side_angle_coord = (
        -tangent_angle_coord.0 + target_angle_coord.0,
        -tangent_angle_coord.1 + target_angle_coord.1,
    );
    // のばした接線に垂直になるような別の接線を作る。
    let half_width_tangent_vector = center_angle_coord;
    let width_tangent_vector = (
        half_width_tangent_vector.0 * 2.0,
        half_width_tangent_vector.1 * 2.0,
    );
    // 開始点側。
    let tangent_from_start_angle_coord = (
        tangent_start_side_angle_coord.0 + width_tangent_vector.0,
        tangent_start_side_angle_coord.1 + width_tangent_vector.1,
    );
    // 終了点側。
    let tangent_from_end_angle_coord = (
        tangent_end_side_angle_coord.0 + width_tangent_vector.0,
        tangent_end_side_angle_coord.1 + width_tangent_vector.1,
    );

    // 実際の座標にする。
    let start_point = calc_point(
        start_angle_coord,
        circle_center_x,
        circle_center_y,
        circle_radius,
    );
    let start_tangent_cross_point = calc_point(
        start_tangent_cross_angle,
        circle_center_x,
        circle_center_y,
        circle_radius,
    );
    let end_point = calc_point(
        end_angle_coord,
        circle_center_x,
        circle_center_y,
        circle_radius,
    );
    let end_tangent_cross_point = calc_point(
        end_tangent_cross_angle,
        circle_center_x,
        circle_center_y,
        circle_radius,
    );
    let tangent_start_side_angle_point = calc_point(
        tangent_start_side_angle_coord,
        circle_center_x,
        circle_center_y,
        circle_radius,
    );
    let tangent_end_side_angle_point = calc_point(
        tangent_end_side_angle_coord,
        circle_center_x,
        circle_center_y,
        circle_radius,
    );
    let tangent_from_start_angle_point = calc_point(
        tangent_from_start_angle_coord,
        circle_center_x,
        circle_center_y,
        circle_radius,
    );
    let tangent_from_end_angle_point = calc_point(
        tangent_from_end_angle_coord,
        circle_center_x,
        circle_center_y,
        circle_radius,
    );

    let data = Data::new()
        .move_to((circle_center_x, circle_center_y))
        .line_to(start_point)
        .line_to(start_tangent_cross_point)
        .line_to(tangent_start_side_angle_point)
        .line_to(tangent_from_start_angle_point)
        .line_to(tangent_from_end_angle_point)
        .line_to(tangent_end_side_angle_point)
        .line_to(end_tangent_cross_point)
        .line_to(end_point)
        .line_to((circle_center_x, circle_center_y))
        .close();

    Ok(data)
}

fn create_data_for_minor_sector(
    (circle_center_x, circle_center_y): Coord,
    circle_radius: u32,
    start_angle: f64,
    end_angle: f64,
    center_angle: f64,
) -> Result<Data, PieChartError> {
    let center_angle_coord = calc_angle_coord(center_angle);

    // 扇形の中心から、接線を計算する。
    let tangent_angle_coord = rotate_perpendicular_positive(center_angle_coord);
    let tangent_angle_expr = ExprKind::new(
        tangent_angle_coord.0,
        tangent_angle_coord.1,
        center_angle_coord.0,
        center_angle_coord.1,
    );

    // 開始点から中心の線と同じ方向に接線を設定する。
    let start_angle_coord = calc_angle_coord(start_angle);
    let start_tangent_angle_expr = ExprKind::new(
        center_angle_coord.0,
        center_angle_coord.1,
        start_angle_coord.0,
        start_angle_coord.1,
    );
    // 接線同士の交点を計算
    let start_tangent_cross_angle = tangent_angle_expr.cross_point(&start_tangent_angle_expr)?;

    // 終了点から中心の線と同じ方向に接線を設定する。
    let end_angle_coord = calc_angle_coord(end_angle);
    let end_tangent_angle_expr = ExprKind::new(
        center_angle_coord.0,
        center_angle_coord.1,
        end_angle_coord.0,
        end_angle_coord.1,
    );
    // 接線同士の交点を計算
    let end_tangent_cross_angle = tangent_angle_expr.cross_point(&end_tangent_angle_expr)?;

    // 実際の座標にする。
    let start_point = calc_point(
        start_angle_coord,
        circle_center_x,
        circle_center_y,
        circle_radius,
    );
    let start_tangent_cross_point = calc_point(
        start_tangent_cross_angle,
        circle_center_x,
        circle_center_y,
        circle_radius,
    );
    let end_point = calc_point(
        end_angle_coord,
        circle_center_x,
        circle_center_y,
        circle_radius,
    );
    let end_tangent_cross_point = calc_point(
        end_tangent_cross_angle,
        circle_center_x,
        circle_center_y,
        circle_radius,
    );

    let data = Data::new()
        .move_to((circle_center_x, circle_center_y))
        .line_to(start_point)
        .line_to(start_tangent_cross_point)
        .line_to(end_tangent_cross_point)
        .line_to(end_point)
        .line_to((circle_center_x, circle_center_y))
        .close();

    Ok(data)
}

#[derive(Debug, PartialEq)]
enum ExprKind {
    Normal(f64, f64),
    ConstantX(f64),
}

impl ExprKind {
    pub fn new(vector_x: f64, vector_y: f64, coord_x: f64, coord_y: f64) -> ExprKind {
        if vector_x == 0.0 {
            return ExprKind::ConstantX(coord_x);
        }
        let coefficient = vector_y / vector_x;
        let constant = coord_y - coord_x * coefficient;
        ExprKind::Normal(coefficient, constant)
    }

    pub fn cross_point(&self, other: &Self) -> Result<(f64, f64), PieChartError> {
        match (self, other) {
            (ExprKind::ConstantX(_), ExprKind::ConstantX(_)) => {
                Err(PieChartError::ParallelVectorsDoNotAcross)
            }
            (
                ExprKind::Normal(lhs_coefficient, lhs_constant),
                ExprKind::Normal(rhs_coefficient, rhs_constant),
            ) => ExprKind::cross_point_between_both_normals(
                *lhs_coefficient,
                *lhs_constant,
                *rhs_coefficient,
                *rhs_constant,
            ),
            (ExprKind::Normal(lhs_coefficient, lhs_constant), ExprKind::ConstantX(x_constant))
            | (ExprKind::ConstantX(x_constant), ExprKind::Normal(lhs_coefficient, lhs_constant)) => {
                ExprKind::cross_poiint_between_normal_and_constant_x(
                    *lhs_coefficient,
                    *lhs_constant,
                    *x_constant,
                )
            }
        }
    }

    fn cross_point_between_both_normals(
        lhs_coefficient: f64,
        lhs_constant: f64,
        rhs_coefficient: f64,
        rhs_constant: f64,
    ) -> Result<(f64, f64), PieChartError> {
        if lhs_coefficient - rhs_coefficient == 0.0 {
            return Err(PieChartError::ParallelVectorsDoNotAcross);
        }

        if lhs_coefficient == 0.0 || rhs_coefficient == 0.0 {
            let (y, simultaneous) = if lhs_coefficient == 0.0 {
                (lhs_constant, (rhs_coefficient, rhs_constant))
            } else {
                (rhs_constant, (lhs_coefficient, lhs_constant))
            };
            let x = (y - simultaneous.1) / simultaneous.0;

            return Ok((x, y));
        }

        let x = (-lhs_constant + rhs_constant) / (lhs_coefficient - rhs_coefficient);
        let y = (lhs_coefficient * x) + lhs_constant;

        Ok((x, y))
    }

    fn cross_poiint_between_normal_and_constant_x(
        lhs_coefficient: f64,
        lhs_constant: f64,
        x_constant: f64,
    ) -> Result<(f64, f64), PieChartError> {
        let x = x_constant;
        let y = (lhs_coefficient * x) + lhs_constant;

        Ok((x, y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI, TAU};

    mod create_pie {
        use std::path::PathBuf;

        use svg::Document;

        use super::*;

        #[test]
        fn success_when_major_sector_pi() {
            let pie =
                create_pie((50, 50), 40, 0.0, PI, FRAC_PI_2, PI, "#fe0033", "test_id").unwrap();
            let document = Document::new().set("viewBox", "0, 0, 100, 100").add(pie);
            let save_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("test_target/test_create_pie_major-PI.svg");

            svg::save(save_path, &document).unwrap();
        }

        #[test]
        fn success_when_major_sector_over_pi() {
            let pie = create_pie(
                (50, 50),
                40,
                0.0,
                PI + FRAC_PI_2,
                (PI + FRAC_PI_2) * 0.5,
                PI + FRAC_PI_2,
                "#fe0033",
                "test_id",
            )
            .unwrap();
            let document = Document::new().set("viewBox", "0, 0, 100, 100").add(pie);
            let save_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("test_target/test_create_pie_major-over-PI.svg");

            svg::save(save_path, &document).unwrap();
        }

        #[test]
        fn success_when_major_sector_pi_accross_0_angle_001() {
            let pie = create_pie(
                (50, 50),
                40,
                TAU - FRAC_PI_2,
                TAU + FRAC_PI_2,
                0.0,
                TAU * 0.5,
                "#fe0033",
                "test_id",
            )
            .unwrap();
            let document = Document::new().set("viewBox", "0, 0, 100, 100").add(pie);
            let save_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("test_target/test_create_pie_major-PI-across-0-angle_001.svg");

            svg::save(save_path, &document).unwrap();
        }

        #[test]
        fn success_when_major_sector_pi_accross_0_angle_002() {
            let pie = create_pie(
                (50, 50),
                40,
                FRAC_PI_2 + (TAU * 0.5),
                FRAC_PI_2 + (TAU * 0.5) + (TAU * 0.5),
                FRAC_PI_2 + (TAU * 0.5) + (TAU * 0.5 * 0.5),
                TAU * 0.5,
                "#fe0033",
                "test_id",
            )
            .unwrap();
            let document = Document::new().set("viewBox", "0, 0, 100, 100").add(pie);
            let save_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("test_target/test_create_pie_major-PI-across-0-angle_002.svg");

            svg::save(save_path, &document).unwrap();
        }

        #[test]
        fn success_when_major_sector_across_0_angle() {
            let pie = create_pie(
                (50, 50),
                40,
                PI + FRAC_PI_4,
                PI - FRAC_PI_4,
                0.0,
                PI + FRAC_PI_2,
                "#fe0033",
                "test_id",
            )
            .unwrap();
            let document = Document::new().set("viewBox", "0, 0, 100, 100").add(pie);
            let save_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("test_target/test_create_pie_major-accross_0_angle.svg");

            svg::save(save_path, &document).unwrap();
        }

        #[test]
        fn success_when_major_sector_approximate_pi() {
            let pie = create_pie(
                (50, 50),
                40,
                0.0,
                PI + 0.1,
                (PI + 0.1) * 0.5,
                PI + 0.1,
                "#fe0033",
                "test_id",
            )
            .unwrap();
            let document = Document::new().set("viewBox", "0, 0, 100, 100").add(pie);
            let save_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("test_target/test_create_pie_major-approximate-PI.svg");

            svg::save(save_path, &document).unwrap();
        }

        #[test]
        fn success_when_minor_sector_approximate_pi() {
            let pie = create_pie(
                (50, 50),
                40,
                0.0,
                PI - f64::EPSILON,
                (PI - f64::EPSILON) * 0.5,
                PI - f64::EPSILON,
                "#fe0033",
                "test_id",
            )
            .unwrap();
            let document = Document::new().set("viewBox", "0, 0, 100, 100").add(pie);
            let save_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("test_target/test_create_pie_minor-approximate-PI.svg");

            svg::save(save_path, &document).unwrap();
        }

        #[test]
        fn success_when_minor_sector_normal() {
            let pie = create_pie(
                (50, 50),
                40,
                0.0,
                FRAC_PI_2,
                FRAC_PI_4,
                FRAC_PI_2,
                "#fe0033",
                "test_id",
            )
            .unwrap();
            let document = Document::new().set("viewBox", "0, 0, 100, 100").add(pie);
            let save_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("test_target/test_create_pie_minor-normal.svg");

            svg::save(save_path, &document).unwrap();
        }

        #[test]
        fn success_when_minor_across_0_angle() {
            let pie = create_pie(
                (50, 50),
                40,
                TAU - FRAC_PI_4,
                FRAC_PI_4,
                TAU,
                FRAC_PI_2,
                "#fe0033",
                "test_id",
            )
            .unwrap();
            let document = Document::new().set("viewBox", "0, 0, 100, 100").add(pie);
            let save_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("test_target/test_create_pie_minor-across_0_angle.svg");

            svg::save(save_path, &document).unwrap();
        }
    }

    mod enum_kind {
        use super::*;

        mod new {
            use super::*;

            #[test]
            fn checking_value() {
                assert_eq!(
                    ExprKind::new(1.0, 0.0, 0.0, 0.0),
                    ExprKind::Normal(0.0, 0.0)
                );
                assert_eq!(ExprKind::new(0.0, 1.0, 1.0, 0.0), ExprKind::ConstantX(1.0));
                assert_eq!(
                    ExprKind::new(1.0, 1.0, 1.0, 1.0),
                    ExprKind::Normal(1.0, 0.0)
                );
                assert_eq!(
                    ExprKind::new(1.0, 1.0, 0.0, 1.0),
                    ExprKind::Normal(1.0, 1.0)
                );
            }
        }

        mod cross_point {
            use super::*;

            #[test]
            fn failed_when_both_constant_x() {
                let lhs = ExprKind::ConstantX(1.0);
                let rhs = ExprKind::ConstantX(0.0);
                assert!(lhs.cross_point(&rhs).is_err());
            }

            #[test]
            fn success_when_both_normal_and_crossing() {
                let lhs = ExprKind::Normal(1.0, 1.0);
                let rhs = ExprKind::Normal(0.0, 1.0);
                let result = lhs.cross_point(&rhs).unwrap();
                assert_eq!(result, (0.0, 1.0));
            }

            #[test]
            fn failed_when_both_normal_and_not_crossing() {
                let lhs = ExprKind::Normal(1.0, 1.0);
                let rhs = ExprKind::Normal(1.0, 0.0);
                assert!(lhs.cross_point(&rhs).is_err());
            }

            #[test]
            fn success_when_normal_and_constant_x() {
                let lhs = ExprKind::Normal(1.0, 1.0);
                let rhs = ExprKind::ConstantX(0.5);
                let result = lhs.cross_point(&rhs).unwrap();
                assert_eq!(result, (0.5, 1.5));
            }
        }
    }
}
