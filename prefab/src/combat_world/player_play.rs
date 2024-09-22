use log::debug;
use pipedream_engine::{
    choice::{Card, Choices},
    command::UiMode,
    description::Description,
    effect::{Effect, Transition},
    state_machine::StateMachine,
};

use crate::combat_world::{HUMAN_DAMAGE, HUMAN_PLAY};
use pipedream_domain::{
    card::PlacedCard, field::FieldPlace, player::Player, player::PlayerCharacter,
};
use pipedream_engine::{choice::Choice, scene::Scene, state::State};

pub fn player_play(player: &Player, machine: &StateMachine) -> State {
    let (character_id, _character) = PlayerCharacter::get_player_character(&machine.conn, player);
    let player_hand = PlacedCard::get_placed_cards(&machine.conn, &character_id, &FieldPlace::Hand);
    debug!(target:"Prefab/Combat/Hand", "{:?}", player_hand);

    State {
        location: HUMAN_PLAY.clone(),
        scene: Scene {
            descriptions: vec![Description::always("Play")],
        },
        choices: Choices::manual(
            player_hand
                .into_iter()
                .flat_map(|(_id, PlacedCard { card: card_id, .. })| {
                    Card::get_card(&machine.conn, &card_id).into_iter()
                })
                .map(|card| {
                    let selectable = card.predicate_satisfied(&machine.conn);
                    Choice {
                        card: Card {
                            effect: Effect {
                                transition: Transition::Goto(HUMAN_DAMAGE.clone()),
                                ..card.effect
                            },
                            ..card
                        },
                        selectable,
                    }
                })
                .collect::<Vec<_>>(),
        ),
        ui_mode: UiMode::Combat,
    }
}
