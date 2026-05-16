use qrcode::{render::svg, QrCode};

pub fn generate_qr_svg(data: &str) -> String {
    let code = QrCode::new(data.as_bytes()).unwrap();

    code.render::<svg::Color>()
        .min_dimensions(180, 180)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build()
}
