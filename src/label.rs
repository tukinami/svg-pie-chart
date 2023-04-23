use svg::node::{
    element::{Group, Text as TextElement},
    Text as TextNode,
};

use crate::util::{calc_angle_coord, calc_point, normalize_angle};

pub(crate) fn crate_label(
    circle_center: (u32, u32),
    color: (u8, u8, u8),
    size: u32,
    position_radius: u32,
    center_angle: f64,
    _target_angle_range: f64,
    label: &str,
) -> Group {
    let base_color = format!("rgb({}, {}, {})", color.0, color.1, color.2);
    let color_total = color.0 as u32 + color.1 as u32 + color.2 as u32;
    let invert_color = if color_total > (u8::MAX / 2) as u32 {
        "#000"
    } else {
        "#fff"
    };

    let center_angle = normalize_angle(center_angle);
    let center_angle_coord = calc_angle_coord(center_angle);
    let center_angle_point = calc_point(
        center_angle_coord,
        circle_center.0,
        circle_center.1,
        position_radius,
    );
    let text_node = TextNode::new(label);
    let text_base = TextElement::new()
        .set("font-family", "sans-serif")
        .set("font-size", size)
        .set("x", center_angle_point.0)
        .set("y", center_angle_point.1)
        .set("text-anchor", "middle")
        .add(text_node);

    let text_body = text_base.clone().set("fill", base_color);
    let text_under = text_base.set("stroke", invert_color).set("stroke-width", 2);

    Group::new().add(text_under).add(text_body)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod create_label {
        use std::{f64::consts::FRAC_PI_2, path::PathBuf};

        use svg::Document;

        use super::*;

        #[test]
        fn when_top_of_circle_brack() {
            let label = crate_label(
                (50, 50),
                (0, 0, 0),
                10,
                40,
                FRAC_PI_2,
                FRAC_PI_2,
                "BlackLabel",
            );
            let document = Document::new().set("viewBox", "0, 0, 100, 100").add(label);
            let save_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("test_target/test_create_label_top-of-circle-black.svg");
            svg::save(save_path, &document).unwrap();
        }

        #[test]
        fn when_bottom_of_circle_white() {
            let label = crate_label(
                (50, 50),
                (255, 255, 255),
                10,
                40,
                -FRAC_PI_2,
                FRAC_PI_2,
                "WhiteLabel",
            );
            let document = Document::new().set("viewBox", "0, 0, 100, 100").add(label);
            let save_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("test_target/test_create_label_bottom-of-circle-white.svg");
            svg::save(save_path, &document).unwrap();
        }
    }
}
