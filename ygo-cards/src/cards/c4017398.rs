//! Zexal Entrust
//! Logical Nonsense
//! Substitute ID
use super::prelude::*;

pub const ID: u32 = 4017398;

// Lists "Utopia", "ZW -", "ZS -", and "ZEXAL" archetypes; specifically lists itself
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_UTOPIA, SET_ZEXAL, SET_ZW, SET_ZS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add or special summon 1 "Utopia", "ZW -", or "ZS -" monster from GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 1));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Add 1 "ZEXAL" spell/trap from GY to hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_condition(d, e2, Some(condition(thcon)));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}
/// `s.ssfilter(c,ft,e,tp)`
fn ssfilter(d: &mut Duel, c: CardId, ft: i32, e: EffectId, tp: u8) -> bool {
    (card::is_set_card(d, c, &[SET_UTOPIA as u16]) || card::is_set_card(d, c, &[SET_ZW as u16]) || card::is_set_card(d, c, &[SET_ZS as u16]))
        && card::is_monster(d, c)
        && ((ft > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), None, None)) || card::is_able_to_hand(d, c, None))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && ssfilter(d, chkc, ft, a.e, tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| ssfilter(d, c, ft, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.ssfilter,tp,LOCATION_GRAVE,0,nil,ft,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| ssfilter(d, c, ft, a.e, tp)), tp, LOCATION_GRAVE, 0, NO_EX);
    // if #g>0 then
    if group::get_count(&g) > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SELECT)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SELECT as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // Duel.SetTargetCard(sg)
        duel::set_target_card(d, sg.clone());
        // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(g), 1, 0, 0);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_GRAVE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,0)
    duel::hint(d, HINT_SELECTMSG as u8, tp, 0);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // local sc=Duel.GetFirstTarget()
    let sc = duel::first_target(d);
    // if sc and sc:IsRelateToEffect(e) then
    if let Some(sc) = sc {
        if card::is_relate_to_effect(d, sc, a.e) {
            // aux.ToHandOrElse(sc,tp,function(c) ... end, function(c) ... end, 2)
            let check = |d: &mut Duel, _c: CardId| card::is_can_be_special_summoned(d, sc, a.e, 0, tp, false, false, Some(POS_FACEUP as u8), None, None) && ft > 0;
            let oper = |d: &mut Duel, _g: &CardSet| -> i32 {
                duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
                0
            };
            aux::to_hand_or_else(d, sc, tp, Some(&check), Some(&oper), Some(2));
        }
    }
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`: "except the turn it was sent, if opponent has 2000+ LP than you"
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_lp(d, a.tp) <= duel::get_lp(d, 1 - a.tp) - 2000 && aux::exccon().call(d, &[Value::Effect(a.e)]).truthy()
}
/// `s.thfilter(c)`: a "ZEXAL" spell/trap except "ZEXAL Entrust"
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_spell_trap(d, c) && card::is_set_card(d, c, &[SET_ZEXAL as u16]) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && thfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 1, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) then Duel.SendtoHand(tc,nil,REASON_EFFECT) end
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) {
            duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        }
    }
}
