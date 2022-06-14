use bevy::{prelude::{Mesh, Vec3}, render::mesh::{Indices, VertexAttributeValues, PrimitiveTopology::TriangleList}};
use bevy::prelude::{Assets, Color, Commands, PbrBundle, ResMut, StandardMaterial};
use slotmap::SlotMap;
use crate::game::roads::components::{Intersection, Road, RoadCap, RoadKey};

const ROAD_WIDTH: f32 = 3.5;

impl Road {
    pub fn generate_road_mesh(
        self: &mut Self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        if let Some(entity) = self.mesh_entity {
            commands.entity(entity).despawn();
        }

        let mut vertices: Vec<[f32; 3]> = Vec::new();
        let mut triangles: Vec<u16> = Vec::new();
        let mut uvs: Vec<[f32; 2]> = Vec::new();

        let mut vert_index = 0;

        for i in 0..self.nodes.len() {

            let mut forward = Vec3::default();

            if i < self.nodes.len() - 1 {
                forward += self.nodes[i + 1].position - self.nodes[i].position;
            }

            if i > 0 {
                forward += self.nodes[i].position - self.nodes[i - 1].position;
            }

            forward = forward.normalize();
            let left = Vec3::new(-forward.z, 0.0, forward.x);

            // Vertices
            vertices.push(float_array_from_vec3(self.nodes[i].position + left * ROAD_WIDTH));
            vertices.push(float_array_from_vec3(self.nodes[i].position - left * ROAD_WIDTH));

            // Uvs
            let completion = i as f32 / (self.nodes.len() - 1) as f32;
            uvs.push([0.0, completion]);
            uvs.push([1.0, completion]);

            // Triangles
            if i < self.nodes.len() - 1 {
                triangles.push(vert_index + 0);
                triangles.push(vert_index + 2);
                triangles.push(vert_index + 1);

                triangles.push(vert_index + 1);
                triangles.push(vert_index + 2);
                triangles.push(vert_index + 3);
            }

            vert_index += 2;
        }

        let mut mesh = Mesh::new(TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, VertexAttributeValues::Float32x3(vec![[0.0, 1.0, 0.0]; vertices.len()]));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, VertexAttributeValues::Float32x3(vertices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::Float32x2(uvs));
        mesh.set_indices(Some(Indices::U16(triangles)));

        self.mesh_entity = Some(commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            ..Default::default()
        }).id());
    }
}

impl Intersection {
    pub fn generate_intersection_mesh(
        self: &mut Self, roads: &SlotMap<RoadKey, Road>,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {

        // Remove old intersection mesh if one exists
        if let Some(entity) = self.mesh_entity {
            commands.entity(entity).despawn();
        }

        let mut vertex_pairs = vec![];

        // Loop over connections and create pairs of vertices of the closest end of the road to the intersection
        for connection in &self.roads {
            let road_nodes = &roads[connection.0].nodes;

            if road_nodes.len() < 2 {
                continue;
            }

            let forward = match connection.1 {
                RoadCap::Start => road_nodes[1].position - road_nodes.first().unwrap().position,
                RoadCap::End => road_nodes.last().unwrap().position - road_nodes[road_nodes.len() - 2].position,
            };

            let forward = forward.normalize();
            let left = Vec3::new(-forward.z, 0.0, forward.x);

            let center = match connection.1 {
                RoadCap::Start => road_nodes.first().unwrap().position,
                RoadCap::End => road_nodes.last().unwrap().position,
            };

            let p0 = center - left * ROAD_WIDTH;
            let p1 = center + left * ROAD_WIDTH;

            match connection.1 {
                RoadCap::Start => vertex_pairs.push((p1, p0)),
                RoadCap::End => vertex_pairs.push((p0, p1)),
            }
        }

        if vertex_pairs.len() < 3 {
            return;
        }

        // Create a list of vertices from the pairs and sort it so they go in a clockwise rotation
        let mut next_index = 0;
        let mut vertices: Vec<[f32; 3]> = Vec::new();
        while vertex_pairs.len() > 0 {
            let pair = vertex_pairs.remove(next_index);
            vertices.push(float_array_from_vec3(pair.0));
            vertices.push(float_array_from_vec3(pair.1));

            let mut closest_index = None;
            let mut closest_distance = f32::INFINITY;
            for i in 0..vertex_pairs.len() {
                let distance_sq = pair.1.distance_squared(vertex_pairs[i].0);
                if distance_sq < closest_distance {
                    closest_index = Some(i);
                    closest_distance = distance_sq;
                }
            }

            if let Some(index) = closest_index {
                next_index = index;
            } else {
                break;
            }
        }

        // Create triangles from the vertices
        let mut triangles: Vec<u16> = Vec::new();
        for i in 2..vertices.len() as u16 {
            triangles.push(0);
            triangles.push(i - 1);
            triangles.push(i);
        }

        // Put together the mesh
        let mut mesh = Mesh::new(TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::Float32x2(vec![[0.0_32, 0.0_f32]; vertices.len()]));
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, VertexAttributeValues::Float32x3(vec![[0.0, 1.0, 0.0]; vertices.len()]));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, VertexAttributeValues::Float32x3(vertices));
        mesh.set_indices(Some(Indices::U16(triangles)));

        // Spawn an entity with the mesh and assign the intersection with the entities id
        self.mesh_entity = Some(commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            ..Default::default()
        }).id());

    }
}

#[inline(always)]
fn float_array_from_vec3(p: Vec3) -> [f32; 3] {
    [p.x, p.y, p.z]
}