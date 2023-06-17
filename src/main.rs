use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
extern crate rand;
use rand::Rng;
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
    let mut v = [0., 2., 8., 18., 32., 0.];
    let r = [25., 80., 130., 180., 0.];
    let mut last = 0.;
    for i in &mut v{
        *i += last;
        last = *i;
    }
    let mut w = -20.;


    println!("{v:?}");


    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let n = s.trim().parse::<usize>().unwrap();

    for i in v{
        w += 50.;
        if n as f32 > i{
            println!("spown! {}", i);

            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(w).into()).into(),
                material: materials.add(ColorMaterial::from(Color::rgb(1., 1., 1.))),
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
    let mut iter_v = v.iter();
    iter_v.next();
    let mut cnt = iter_v.next().unwrap();
    let mut idx = 0;
    let rnd= rand::thread_rng().gen_range(5., 10.);

    for i in 0..n{
        let i = i as f32;
        println!("spawn element: {cnt}");
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(8.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(1., 1., 1.))),
            transform: Transform::from_translation(Vec3::new(f32::sin(i * rnd)*r[idx], f32::cos(i * rnd)*r[idx], 500.)),
            ..default()
        });

        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(8.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(1., 1., 1.))),
            transform: Transform::from_translation(Vec3::new(f32::sin(i * rnd)*r[idx], f32::cos(i * rnd)*r[idx], 500.)),
            ..default()
        });
        
        if i+2. > *cnt{
            println!("next");
            cnt = iter_v.next().unwrap();
            idx += 1;
        }
    }
}