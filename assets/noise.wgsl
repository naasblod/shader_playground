#define_import_path shader_playground::noise

// 2D Random
fn random(st: vec2<f32>) -> f32 {
    return fract(sin(dot(st.xy, vec2(12.9898, 78.233))) * 43758.5453123);
}


// 2D Noise based on Morgan McGuire @morgan3d
// https://www.shadertoy.com/view/4dS3Wd
fn noise(st: vec2<f32>) -> f32 {
    let i = floor(st);
    let f = fract(st);

    // Four corners in 2D of a tile
    let a = random(i);
    let b = random(i + vec2(1.0, 0.0));
    let c = random(i + vec2(0.0, 1.0));
    let d = random(i + vec2(1.0, 1.0));

    // Smooth Interpolation

    // Cubic Hermine Curve.  Same as SmoothStep()
    let u = f * f * (3.0 - 2.0 * f);
    // u = smoothstep(0.,1.,f);

    // Mix 4 coorners percentages
    return mix(a, b, u.x) + (c - a) * u.y * (1.0 - u.x) + (d - b) * u.x * u.y;
}
