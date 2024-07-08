use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CardID>()
            .add_systems(Startup, setup);
    }
}

fn setup(
    mut card_id: ResMut<CardID>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for i in -2..3 {
        let posn = Vec2::new(i as f32 * 100., 0.);
        spawn_card(card_id.reborrow(), commands.reborrow(), meshes.reborrow(), materials.reborrow(), posn);
    }

}

#[derive(Resource, Debug, Default)]
struct CardID(u16);

#[derive(Component, Debug)]
struct Card {
    id: u16,
}

#[derive(Bundle)]
struct CardBundle {
    card: Card,
    mesh: MaterialMesh2dBundle<ColorMaterial>,
}

const CARD_WIDTH: f32 = 63.;
const CARD_HEIGHT_SCALE: f32 = 1.4;

fn spawn_card(
    mut card_id: Mut<CardID>,
    mut commands: Commands,
    mut meshes: Mut<Assets<Mesh>>,
    mut materials: Mut<Assets<ColorMaterial>>,
    posn: Vec2
) {
    info!("{:?} at {:?}", card_id, posn);
    let shape = Mesh2dHandle(meshes.add(
            Rectangle::new(CARD_WIDTH, CARD_WIDTH * CARD_HEIGHT_SCALE)));
    let color = Color::srgb(139., 69., 19.);
    let mat = materials.add(color);
    commands.spawn(CardBundle {
        card: Card {
            id: card_id.0,
        },
        mesh: MaterialMesh2dBundle {
            mesh: shape,
            material: mat,
            transform: Transform::from_translation(posn.extend(0.)),
            ..default()
        }
    });

    // increment the id for uniqueness
    card_id.0 = card_id.0 + 1;
}
