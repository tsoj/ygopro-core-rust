//! Pendulum Transfer
use super::prelude::*;

pub const ID: u32 = 19501924;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_PENDULUM)
}
/// `s.count_free_pendulum_zones(tp)`
fn count_free_pendulum_zones(d: &Duel, tp: u8) -> i32 {
    let mut count = 0;
    if duel::check_location(d, tp, LOCATION_PZONE, 0) {
        count += 1;
    }
    if duel::check_location(d, tp, LOCATION_PZONE, 1) {
        count += 1;
    }
    count
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        // if chk==0 then return s.count_free_pendulum_zones(tp) > 0
        //     and (Duel.GetLocationCount(tp,LOCATION_SZONE) > 1 or e:GetHandler():IsLocation(LOCATION_SZONE))
        //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil) end
        let handler = effect::get_handler(d, e).nil();
        return count_free_pendulum_zones(d, tp) > 0
            && (duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 1
                || card::is_location(d, handler, LOCATION_SZONE))
            && duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 1));
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,s.count_free_pendulum_zones(tp),nil)
    let max = count_free_pendulum_zones(d, tp) as u16;
    let _g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, max, false, NO_EX);
    true
}
/// `s.move_to_pendulum_zone(c,tp,e)`
fn move_to_pendulum_zone(d: &mut Duel, c: Option<CardId>, tp: u8, e: EffectId) {
    let Some(c) = c else { return };
    if !duel::check_pendulum_zones(d, tp)
        || !card::is_relate_to_effect(d, c, e)
        || !(card::is_controler(d, c, tp) && filter(d, c))
    {
        return;
    }
    // Duel.MoveToField(c,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
    duel::move_to_field(d, c, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // g:GetFirst() / g:GetNext(): the iteration order of the group
    let order = group::iter(d, &g);
    // if #g>0 then
    if !g.is_empty() {
        move_to_pendulum_zone(d, order.first().copied(), tp, a.e);
        move_to_pendulum_zone(d, order.get(1).copied(), tp, a.e);
    }
}
