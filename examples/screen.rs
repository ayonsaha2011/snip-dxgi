use image;

use snip::screen_capture;

fn main() {
    let s = screen_capture(0).unwrap();

    println!(
        "{} x {} x {} = {} bytes",
        s.height(),
        s.width(),
        s.pixel_width(),
        s.raw_len()
    );

    let origin = s.get_pixel(0, 0);
    println!("(0,0): R: {}, G: {}, B: {}", origin.r, origin.g, origin.b);

    let end_col = s.get_pixel(0, s.width() - 1);
    println!(
        "(0,end): R: {}, G: {}, B: {}",
        end_col.r, end_col.g, end_col.b
    );

    let opp = s.get_pixel(s.height() - 1, s.width() - 1);
    println!("(end,end): R: {}, G: {}, B: {}", opp.r, opp.g, opp.b);

    let mut buf = vec![];
    repng::encode(&mut buf, s.width() as u32, s.height() as u32, &s.get_data()).unwrap();
    // println!("data:image/png;base64,{}", base64::encode(buf));

    image::save_buffer(
        "test.png",
        s.as_ref(),
        s.width() as u32,
        s.height() as u32,
        image::ColorType::Rgba8,
    )
    .unwrap();
}
