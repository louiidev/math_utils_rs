

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    (1. - t) * a + b * t 
}

fn inverse_lerp(a: f32, b: f32, v: f32) -> f32 {
    (v - a) / (b - a)
}

fn remap(i: (f32, f32), o: (f32, f32), v: f32) -> f32 {
    let t = inverse_lerp(i.0, i.1, v);
    lerp(o.0, o.1, t)
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
