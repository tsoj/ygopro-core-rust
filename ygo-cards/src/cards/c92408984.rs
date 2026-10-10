//! The Dragon's Bead
use super::prelude::*;

pub const ID: u32 = 92408984;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Negate and destroy
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_DISABLE + CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_condition(d, e2, Some(condition(discon)));
    effect::set_cost(d, e2, Some(cost_target(discost)));
    effect::set_target(d, e2, Some(cost_target(distg)));
    effect::set_operation(d, e2, Some(operation(disop)));
    card::register_effect(d, c, e2, false);
    // aux.DoubleSnareValidity(c,LOCATION_SZONE)
    aux::double_snare_validity(d, Some(c), LOCATION_SZONE, None);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_MZONE) and c:IsFaceup() and c:IsRace(RACE_DRAGON)
    card::is_location(d, c, LOCATION_MZONE) && card::is_faceup(d, c) && card::is_race(d, c, RACE_DRAGON)
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    // if not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) or not (re:IsTrapEffect() and re:IsHasType(EFFECT_TYPE_ACTIVATE)) then return false end
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) || !(effect::is_trap_effect(d, re) && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)) {
        return false;
    }
    // local tg=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let tg = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned());
    // return tg and tg:IsExists(s.cfilter,1,nil) and Duel.IsChainDisablable(ev)
    match tg {
        Some(g) if group::is_exists(d, &g, &cfilter, 1, NO_EX) => duel::is_chain_disablable(d, a.ev as u8),
        _ => false,
    }
}
/// `s.discost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn discost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), a.tp, LOCATION_HAND, 0, 1, h.into());
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, a.tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    // if eg:GetFirst():IsOnField() then
    let eg = a.eg.clone().nil();
    let first = group::get_first(d, &eg).nil();
    if card::is_on_field(d, first) {
        // Duel.SetTargetCard(eg)
        duel::set_target_card(d, eg.clone());
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsRelateToEffect(e) and Duel.NegateEffect(ev) and re:GetHandler():IsRelateToEffect(re) then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) && duel::negate_effect(d, a.ev as u8) {
        let re = a.re.nil();
        let rh = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rh, re) {
            // Duel.Destroy(eg,REASON_EFFECT)
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
