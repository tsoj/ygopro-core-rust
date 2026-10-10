//! Reeshaddoll Wendi
use super::prelude::*;

pub const ID: u32 = 51023024;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_SHADDOLL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon (any pos)
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(move |d, a| sptg(d, a, POS_DEFENSE))));
    effect::set_operation(d, e1, Some(operation(move |d, a| spop(d, a, POS_DEFENSE))));
    card::register_effect(d, c, e1, false);
    // special summon (face-down)
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(move |d, a| sptg(d, a, POS_FACEDOWN_DEFENSE))));
    effect::set_operation(d, e2, Some(operation(move |d, a| spop(d, a, POS_FACEDOWN_DEFENSE))));
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter(c,e,tp,pos)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, pos: u32) -> bool {
    // return c:IsSetCard(SET_SHADDOLL) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,pos) and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_SHADDOLL as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(pos as u8), None, None)
        && !card::is_code(d, c, &[ID])
}
/// `s.sptg(pos)`: the target function of the effect with position `pos`
fn sptg(d: &mut Duel, a: &EffArgs, pos: u32) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp,pos) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, pos)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,0,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 0, LOCATION_DECK as i32);
    true
}
/// `s.spop(pos)`: the operation of the effect with position `pos`
fn spop(d: &mut Duel, a: &EffArgs, pos: u32) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,pos)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp, pos)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,pos)
        duel::special_summon(d, g.clone(), 0, a.tp, a.tp, false, false, pos as u8, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, &g);
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsReason(REASON_EFFECT)
    let c = effect::get_handler(d, a.e).nil();
    card::is_reason(d, c, REASON_EFFECT)
}
