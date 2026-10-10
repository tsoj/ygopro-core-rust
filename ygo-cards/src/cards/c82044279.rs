//! Clear Wing Synchro Dragon
use super::prelude::*;

pub const ID: u32 = 82044279;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    card::enable_revive_limit(d, c);
    // negate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_condition(d, e2, Some(condition(cond2)));
    card::register_effect(d, c, e2, false);
    // atk
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_DESTROYED);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_condition(d, e3, Some(condition(atkcon)));
    effect::set_operation(d, e3, Some(operation(atkop)));
    card::register_effect(d, c, e3, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let rc = effect::get_handler(d, a.re.nil()).nil();
    let loc = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int();
    let re = a.re.nil();
    effect::is_monster_effect(d, re)
        && rc != c
        && card::is_level_above(d, rc, 5)
        && loc == LOCATION_MZONE as i64
        && !card::is_status(d, c, STATUS_BATTLE_DESTROYED)
        && duel::is_chain_negatable(d, a.ev as u8)
}
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    let g = match duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS])[0].as_group() {
        Some(g) => g.clone(),
        None => return false,
    };
    if g.len() != 1 {
        return false;
    }
    let tc = group::get_first(d, &g).nil();
    let c = effect::get_handler(d, a.e).nil();
    effect::is_monster_effect(d, re)
        && card::is_faceup(d, tc)
        && card::is_level_above(d, tc, 5)
        && card::is_location(d, tc, LOCATION_MZONE)
        && !card::is_status(d, c, STATUS_BATTLE_DESTROYED)
        && duel::is_chain_negatable(d, a.ev as u8)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, effect::get_description(d, a.e));
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, re) {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let rc = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rc, re) {
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e);
    a.r & REASON_EFFECT != 0
        && a.re.is_some()
        && effect::get_owner(d, a.re.nil()) == h
        && group::is_exists(d, &a.eg.clone().nil(), &|d, c| card::is_monster(d, c), 1, NO_EX)
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let g = group::filter(d, &a.eg.clone().nil(), &|d, c| card::is_monster(d, c), NO_EX);
    let c = effect::get_handler(d, a.e).nil();
    let mut atk: i64 = 0;
    let _ = group::get_first(d, &g);
    for tc in group::iter(d, &g) {
        if card::get_text_attack(d, tc) > 0 {
            atk += card::get_text_attack(d, tc) as i64;
        }
    }
    if atk > 0 {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, atk);
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        card::register_effect(d, c, e1, false);
    }
}
