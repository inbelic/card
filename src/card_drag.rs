use bevy::{
    prelude::*,
    input::common_conditions::*,
    math::bounding::Aabb2d,
    render::primitives::Aabb,
};

use crate::card;
use crate::mouse;

pub struct CardDragPlugin;

#[derive(Resource, Debug, Default)]
struct SelectedCardID(Option<u16>);

impl Plugin for CardDragPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedCardID>()
            .add_systems(Update,
                select_card.run_if(input_just_pressed(MouseButton::Left))
                           .run_if(deselected))
            .add_systems(Update,
                deselect_card.run_if(input_just_released(MouseButton::Left))
                             .run_if(selected))
            .add_systems(Update, update_card_target.run_if(selected));
    }
}

fn selected(selected: Res<SelectedCardID>) -> bool { selected.0.is_some() }
fn deselected(selected: Res<SelectedCardID>) -> bool { selected.0.is_none() }

fn select_card(
    mouse: Res<mouse::Mouse>,
    mut selected: ResMut<SelectedCardID>,
    query: Query<(&card::Card, &Transform, &Aabb)>
) {
    for (card, transform, aabb) in query.iter() {
        let card_bounds = Aabb2d::new(
            transform.translation.truncate(), aabb.half_extents.truncate()
        );
        let mouse_posn = mouse.get_posn();
        if mouse_posn == card_bounds.closest_point(mouse_posn) {
            selected.0 = Some(card.get_id());
            info!("Mouse selected {:?} at {:?}", card.get_id(), mouse.get_posn());
        }
    }
}

fn deselect_card(
    mouse: Res<mouse::Mouse>,
    mut selected: ResMut<SelectedCardID>,
) {
    info!("Mouse deselected {:?} at {:?}", selected.0.unwrap(), mouse.get_posn());
    selected.0 = None;
}

fn update_card_target(
    mouse: Res<mouse::Mouse>,
    selected: Res<SelectedCardID>,
    mut query: Query<(&card::Card, &mut card::Target)>
) {
    let selected = selected.0.unwrap();
    for (card, mut target) in query.iter_mut() {
        if card.get_id() == selected {
            target.posn = mouse.get_posn()
        }
    }
}
