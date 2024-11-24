use bevy::{
    prelude::{Mat4, Mesh, Vec3},
    render::render_asset::RenderAssetUsages,
};
use meshtext::{MeshGenerator, MeshText, TextSection};

// text generation function based on the following example:
// https://github.com/FrankenApps/bevy_meshtext_sample
// Copyright (c) 2022 Fichtelcoder

pub fn generate_text(font_data: &'static [u8], text: &str) -> (Mesh, f32) {
    let mut mesh_generator = MeshGenerator::new(font_data);
    let transform = Mat4::from_scale(Vec3::new(1f32, 1f32, 0.1f32)).to_cols_array();
    let text_mesh: MeshText = mesh_generator
        .generate_section(text, false, Some(&transform))
        .unwrap();
    let vertices = text_mesh.vertices;
    let positions: Vec<[f32; 3]> = vertices.chunks(3).map(|c| [c[0], c[1], c[2]]).collect();
    let uvs = vec![[0f32, 0f32]; positions.len()];
    let mut bevy_text_mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );
    bevy_text_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    bevy_text_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    bevy_text_mesh.compute_flat_normals();
    (bevy_text_mesh, text_mesh.bbox.size().x)
}
