//! Flying Fortress SKY FIRE
use super::prelude::*;

pub const ID: u32 = 16898077;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // cannot special summon
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    card::register_effect(d, c, e1, false);
    // destroy
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_cost(d, e2, Some(cost_target(descost)));
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
    // destroy & damage
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    effect::set_condition(d, e3, Some(condition(damcon)));
    effect::set_target(d, e3, Some(cost_target(damtg)));
    effect::set_operation(d, e3, Some(operation(damop)));
    card::register_effect(d, c, e3, false);
    let e4 = effect::clone(d, e3);
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e4, false);
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 2));
    effect::set_category(d, e5, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e5, EVENT_MSET);
    effect::set_range(d, e5, LOCATION_MZONE);
    effect::set_count_limit(d, e5, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    effect::set_condition(d, e5, Some(condition(damcon2)));
    effect::set_target(d, e5, Some(cost_target(damtg2)));
    effect::set_operation(d, e5, Some(operation(damop2)));
    card::register_effect(d, c, e5, false);
    let e6 = effect::clone(d, e5);
    effect::set_code(d, e6, EVENT_SSET);
    card::register_effect(d, c, e6, false);
    let e7 = effect::create_effect(d, c);
    effect::set_description(d, e7, aux::stringid(ID, 2));
    effect::set_category(d, e7, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    effect::set_type(d, e7, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e7, EVENT_CHANGE_POS);
    effect::set_range(d, e7, LOCATION_MZONE);
    effect::set_count_limit(d, e7, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    effect::set_condition(d, e7, Some(condition(damcon3)));
    effect::set_target(d, e7, Some(cost_target(damtg3)));
    effect::set_operation(d, e7, Some(operation(damop3)));
    card::register_effect(d, c, e7, false);
}
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&card::is_able_to_grave_as_cost), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&card::is_able_to_grave_as_cost), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, tp, None, tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) {
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
/// `s.dfilter(c,e,sp)`
fn dfilter(d: &mut Duel, c: CardId, e: Option<EffectId>, sp: u8) -> bool {
    card::get_summon_player(d, c) == sp && match e {
        None => true,
        Some(e) => card::is_relate_to_effect(d, c, e),
    }
}
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &|d, c| dfilter(d, c, None, 1 - tp), 1, NO_EX);
    }
    let g = group::filter(d, &eg, &|d, c| dfilter(d, c, None, 1 - tp), NO_EX);
    duel::set_target_card(d, &eg);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 800);
    true
}
fn damop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| dfilter(d, c, Some(a.e), 1 - tp), NO_EX);
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) && !g.is_empty() && duel::destroy(d, g, REASON_EFFECT, None, None) != 0 {
        duel::damage(d, 1 - tp, 800, REASON_EFFECT, false, None);
    }
}
fn damcon2(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp) && a.rp != a.tp
}
fn damtg2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_card(d, &eg);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(eg.clone()), eg.len() as u8, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 800);
    true
}
fn damop2(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg);
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) && card::is_relate_to_effect(d, tc.nil(), a.e) && duel::destroy(d, tc.nil(), REASON_EFFECT, None, None) != 0 {
        duel::damage(d, 1 - tp, 800, REASON_EFFECT, false, None);
    }
}
fn damcon3(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp) && a.rp != a.tp
}
/// `s.sfilter(c,e)`
fn sfilter(d: &mut Duel, c: CardId, e: Option<EffectId>) -> bool {
    card::is_facedown(d, c) && match e {
        None => true,
        Some(e) => card::is_relate_to_effect(d, c, e),
    }
}
fn damtg3(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &|d, c| sfilter(d, c, None), 1, NO_EX);
    }
    let g = group::filter(d, &eg, &|d, c| sfilter(d, c, None), NO_EX);
    duel::set_target_card(d, &eg);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 800);
    true
}
fn damop3(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| sfilter(d, c, Some(a.e)), NO_EX);
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) && !g.is_empty() && duel::destroy(d, g, REASON_EFFECT, None, None) != 0 {
        duel::damage(d, 1 - tp, 800, REASON_EFFECT, false, None);
    }
}
