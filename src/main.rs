use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .run();
}

struct Element;
impl Default for Element{
    fn default() -> Self {
        Self{}
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
){
    commands.spawn(Camera2dBundle::default());

    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let n = s.trim().parse::<usize>().unwrap();

    let mut v = [2., 8., 18., 32.];
    v.reverse();

    let mut w = 20. * (v.len() + 7) as f32;
    for i in v{
        w -= 50.;
        if n as f32 > i{
            println!("spown! {}", i * 8.);

            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(w).into()).into(),
                material: materials.add(ColorMaterial::from(Color::rgb(i * 8., i * 8., i * 8.))),
                transform: Transform::from_translation(Vec3::new(0., 0., 100. - i)),
                ..default()
            });

            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(w - 1.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::rgb(0., 0., i * 0.))),
                transform: Transform::from_translation(Vec3::new(0., 0., 101. - i)),
                ..default()
            });
            // break;
        }
    }
}