//! XZ-Tank Cannon
use super::prelude::*;

pub const ID: u32 = 99724761;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,62651957,64500000)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(62651957), fusion::Mat::Code(64500000)]);
    // Fusion.AddContactProc(c,s.contactfil,s.contactop,s.splimit)
    fusion::add_contact_proc(
        d,
        c,
        fusion::Contact {
            material_filter: Some(std::sync::Arc::new(contactfil)),
            material_operation: Some(std::sync::Arc::new(contactop)),
            sumcon: Some(Some(lua_fn(splimit))),
            ..Default::default()
        },
    );
    // destroy
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_DESTROY);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_cost(d, e3, Some(cost_target(descost)));
    effect::set_target(d, e3, Some(cost_target(destg)));
    effect::set_operation(d, e3, Some(operation(desop)));
    card::register_effect(d, c, e3, false);
}
/// `s.contactfil(tp)`
fn contactfil(d: &mut Duel, tp: u8) -> CardSet {
    duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove_as_cost(d, c, None)), tp, LOCATION_ONFIELD, 0, NO_EX)
}
/// `s.contactop(g)`
fn contactop(d: &mut Duel, g: &CardSet, _tp: u8, _c: CardId) {
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST | REASON_MATERIAL, None, None);
}
/// `s.splimit(e,se,sp,st)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    !card::is_location(d, h, LOCATION_EXTRA | LOCATION_GRAVE)
}
fn discardable(d: &mut Duel, c: CardId) -> bool {
    card::is_discardable(d, c, None, None)
}
/// `s.descost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&discardable), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::discard_hand(d, a.tp, Some(&discardable), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_facedown(d, c)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_SZONE) && card::is_controler(d, chkc, 1 - a.tp) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, 0, LOCATION_SZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, 0, LOCATION_SZONE, 1, 1, false, NO_EX).nil();
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFacedown() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_facedown(d, tc) {
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
