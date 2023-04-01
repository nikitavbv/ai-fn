use {
    ai_fn_macro::ai_fn,
};

fn main() {
    let radians = degrees_to_radians(42.0);
    println!("radians: {}", radians);
}

#[ai_fn("convert degress to radians")]
fn degrees_to_radians(degress: f64) -> f64;