//! Annihilator Archfiend
use super::prelude::*;

pub const ID: u32 = 29603180;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy 1 card from each side of the field
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_previous_position(d, c, POS_FACEUP as u32)
        && (card::get_previous_race_on_field(d, c) & RACE_FIEND) == RACE_FIEND
        && card::is_previous_location(d, c, LOCATION_MZONE)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chkc().is_some() {
        return false;
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
            && duel::is_existing_target(d, None, a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let mut g1 = duel::select_target(d, a.tp, None, a.tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let g2 = duel::select_target(d, a.tp, None, a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    group::add_card(&mut g1, &g2);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g1), 2, 0, 0);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g>0 and Duel.Destroy(g,REASON_EFFECT)~=0 then
    if !g.is_empty() && duel::destroy(d, g, REASON_EFFECT, None, None) != 0 {
        // local sg=Duel.GetOperatedGroup():Filter(s.filter,nil)
        let operated = duel::get_operated_group(d);
        let sg = group::filter(d, &operated, &filter, NO_EX);
        // if #sg>0 and c:IsRelateToEffect(e) and c:IsFaceup() then
        if !sg.is_empty() && card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
            // local atk=sg:GetSum(Card.GetBaseAttack)/2
            let sum = group::get_sum(d, &sg, &|d: &mut Duel, c: CardId| card::get_base_attack(d, c) as i64);
            let atk = lua_int(sum as f64 / 2.0);
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            effect::set_value(d, e1, atk);
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            card::register_effect(d, c, e1, false);
        }
    }
}
