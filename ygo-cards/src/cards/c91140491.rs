//! Seraphim Papillion
use super::prelude::*;

pub const ID: u32 = 91140491;
const COUNTER_PAPILLON: u16 = 0x14d;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    card::enable_revive_limit(d, c);
    card::enable_counter_permit(d, c, COUNTER_PAPILLON, None, None);
    // Link.AddProcedure(c,nil,2,3,s.lcheck)
    link::add_procedure(d, c, None, 2, Some(3), Some(group_check(lcheck)), None, None);
    // counter
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_COUNTER);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_label(d, e1, &[0]);
    effect::set_condition(d, e1, Some(condition(ctcon)));
    effect::set_operation(d, e1, Some(operation(ctop)));
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_MATERIAL_CHECK);
    effect::set_value_fn(d, e2, effect_card_value(valcheck));
    effect::set_label_object(d, e2, Value::Effect(e1));
    card::register_effect(d, c, e2, false);
    // atk up
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_value_fn(d, e3, effect_card_value(atkval));
    card::register_effect(d, c, e3, false);
    // special summon
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e4, EVENT_FREE_CHAIN);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    effect::set_hint_timing(d, e4, 0, Some(TIMING_END_PHASE as u32));
    effect::set_cost(d, e4, Some(cost_target(spcost)));
    effect::set_target(d, e4, Some(cost_target(sptg)));
    effect::set_operation(d, e4, Some(operation(spop)));
    card::register_effect(d, c, e4, false);
}
fn code_list(d: &mut Duel, c: CardId) -> Vec<i64> {
    card::get_code(d, c).iter().map(|x| *x as i64).collect()
}
/// `s.lcheck(g,lc,sumtype,tp)`: `g:CheckDifferentProperty(Card.GetCode,lc,sumtype,tp)`
fn lcheck(d: &mut Duel, g: &CardSet, _lc: CardId, _sumtype: u64, _tp: u8) -> bool {
    group::check_different_property(d, g, &code_list)
}
/// `s.ctcon(e,tp,eg,ep,ev,re,r,rp)`
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLinkSummoned()
    let c = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, c)
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let ct = effect::get_label(d, a.e);
    // if not (c:IsRelateToEffect(e) and c:IsFaceup() and ct>0) then return end
    if !(card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) && ct > 0) {
        return;
    }
    // c:AddCounter(COUNTER_PAPILLON,ct)
    card::add_counter(d, c, COUNTER_PAPILLON, ct as u16, false);
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // e:GetLabelObject():SetLabel(c:GetMaterial():FilterCount(Card.IsRace,nil,RACE_INSECT,c,SUMMON_TYPE_LINK))
    let e1 = effect::get_label_object(d, e).as_effect().nil();
    let mats = card::get_material(d, c);
    let n = group::filter_count(d, &mats, &|d, x| card::is_race(d, x, RACE_INSECT), NO_EX);
    effect::set_label(d, e1, &[n as i64]);
    // (the function returns nothing: the core reads nil as 0)
    0
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetCounter(COUNTER_PAPILLON)*200
    card::get_counter(d, c, COUNTER_PAPILLON) as i64 * 200
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_can_remove_counter(d, c, a.tp, COUNTER_PAPILLON, 1, REASON_COST);
    }
    // c:RemoveCounter(tp,COUNTER_PAPILLON,1,REASON_COST)
    card::remove_counter(d, c, a.tp, COUNTER_PAPILLON, 1, REASON_COST);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_race(d, c, RACE_INSECT)
        && card::is_level_below(d, c, 4)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp)
        let lc = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
        return lc > 0 && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let f = aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp));
    let g = duel::select_matching_card(d, a.tp, Some(&f), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP_DEFENSE) end
    if !g.is_empty() {
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
