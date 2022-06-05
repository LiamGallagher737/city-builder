use bevy::{prelude::{Mesh, Vec3}, render::mesh::{Indices, VertexAttributeValues}};
use super::components::Node;

const ROAD_WIDTH: f32 = 3.5;

pub fn generate_road_mesh (
    points: &Vec<Node>,
) -> Mesh {

    let mut vertices: Vec::<[f32; 3]> = Vec::new();
    let mut triangles: Vec::<u16> = Vec::new();
    let mut uvs: Vec::<[f32; 2]> = Vec::new();

    let mut vert_index = 0;

    for i in 0..points.len() {

        let mut forward = Vec3::default();

        if i < points.len() - 1 {
            forward += points[i + 1].position - points[i].position;
        }

        if i > 0 {
            forward += points[i].position - points[i - 1].position;
        }

        forward = forward.normalize();
        let left = Vec3::new(-forward.z, 0.0, forward.x);

        // Vertices
        vertices.push(float_array_from_vec3(points[i].position + left * ROAD_WIDTH));
        vertices.push(float_array_from_vec3(points[i].position - left * ROAD_WIDTH));

        // Uvs
        let completion = i as f32 / (points.len() - 1) as f32;
        uvs.push([0.0, completion]);
        uvs.push([1.0, completion]);

        // Triangles
        if i < points.len() - 1 {
            triangles.push(vert_index + 0);
            triangles.push(vert_index + 2);
            triangles.push(vert_index + 1);

            triangles.push(vert_index + 1);
            triangles.push(vert_index + 2);
            triangles.push(vert_index + 3);
        }

        vert_index += 2;
    }

    let mut mesh = Mesh::new(bevy::render::mesh::PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, VertexAttributeValues::Float32x3(vec![[0.0, 1.0, 0.0]; vertices.len()]));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, VertexAttributeValues::Float32x3(vertices));
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::Float32x2(uvs));
    mesh.set_indices(Some(Indices::U16(triangles)));

    mesh
}

fn float_array_from_vec3(p: Vec3) -> [f32; 3] {
    [p.x, p.y, p.z]
}