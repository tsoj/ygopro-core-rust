//! Salamangreat of Fire
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 11962031;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Search 1 Level 4 or lower "Salamangreat" monster
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e2, false);

    // Destroy your battling Cyberse monster
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_DESTROY);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e3, EVENT_BATTLE_START);
    effect::set_range(d, e3, LOCATION_GRAVE);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_condition(d, e3, Some(condition(descon)));
    effect::set_cost(d, e3, Some(cost::self_banish()));
    effect::set_target(d, e3, Some(cost_target(destg)));
    effect::set_operation(d, e3, Some(operation(desop)));
    card::register_effect(d, c, e3, false);
}

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_SALAMANGREAT],
    ..ScriptInfo::DEFAULT
};

fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_SALAMANGREAT as u16])
        && card::is_monster(d, c)
        && card::is_level_below(d, c, 4)
        && !card::is_code(d, c, &[ID])
        && card::is_able_to_hand(d, c, None)
}

fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}

fn thop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX);
    if let Some(g) = g {
        if !g.is_empty() {
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            duel::confirm_cards(d, 1 - tp, g);
        }
    }
    // Cannot Special Summon, except FIRE monsters
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_description(d, e1, aux::stringid(ID, 2));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_attribute_except(d, c, ATTRIBUTE_FIRE, None, 0, None))));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
}

fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let bc = duel::get_battle_monster(d, a.tp).0;
    match bc {
        Some(bc) => card::is_faceup(d, bc) && card::is_race(d, bc, RACE_CYBERSE as u64),
        None => false,
    }
}

fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        return true;
    }
    let bc = duel::get_battle_monster(d, tp).0;
    let label = match bc {
        Some(bc) => Value::Card(bc),
        None => Value::Nil,
    };
    effect::set_label_object(d, e, label);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, bc.map(CardSet::from_card), 1, tp, 0);
    true
}

fn desop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let bc = effect::get_label_object(d, e).as_card().nil();
    if card::is_controler(d, bc, tp) && card::is_relate_to_battle(d, bc) {
        duel::destroy(d, bc, REASON_EFFECT, None, None);
    }
}
