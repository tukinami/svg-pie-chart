//! A function to create pie chart by svg.
//!
//! This function returns pie chart made by `Document` of [`svg`].
//!
//! [`svg`]: https://github.com/bodoni/svg
use std::{f64::consts::FRAC_PI_2, f64::consts::TAU};

use svg::node::element::Group;
use util::normalize_angle;

pub mod error;
mod label;
mod pie;
pub(crate) mod util;

pub use svg::Document;

/// Creates pie chart.
///
/// This function returns pie chart made by `Document` of [`svg`].
///
/// The `width`, `height`, `circle_radius`, `label_size`, and `label_position_radius` are in pixels.
/// The `label_color` is RGB ((0 ~ 255) * 3).
///
/// The 1st of `pie_statuses` element is a label text.
/// The 2nd of `pie_statuses` element is a ratio (0.0 ~ 1.0).
/// The 3rd of `pie_statuses` element is a color of the pie (CSS style).
///
///
/// # Examples
///
/// ```
/// use svg_pie_chart::create_pie_chart;
///
/// // (label, ratio, color)
/// let case = [
///     ("Red", 0.5, "#fe5555"),
///     ("Green", 0.10, "#55fe55"),
///     ("Blue", 0.25, "#3366fe"),
///     ("Other", 0.15, "#999"),
/// ];
///
/// let pie_chart = create_pie_chart(
///     100,         // width
///     100,         // height
///     40,          // radius of circle
///     (0, 0, 0),   // color of label
///     "sans-serif",// font-family of label
///     10,          // size of label
///     20,          // radius of label's position
///     &case        // statuses of pies
/// );
///
/// assert!(pie_chart.is_ok());
/// ```
///
/// [`svg`]: https://github.com/bodoni/svg
pub fn create_pie_chart<S, T, R>(
    width: u32,
    height: u32,
    circle_radius: u32,
    label_color: (u8, u8, u8),
    label_font: S,
    label_size: u32,
    label_position_radius: u32,
    pie_statuses: &[(T, f64, R)],
) -> Result<Document, error::PieChartError>
where
    S: AsRef<str>,
    T: AsRef<str>,
    R: AsRef<str>,
{
    let mut document = Document::new().set("viewBox", format!("0, 0, {width}, {height}"));
    let circle_center = (width / 2, height / 2);

    let mut pie_group = Group::new();
    let mut label_group = Group::new();

    let mut base_angle = FRAC_PI_2;
    for (i, (label, ratio, pie_color)) in pie_statuses.iter().enumerate() {
        let target_angle_range = TAU * ratio;
        let start_angle = base_angle;
        let end_angle = base_angle - target_angle_range;
        let center_angle = base_angle - (target_angle_range * 0.5);

        let pie = pie::create_pie(
            circle_center,
            circle_radius,
            start_angle,
            end_angle,
            center_angle,
            target_angle_range,
            pie_color.as_ref(),
            &format!("p_{i}"),
        )?;
        pie_group = pie_group.add(pie);

        let label = label::crate_label(
            circle_center,
            label_color,
            &label_font,
            label_size,
            label_position_radius,
            center_angle,
            target_angle_range,
            label.as_ref(),
        );
        label_group = label_group.add(label);

        base_angle -= target_angle_range;
        base_angle = normalize_angle(base_angle);
    }

    document = document.add(pie_group).add(label_group);

    Ok(document)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod create_pie_chart {
        use std::path::PathBuf;

        use super::*;

        #[test]
        fn success_when_valid_data_001() {
            let case = vec![
                ("Red", 0.5, "#fe5555"),
                ("Green", 0.10, "#55fe55"),
                ("Blue", 0.25, "#3366fe"),
                ("Other", 0.15, "#999"),
            ];

            let document =
                create_pie_chart(100, 100, 40, (0, 0, 0), "游ゴシック", 10, 20, &case).unwrap();
            let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("test_target/test_create_pie_chart_001.svg");
            svg::save(path, &document).unwrap();
        }

        #[test]
        fn success_when_valid_data_002() {
            let case = vec![("Red", 0.5, "#fe5555")];

            let document = create_pie_chart(
                200,
                200,
                40,
                (255, 255, 255),
                "ＭＳ ゴシック",
                10,
                50,
                &case,
            )
            .unwrap();
            let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("test_target/test_create_pie_chart_002.svg");
            svg::save(path, &document).unwrap();
        }
    }
}
