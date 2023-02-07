use bevy::app::App;
use bevy::ecs::component::*;
use bevy::ecs::system::*;
use bevy::ecs::query::*;

//bevy programs are Apps, this is the core component
fn main() {
    App::new()
        //startup systems run exactly one time when the app/game starts
        .add_startup_system(add_people)
        //this adds the hello_world() function to the scheduler
        //don't use function name parentheses
        .add_system(hello_world)
        .add_system(greet_people)
        .run();
}

fn hello_world() {
    println!("Hello World!");
}

//adding these components marks that an entity is a person, and adds their name
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

//so Query basically means "iterate over every Name component for entities that also have a Person component
fn greet_people(query: Query<&Name, With<Person>>) {
    //looks like this pulls the Name component being iterated and stores it in 'name'
    //then the println inserts 'name' into the string
    for name in query.iter() {
        println!("Hello {}!", name.0);
    }
}