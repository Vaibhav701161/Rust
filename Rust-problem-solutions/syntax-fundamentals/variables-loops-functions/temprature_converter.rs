fn convert_temperature(value: f64, from: &str, to: &str) -> f64 {
    match (from, to) {
        ("C", "F") => value * 9.0 / 5.0 + 32.0,
        ("C", "K") => value + 273.15,
        ("F", "C") => (value - 32.0) * 5.0 / 9.0,
        ("F", "K") => (value - 32.0) * 5.0 / 9.0 + 273.15,
        ("K", "C") => value - 273.15,
        ("K", "F") => (value - 273.15) * 9.0 / 5.0 + 32.0,
        _ => value,
    }
}

fn main() {
    println!("30°C to F = {:.2}", convert_temperature(30.0, "C", "F"));
}
