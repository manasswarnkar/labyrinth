use std::{fs::File, io::{self, BufRead}, path::Path};

use bevy::{
    // color::palettes::css::{BLUE, RED}, 
    prelude::*
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self,app: &mut App) {
        app.add_systems(Startup, (spawn_floor, spawn_light, spawn_walls));
    }
}

fn spawn_light(mut commands: Commands) {
    let light =( 
        PointLightBundle {
            point_light : PointLight {
                shadows_enabled: true,
                intensity: 0.0,
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
) {
    let floor = (
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
            material: materials.add(Color::srgb(0.0, 1.0, 0.0).darker(0.5)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
    Name::new("Floor"),
    );

    commands.spawn(floor);
}

// fn spawn_objects(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     let mut create_cube = |size: f32, color: Srgba, xyz: (f32, f32, f32), name: String| -> (PbrBundle, Name) {
//         (
//             PbrBundle {
//                 mesh : meshes.add(Mesh::from(Cuboid::new(size,size,size)
//                 )),
//                 material : materials.add(StandardMaterial{
//                     base_color: color.into(),
//                     ..default()
//                 }),
//                 transform: Transform::from_xyz(xyz.0, xyz.1, xyz.2),
//                 ..default()
//             },
//             Name::new(name),
//         )
//     };
//     commands.spawn(create_cube(4.0, BLUE, (-5.0, 2.0, 5.0), "BLUE CUBE".to_string()));
//     commands.spawn(create_cube(2.0, RED, (6.0, 1.0, -6.0), "RED CUBE".to_string()));
// }


fn spawn_walls (
    mut commands : Commands,
    mut meshes : ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>> 
) {
   
    let wall_material = materials.add(Color::srgb(0.8, 0.2, 0.2));
    println!("wall?");

    for (_c, row) in read_lines("assets/map.txt").into_iter().enumerate() {
        for (y, line) in row.enumerate().into_iter() {
            for (x, tile) in line.unwrap().chars().enumerate().into_iter(){
                let pos = Vec3::new(x as f32, 1.0, y as f32);
                // println!("Tile[{}][{}] : {}", x, y, tile);
                match tile {
                    '#' => {
                        commands.spawn(
                            PbrBundle {
                                mesh : meshes.add(Mesh::from(Cuboid::new(1.0,2.0,1.0))),
                                material: wall_material.clone(),
                                transform: Transform::from_translation(pos),
                                ..default()
                            }
                        );
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
