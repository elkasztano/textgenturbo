use bevy::{
    math::Vec2,
    prelude::Image as BevyImage,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};
use colorgrad::{Color as GradColor, Gradient, GradientBuilder, LinearGradient};
use noise::NoiseFn;

pub fn gen_normal_basic_multi(scale: f64, seed: u32, dims: (u32, u32)) -> BevyImage {
    let grad = GradientBuilder::new()
        .colors(&[
            GradColor::from((0.1, 0.1, 0.1)),
            GradColor::from((0.3, 0.3, 0.3)),
        ])
        .build::<LinearGradient>()
        .unwrap();
    let noise = noise::BasicMulti::<noise::Value>::new(seed);
    let mut imgbuf = image::ImageBuffer::new(dims.0, dims.1);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let t = noise.get([x as f64 * scale, y as f64 * scale]);
        let mut rgba = grad.at(t as f32 + 0.5).to_rgba8();
        // modify alpha channel
        rgba[3] = distance_converted((x, y), (dims.0 / 2, dims.1 / 2), (1.0, 0.0));
        *pixel = image::Rgba(rgba);
    }

    BevyImage::new_fill(
        Extent3d {
            width: dims.0,
            height: dims.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &imgbuf,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}

// distance function for alpha channel, saturates at 0 and 255
fn distance_converted(a: (u32, u32), b: (u32, u32), bias: (f32, f32)) -> u8 {
    let vec2_a = Vec2::new(a.0 as f32, a.1 as f32);
    let vec2_b = Vec2::new(b.0 as f32, b.1 as f32);
    let dist = vec2_a.distance(vec2_b) * bias.0 + bias.1;
    (511.0 - dist) as u8
}
