pub fn hsv_to_rgb(h: f64) -> u32 {
    let s = 1.0;
    let v = 1.0;
    let c = s * v;
    let x = c * (1.0 - f64::abs((h / (u8::MAX / 6) as f64) % 2.0 - 1.0));
    let m = v - c;
    let bound = u8::MAX as f64;
    let (r, g, b) = if h < bound / 6.0 {
        (c, x, 0.0)
    } else if h < bound / 3.0 {
        (c, x, 0.0)
    } else if h < bound / 2.0 {
        (0.0, c, x)
    } else if h < (bound / 3.0) * 2.0 {
        (0.0, x, c)
    } else if h < (bound / 6.0) * 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    let (r, g, b) = ((r + m) * 255.0, (g + m) * 255.0, (b + m) * 255.0);
    (b as u32) | ((g as u32) << 8) | ((r as u32) << 16)
}
