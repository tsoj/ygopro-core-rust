//! Fists of the Unrivaled Tenyi
use super::prelude::*;

pub const ID: u32 = 21834870;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_NEGATE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_condition(d, e1, Some(condition(negcon)));
    effect::set_target(d, e1, Some(cost_target(negtg)));
    effect::set_operation(d, e1, Some(operation(negop)));
    card::register_effect(d, c, e1, false);
    // Special summon 1 non-Effect monster
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_DESTROYED);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
/// `s.negcon(e,...)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsNonEffectMonster),tp,LOCATION_MZONE,0,1,nil) then return false end
    if !duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_non_effect_monster(d, c)), a.tp, LOCATION_MZONE, 0, 1, NO_EX) {
        return false;
    }
    // return Duel.IsChainNegatable(ev) and (re:IsMonsterEffect() or re:IsHasType(EFFECT_TYPE_ACTIVATE))
    duel::is_chain_negatable(d, a.ev as u8) && {
        let re = a.re.nil();
        effect::is_monster_effect(d, re) || effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
    }
}
/// `s.negtg(e,...,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    true
}
/// `s.negop(e,...)`
fn negop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        if effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
            let h = effect::get_handler(d, re).nil();
            if card::is_relate_to_effect(d, h, re) {
                // Duel.SendtoGrave(eg,REASON_EFFECT)
                duel::send_to_grave(d, a.eg(), REASON_EFFECT, None, None);
            }
        }
    }
}
/// `s.spcon(e,...)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsReason(REASON_EFFECT) and rp==1-tp and c:IsPreviousLocation(LOCATION_ONFIELD) and c:IsPreviousPosition(POS_FACEDOWN)
    card::is_reason(d, c, REASON_EFFECT) && a.rp == 1 - a.tp && card::is_previous_location(d, c, LOCATION_ONFIELD) && card::is_previous_position(d, c, POS_FACEDOWN)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false) and not c:IsType(TYPE_EFFECT) and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && !card::is_type(d, c, TYPE_EFFECT)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
}
/// `s.sptg(e,...,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop(e,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
