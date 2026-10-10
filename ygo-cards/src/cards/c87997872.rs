//! Theinen the Great Sphinx
use super::prelude::*;

pub const ID: u32 = 87997872;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // spsummon condition
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    card::register_effect(d, c, e1, false);
    // special summon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_cost(d, e2, Some(cost::pay_lp(cost::PayLp::Amount(500))));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_range(d, e3, LOCATION_DECK);
    card::register_effect(d, c, e3, false);
    // atkup
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_category(d, e4, CATEGORY_ATKCHANGE);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    effect::set_cost(d, e4, Some(cost::pay_lp(cost::PayLp::Amount(500))));
    effect::set_operation(d, e4, Some(operation(atkop)));
    card::register_effect(d, c, e4, false);
}
/// `function s.cfilter(c,tp,code)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8, code: u32) -> bool {
    card::is_code(d, c, &[code]) && card::is_previous_controler(d, c, tp) && card::is_reason(d, c, REASON_DESTROY)
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return eg:IsExists(s.cfilter,1,nil,tp,15013468) and eg:IsExists(s.cfilter,1,nil,tp,51402177)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, tp, 15013468), 1, NO_EX)
        && group::is_exists(d, &eg, &|d, c| cfilter(d, c, tp, 51402177), 1, NO_EX)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let h = effect::get_handler(d, e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,true,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, h, e, 0, tp, true, false, None, None, None);
    }
    // if e:GetHandler():IsLocation(LOCATION_DECK) then Duel.ConfirmCards(1-tp,e:GetHandler()) end
    if card::is_location(d, h, LOCATION_DECK) {
        duel::confirm_cards(d, 1 - tp, h);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if Duel.SpecialSummon(c,0,tp,tp,true,false,POS_FACEUP)~=0 then c:CompleteProcedure() end
    if duel::special_summon(d, c, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None) != 0 {
        card::complete_procedure(d, c);
    }
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, 3000);
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        card::register_effect(d, c, e1, false);
    }
}
