//! Shadow of Eyes
use super::prelude::*;

pub const ID: u32 = 58621589;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_POSITION);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_CHANGE_POS);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_MSET);
    effect::set_target(d, e2, Some(cost_target(target2)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e1);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    effect::set_target(d, e3, Some(cost_target(target2)));
    card::register_effect(d, c, e3, false);
}
fn filter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFacedown() and c:IsPreviousPosition(POS_FACEUP) and c:IsControler(1-tp)
    card::is_facedown(d, c) && card::is_previous_position(d, c, POS_FACEUP) && card::is_controler(d, c, 1 - tp)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return group::is_exists(d, &eg, &|d, c| filter1(d, c, a.tp), 1, NO_EX);
    }
    let eg = a.eg.clone().nil();
    duel::set_target_card(d, &eg);
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(eg.clone()), eg.len() as u8, 0, 0);
    true
}
fn filter2(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFacedown() and c:IsControler(1-tp)
    card::is_facedown(d, c) && card::is_controler(d, c, 1 - tp)
}
fn target2(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return group::is_exists(d, &eg, &|d, c| filter2(d, c, a.tp), 1, NO_EX);
    }
    let eg = a.eg.clone().nil();
    duel::set_target_card(d, &eg);
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(eg.clone()), eg.len() as u8, 0, 0);
    true
}
fn filter3(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFacedown() and c:IsRelateToEffect(e)
    card::is_facedown(d, c) && card::is_relate_to_effect(d, c, e)
}
fn op(d: &mut Duel, a: &EffArgs) {
    // local g=eg:Filter(s.filter3,nil,e,tp)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| filter3(d, c, a.e), NO_EX);
    // Duel.ChangePosition(g,POS_FACEUP_ATTACK,POS_FACEUP_ATTACK,POS_FACEUP_ATTACK,POS_FACEUP_ATTACK,true)
    duel::change_position(d, g, POS_FACEUP_ATTACK as u8, Some(POS_FACEUP_ATTACK as u8), Some(POS_FACEUP_ATTACK as u8), Some(POS_FACEUP_ATTACK as u8), true);
}
