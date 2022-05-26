use bevy::prelude::*;

// // entity
// struct Entity(f64)

// // component
// #[derive(Component)]
// struct Position { x: f32, y: f32 }

// // system
// fn print_position_system(query: Query<&Transform>) {
//     for transform in query.iter() {
//         println!("Position: {:?}", transform.translation);
//     }
// }
// ENTITY COMPONENT SYSTEM = ECS

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Elaina Proctor".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Wong Kar".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Michael Grooves".to_string()));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("hello {}", name.0);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_people)
        .add_system(greet_people)
        .run();
}
