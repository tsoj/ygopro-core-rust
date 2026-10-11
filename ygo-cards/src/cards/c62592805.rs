//! Epurrely Noir
use super::prelude::*;

pub const ID: u32 = 62592805;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[21347668], listed_series: &[SET_PURRELY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon Procedure
    // Xyz.AddProcedure(c,nil,2,2)
    xyz::add_procedure(d, c, None, Some(2), 2, None, None, None, None, false, None);
    // Return cards the opponent controls to the hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(thcost)));
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
    // Attach a "Purrely" Quick-Play Spell
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SET);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 3, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(attachcond)));
    effect::set_operation(d, e2, Some(operation(attachop)));
    card::register_effect(d, c, e2, false);
}
/// `Card.IsDiscardable` (no extra arguments)
fn discardable(d: &mut Duel, c: CardId) -> bool {
    card::is_discardable(d, c, None, None)
}
/// `s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&discardable), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, a.tp, Some(&discardable), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - a.tp) && card::is_able_to_hand(d, chkc, None);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // local ct=e:GetHandler():GetOverlayGroup():IsExists(Card.IsCode,1,nil,21347668) and 2 or 1
    let h = effect::get_handler(d, a.e).nil();
    let overlay = card::get_overlay_group(d, h);
    let ct: u16 = if group::is_exists(d, &overlay, &|d, c| card::is_code(d, c, &[21347668]), 1, NO_EX) { 2 } else { 1 };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToHand,tp,0,LOCATION_ONFIELD,1,ct,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, 0, LOCATION_ONFIELD, 1, ct, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g>0 then Duel.SendtoHand(g,nil,REASON_EFFECT) end
    if !g.is_empty() {
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
/// `s.attachcond(e,tp,eg,ep,ev,re,r,rp)`
fn attachcond(d: &mut Duel, a: &EffArgs) -> bool {
    // if rp==1-tp or not re:IsHasType(EFFECT_TYPE_ACTIVATE) then return false end
    if a.rp == 1 - a.tp {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
        return false;
    }
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, re).nil();
    // return rc:IsSetCard(SET_PURRELY) and rc:IsQuickPlaySpell() and rc:IsOnField()
    //     and rc:IsCanBeXyzMaterial(e:GetHandler(),tp,REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    card::is_set_card(d, rc, &[SET_PURRELY as u16])
        && card::is_quick_play_spell(d, rc)
        && card::is_on_field(d, rc)
        && card::is_can_be_xyz_material(d, rc, Some(h), Some(a.tp), Some(REASON_EFFECT))
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_PURRELY as u16]) && card::is_trap(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.attachop(e,tp,eg,ep,ev,re,r,rp)`
fn attachop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // if c:IsRelateToEffect(e) and rc:IsRelateToEffect(re) and not c:IsImmuneToEffect(e)
    //     and not rc:IsImmuneToEffect(e) and rc:IsCanBeXyzMaterial(c,tp,REASON_EFFECT) then
    if card::is_relate_to_effect(d, c, a.e)
        && card::is_relate_to_effect(d, rc, re)
        && !card::is_immune_to_effect(d, c, a.e)
        && !card::is_immune_to_effect(d, rc, a.e)
        && card::is_can_be_xyz_material(d, rc, Some(c), Some(a.tp), Some(REASON_EFFECT))
    {
        // Duel.Overlay(c,rc)
        duel::overlay(d, c, rc, false);
        // if not c:GetOverlayGroup():IsContains(rc) then return end
        let overlay = card::get_overlay_group(d, c);
        if !group::is_contains(&overlay, rc) {
            return;
        }
        // rc:CancelToGrave()
        card::cancel_to_grave(d, rc, true);
        // local g=Duel.GetMatchingGroup(s.setfilter,tp,LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, NO_EX);
        // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
            // local sg=g:Select(tp,1,1,nil)
            let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SSet(tp,sg)
            duel::s_set(d, a.tp, sg, None, true);
        }
    }
}
