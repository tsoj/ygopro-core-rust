//! Vampire Takeover
use super::prelude::*;

pub const ID: u32 = 22900598;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[62188962], listed_series: &[SET_VAMPIRE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // if Duel.GetFieldCard(tp,LOCATION_FZONE,0)~=nil then return false end
    if duel::get_field_card(d, a.tp, LOCATION_FZONE, 0).is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, 0, NO_EX);
    // return #g>0 and g:FilterCount(Card.IsRace,nil,RACE_ZOMBIE)==#g
    !g.is_empty() && group::filter_count(d, &g, &|d, c| card::is_race(d, c, RACE_ZOMBIE), NO_EX) == g.len()
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsCode(62188962) and c:GetActivateEffect():IsActivatable(tp,true,true)
    card::is_code(d, c, &[62188962]) && {
        let te = card::get_activate_effect(d, c).first().copied().nil();
        effect::is_activatable(d, te, tp, true, true)
    }
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_VAMPIRE) and c:IsAttribute(ATTRIBUTE_DARK)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_set_card(d, c, &[SET_VAMPIRE as u16])
        && card::is_attribute(d, c, ATTRIBUTE_DARK)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,tp) end
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local tc=Duel.GetFirstMatchingCard(s.filter,tp,LOCATION_DECK,0,nil,tp)
    let tc = duel::get_first_matching_card(d, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_DECK, 0, NO_EX);
    // if Duel.ActivateFieldSpell(tc,e,tp,eg,ep,ev,re,r,rp) then
    if duel::activate_field_spell(d, tc, a, None) {
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        // local sg=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,nil,e,tp)
        let sg = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, e, tp))), tp, LOCATION_GRAVE, 0, NO_EX);
        // if #sg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
        if !sg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local g=sg:Select(tp,1,1,nil)
            let g = group::select(d, &sg, tp, 1, 1, false, NO_EX).nil();
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        }
    }
}
