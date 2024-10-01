use std::{fs::File, io::{self, BufRead}, path::Path};
use bevy::{ math::Affine2, prelude::*, render::{mesh::Indices, render_asset:: RenderAssetUsages}};
use bevy_rapier3d::prelude::*;
use crate::menu::GameState;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self,app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), (spawn_floor, spawn_light, spawn_walls));
    }
}

fn spawn_light(mut commands: Commands) {
    let light =( 
        PointLightBundle {
            point_light : PointLight {
                shadows_enabled: true,
                intensity: 100.0,
                ..Default::default()
            },
            transform : Transform::from_xyz(0.0, 5.0, 0.0),
            ..Default::default()
        },
        Name::new("Main Light"),
    );

    commands.spawn(light);
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    server : Res<AssetServer>
) {
    let floor_material_handle: Handle<Image> = server.load("grass.jpg");

    let mut lines = read_lines("assets/map.txt").unwrap();

    let n = match lines.next() {
        Some(Ok(line)) => line.chars().count(),
        Some(Err(e)) => return,
        None => 0,
    };
    
    let m = 1 + lines.count();

    println!("{}, {}", m, n);

    let floor = (
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(n as f32, m as f32)),
            material: materials.add(floor_material_handle),
            transform: Transform::from_xyz((n as f32)/2.0 - 0.5, 0.0, (m as f32)/2.0 - 0.5),
            ..Default::default()
        },
        Name::new("Floor"),
        RigidBody::Fixed,
        Collider::cuboid(50.0, 0.0, 50.0)
    );

    commands.spawn(floor);
}

fn spawn_walls (
    mut commands : Commands,
    mut meshes : ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    server : Res<AssetServer> 
) {
let mat: [f32; 4]  =  [1.0, 0.0, 0.0, 2.0];
    let wall_handle: Handle<Image> = server.load("textures\\patched-brickwork-bl\\patched-brickwork_albedo.png");
    let wall_material = materials.add(
        StandardMaterial {
            base_color : Color::WHITE,
            base_color_texture : Some(wall_handle),
            metallic_roughness_texture: Some(server.load("textures\\patched-brickwork-bl\\patched-brickwork_metallic.png")),
            normal_map_texture: Some(server.load("textures\\patched-brickwork-bl\\patched-brickwork_normal-ogl.png")),
            occlusion_texture: Some(server.load("textures\\patched-brickwork-bl\\patched-brickwork_ao.png")),
            uv_transform: Affine2::from_mat2(Mat2::from_cols_array(&mat)),
            ..Default::default()
        }
    );
    // println!("wall?");

    let mesh1 = Mesh::new(bevy::render::mesh::PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
                .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, 
                    vec![
                        // top (facing towards +y)
                        [-0.5, 1.0, -0.5], // vertex with index 0
                        [0.5, 1.0, -0.5], // vertex with index 1
                        [0.5, 1.0, 0.5], // etc. until 23
                        [-0.5, 1.0, 0.5],
                        // bottom   (-y)
                        [-0.5, -0.5, -0.5],
                        [0.5, -0.5, -0.5],
                        [0.5, -0.5, 0.5],
                        [-0.5, -0.5, 0.5],
                        // right    (+x)
                        [0.5, -0.5, -0.5],
                        [0.5, -0.5, 0.5],
                        [0.5, 1.0, 0.5], // This vertex is at the same position as vertex with index 2, but they'll have different UV and normal
                        [0.5, 1.0, -0.5],
                        // left     (-x)
                        [-0.5, -0.5, -0.5],
                        [-0.5, -0.5, 0.5],
                        [-0.5, 1.0, 0.5],
                        [-0.5, 1.0, -0.5],
                        // back     (+z)
                        [-0.5, -0.5, 0.5],
                        [-0.5, 1.0, 0.5],
                        [0.5, 1.0, 0.5],
                        [0.5, -0.5, 0.5],
                        // forward  (-z)
                        [-0.5, -0.5, -0.5],
                        [-0.5, 1.0, -0.5],
                        [0.5, 1.0, -0.5],
                        [0.5, -0.5, -0.5],
                    ],
                ).with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, 
                    vec![
                        // Assigning the UV coords for the top side.
                        [0.0, 0.2], [0.0, 0.0], [1.0, 0.0], [1.0, 0.2],
                        // Assigning the UV coords for the bottom side.
                        [0.0, 0.45], [0.0, 0.25], [1.0, 0.25], [1.0, 0.45],
                        // Assigning the UV coords for the right side.
                        [1.0, 0.45], [0.0, 0.45], [0.0, 0.2], [1.0, 0.2],
                        // Assigning the UV coords for the left side.
                        [1.0, 0.45], [0.0, 0.45], [0.0, 0.2], [1.0, 0.2],
                        // Assigning the UV coords for the back side.
                        [0.0, 0.45], [0.0, 0.2], [1.0, 0.2], [1.0, 0.45],
                        // Assigning the UV coords for the forward side.
                        [0.0, 0.45], [0.0, 0.2], [1.0, 0.2], [1.0, 0.45],
                    ],
                )
                .with_inserted_attribute(
                    Mesh::ATTRIBUTE_NORMAL,
                    vec![
                        // Normals for the top side (towards +y)
                        [0.0, 1.0, 0.0],
                        [0.0, 1.0, 0.0],
                        [0.0, 1.0, 0.0],
                        [0.0, 1.0, 0.0],
                        // Normals for the bottom side (towards -y)
                        [0.0, -1.0, 0.0],
                        [0.0, -1.0, 0.0],
                        [0.0, -1.0, 0.0],
                        [0.0, -1.0, 0.0],
                        // Normals for the right side (towards +x)
                        [1.0, 0.0, 0.0],
                        [1.0, 0.0, 0.0],
                        [1.0, 0.0, 0.0],
                        [1.0, 0.0, 0.0],
                        // Normals for the left side (towards -x)
                        [-1.0, 0.0, 0.0],
                        [-1.0, 0.0, 0.0],
                        [-1.0, 0.0, 0.0],
                        [-1.0, 0.0, 0.0],
                        // Normals for the back side (towards +z)
                        [0.0, 0.0, 1.0],
                        [0.0, 0.0, 1.0],
                        [0.0, 0.0, 1.0],
                        [0.0, 0.0, 1.0],
                        // Normals for the forward side (towards -z)
                        [0.0, 0.0, -1.0],
                        [0.0, 0.0, -1.0],
                        [0.0, 0.0, -1.0],
                        [0.0, 0.0, -1.0],
                    ],
                )
                .with_inserted_indices(Indices::U32(vec![
                    0,3,1 , 1,3,2, // triangles making up the top (+y) facing side.
                    4,5,7 , 5,6,7, // bottom (-y)
                    8,11,9 , 9,11,10, // right (+x)
                    12,13,15 , 13,14,15, // left (-x)
                    16,19,17 , 17,19,18, // back (+z)
                    20,21,23 , 21,22,23, // forward (-z)
                ]));


    for (_c, row) in read_lines("assets/map.txt").into_iter().enumerate() {
        for (y, line) in row.enumerate().into_iter() {
            for (x, tile) in line.unwrap().chars().enumerate().into_iter(){
                let pos = Vec3::new(x as f32, 0.5, y as f32);
                // println!("Tile[{}][{}] : {}", x, y, tile);
                match tile {
                    '#' => {
                        commands.spawn((
                            PbrBundle {
                                mesh : meshes.add(mesh1.clone()),
                                material: wall_material.clone(),
                                transform: Transform::from_translation(pos),
                                ..default()
                            },
                            RigidBody::Fixed,
                            Collider::cuboid(0.25,1.0,0.25)
                        ));
                    },
                    _=>()
                };
            }
        }
    }


}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
