//! Deep Dark Trap Hole
use super::prelude::*;

pub const ID: u32 = 28654932;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate(summon)
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId, e: Option<EffectId>) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_EFFECT) and c:GetLevel()>=5
    //     and (not e or c:IsRelateToEffect(e)) and c:IsAbleToRemove()
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_EFFECT)
        && card::get_level(d, c) >= 5
        && match e {
            None => true,
            Some(e) => card::is_relate_to_effect(d, c, e),
        }
        && card::is_able_to_remove(d, c, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return a.eg.as_ref().is_some_and(|eg| group::is_exists(d, eg, &|d, c| filter(d, c, None), 1, NO_EX));
    }
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| filter(d, c, None), NO_EX);
    duel::set_target_card(d, &eg);
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=eg:Filter(s.filter,nil,e)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| filter(d, c, Some(a.e)), NO_EX);
    if !g.is_empty() {
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
