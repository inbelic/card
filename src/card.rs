use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NextCardID>()
            .add_systems(Startup, setup)
            .add_systems(Update, move_card);
    }
}

fn setup(
    mut next_card_id: ResMut<NextCardID>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for i in -2..3 {
        let posn = Vec2::new(i as f32 * 100., 0.);
        spawn_card(next_card_id.reborrow(), commands.reborrow(), meshes.reborrow(), materials.reborrow(), posn);
    }

}

#[derive(Resource, Debug, Default)]
struct NextCardID(u16);

#[derive(Component, Debug)]
pub struct Card {
    id: u16,
}

impl Card {
    pub fn get_id(&self) -> u16 {
        self.id
    }
}

#[derive(Component, Debug)]
pub struct Target {
    pub posn: Vec2
}

#[derive(Bundle)]
struct CardBundle {
    card: Card,
    mesh: MaterialMesh2dBundle<ColorMaterial>,
    target: Target,
}

const CARD_WIDTH: f32 = 63.;
const CARD_HEIGHT_SCALE: f32 = 1.4;

fn spawn_card(
    mut next_card_id: Mut<NextCardID>,
    mut commands: Commands,
    mut meshes: Mut<Assets<Mesh>>,
    mut materials: Mut<Assets<ColorMaterial>>,
    posn: Vec2
) {
    info!("{:?} at {:?}", next_card_id, posn);
    let shape = Mesh2dHandle(meshes.add(
            Rectangle::new(CARD_WIDTH, CARD_WIDTH * CARD_HEIGHT_SCALE)));
    let color = Color::srgb(139., 69., 19.);
    let mat = materials.add(color);
    commands.spawn(CardBundle {
        card: Card {
            id: next_card_id.0,
        },
        mesh: MaterialMesh2dBundle {
            mesh: shape,
            material: mat,
            transform: Transform::from_translation(posn.extend(0.)),
            ..default()
        },
        target: Target {
            posn: posn
        },
    });

    // increment the id for uniqueness
    next_card_id.0 = next_card_id.0 + 1;
}


fn interpolate(from: Vec2, to: Vec2) -> Vec2 {
    let interpolation_factor = 0.1;
    let x = from.x + (to.x - from.x) * interpolation_factor;
    let y = from.y + (to.y - from.y) * interpolation_factor;
    Vec2::new(x, y)
}

fn move_card(
    mut query: Query<(&mut Transform, &Target), With<Card>>,
) {
    for (mut transform, target) in query.iter_mut() {
        transform.translation = interpolate(
            transform.translation.truncate(),
            target.posn
        ).extend(transform.translation.z)
    }
}
