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

#[inline(always)]
fn float_array_from_vec3(p: Vec3) -> [f32; 3] {
    [p.x, p.y, p.z]
}

mod tests {
    use bevy::math::Vec3;
    use bevy::render::mesh::PrimitiveTopology;
    use super::*;

    #[test]
    fn float_array_from_vec3_test() {
        assert_eq!([5.0, 3.0, 8.0], float_array_from_vec3(Vec3::new(5.0, 3.0, 8.0)));
    }

    #[test]
    fn correct_road_mesh_test() {
        let nodes = vec![
            Node::new(Vec3::new(0.0, 0.0, 0.0)),
            Node::new(Vec3::new(2.0, 0.0, 0.0)),
            Node::new(Vec3::new(4.0, 0.0, 0.0)),
            Node::new(Vec3::new(6.0, 0.0, 0.0)),
            Node::new(Vec3::new(7.0, 0.0, 1.0)),
            Node::new(Vec3::new(7.0, 0.0, 3.0)),
            Node::new(Vec3::new(7.0, 0.0, 5.0)),
        ];

        let mesh = generate_road_mesh(&nodes);

        assert_eq!(mesh.primitive_topology(), PrimitiveTopology::TriangleList, "Topology");

        assert_eq!(mesh.count_vertices(), 14, "Vertices Count");

        let correct_vertices = VertexAttributeValues::Float32x3(vec![
            [
                0.0,
                0.0,
                3.5,
            ],
            [
                0.0,
                0.0,
                -3.5,
            ],
            [
                2.0,
                0.0,
                3.5,
            ],
            [
                2.0,
                0.0,
                -3.5,
            ],
            [
                4.0,
                0.0,
                3.5,
            ],
            [
                4.0,
                0.0,
                -3.5,
            ],
            [
                4.893203,
                0.0,
                3.3203914,
            ],
            [
                7.106797,
                0.0,
                -3.3203914,
            ],
            [
                3.6796086,
                0.0,
                2.1067972,
            ],
            [
                10.320392,
                0.0,
                -0.10679722,
            ],
            [
                3.5,
                0.0,
                3.0,
            ],
            [
                10.5,
                0.0,
                3.0,
            ],
            [
                3.5,
                0.0,
                5.0,
            ],
            [
                10.5,
                0.0,
                5.0,
            ],
        ]);
        assert_eq!(mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().get_bytes(), correct_vertices.get_bytes(), "Vertices");

        let correct_normals = VertexAttributeValues::Float32x3(vec![[0.0, 1.0, 0.0]; 14]);
        assert_eq!(mesh.attribute(Mesh::ATTRIBUTE_NORMAL).unwrap().get_bytes(), correct_normals.get_bytes(), "Normals");

        let correct_uvs = VertexAttributeValues::Float32x2(vec![
            [
                0.0,
                0.0,
            ],
            [
                1.0,
                0.0,
            ],
            [
                0.0,
                0.16666667,
            ],
            [
                1.0,
                0.16666667,
            ],
            [
                0.0,
                0.33333334,
            ],
            [
                1.0,
                0.33333334,
            ],
            [
                0.0,
                0.5,
            ],
            [
                1.0,
                0.5,
            ],
            [
                0.0,
                0.6666667,
            ],
            [
                1.0,
                0.6666667,
            ],
            [
                0.0,
                0.8333333,
            ],
            [
                1.0,
                0.8333333,
            ],
            [
                0.0,
                1.0,
            ],
            [
                1.0,
                1.0,
            ],

        ]);
        assert_eq!(mesh.attribute(Mesh::ATTRIBUTE_UV_0).unwrap().get_bytes(), correct_uvs.get_bytes(), "UVs");

        let correct_indices: Vec<u16> = vec![
            0, 2, 1, 1, 2, 3, 2, 4, 3, 3, 4, 5, 4, 6, 5, 5, 6, 7, 6, 8, 7, 7, 8, 9, 8, 10, 9, 9, 10, 11, 10, 12, 11, 11, 12, 13,
        ];
        let mesh_indices = match mesh.indices().unwrap() {
            Indices::U16(indices) => indices,
            Indices::U32(_) => panic!("Mesh Indices are formatted as U32 instead of U16"),
        };
        assert_eq!(mesh_indices, &correct_indices, "Indices");
    }
}