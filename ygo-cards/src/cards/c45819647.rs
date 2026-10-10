//! Selene, Queen of the Master Magicians
use super::prelude::*;

pub const ID: u32 = 45819647;
pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_SPELL], listed_series: &[SET_ENDYMION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_SPELL)
    card::enable_counter_permit(d, c, COUNTER_SPELL as u16, None, None);
    // link summon
    link::add_procedure(d, c, None, 2, Some(3), Some(group_check(lcheck)), None, None);
    card::enable_revive_limit(d, c);
    // add counter
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_COUNTER);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e1, Some(condition(ctcon)));
    effect::set_target(d, e1, Some(cost_target(cttg)));
    effect::set_operation(d, e1, Some(operation(ctop)));
    card::register_effect(d, c, e1, false);
    // cannot be attacked
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_CANNOT_BE_BATTLE_TARGET);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(cacon)));
    effect::set_value_fn(d, e2, aux::imval2());
    card::register_effect(d, c, e2, false);
    // special summon
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_hint_timing(d, e3, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(spcon)));
    effect::set_cost(d, e3, Some(cost_target(spcost)));
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
}
/// `s.lcheck(g,lc,sumtype,tp)`
fn lcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(Card.IsRace,1,nil,RACE_SPELLCASTER,lc,sumtype,tp)
    group::is_exists(d, g, &|d, c| card::is_race_ctx(d, c, RACE_SPELLCASTER, Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
/// `s.ctcon(e,...)`: `return e:GetHandler():IsLinkSummoned()`
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, h)
}
/// `s.ctfilter(c)`
fn ctfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpell() and (c:IsLocation(LOCATION_GRAVE) or c:IsFaceup() or c:GetEquipTarget() or c:IsLocation(LOCATION_FZONE))
    card::is_spell(d, c)
        && (card::is_location(d, c, LOCATION_GRAVE)
            || card::is_faceup(d, c)
            || card::get_equip_target(d, c).is_some()
            || card::is_location(d, c, LOCATION_FZONE))
}
/// `s.cttg(e,...,chk)`
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local ct=Duel.GetMatchingGroupCount(s.ctfilter,tp,LOCATION_ONFIELD|LOCATION_GRAVE,LOCATION_ONFIELD|LOCATION_GRAVE,nil)
    let ct = duel::get_matching_group_count(d, Some(&ctfilter), a.tp, LOCATION_ONFIELD | LOCATION_GRAVE, LOCATION_ONFIELD | LOCATION_GRAVE, NO_EX);
    // if ct>0 then Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,ct,0,COUNTER_SPELL) end
    if ct > 0 {
        duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, ct as u8, 0, COUNTER_SPELL as i32);
    }
    true
}
/// `s.ctop(e,...)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=Duel.GetMatchingGroupCount(s.ctfilter,tp,LOCATION_ONFIELD|LOCATION_GRAVE,LOCATION_ONFIELD|LOCATION_GRAVE,nil)
    let ct = duel::get_matching_group_count(d, Some(&ctfilter), a.tp, LOCATION_ONFIELD | LOCATION_GRAVE, LOCATION_ONFIELD | LOCATION_GRAVE, NO_EX);
    // if c:IsRelateToEffect(e) and ct>0 then c:AddCounter(COUNTER_SPELL,ct) end
    if card::is_relate_to_effect(d, c, a.e) && ct > 0 {
        card::add_counter(d, c, COUNTER_SPELL as u16, ct as u16, false);
    }
}
/// `s.cacon(e,...)`
fn cacon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_ENDYMION),0,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_ENDYMION as u16]))), 0, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
}
/// `s.spcon(e,...)`
fn spcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // local ph=Duel.GetCurrentPhase()
    let _ph = duel::get_current_phase(d);
    // return Duel.IsMainPhase()
    duel::is_main_phase(d, None)
}
/// `s.spcost(e,...,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsCanRemoveCounter(tp,1,0,COUNTER_SPELL,3,REASON_COST) end
    if a.chk == Some(0) {
        return duel::is_can_remove_counter(d, a.tp, 1, 0, COUNTER_SPELL as u16, 3, REASON_COST);
    }
    // Duel.RemoveCounter(tp,1,0,COUNTER_SPELL,3,REASON_COST)
    duel::remove_counter(d, a.tp, 1, 0, COUNTER_SPELL as u16, 3, REASON_COST);
    true
}
/// `s.spfilter(c,e,tp,zone)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, zone: u32) -> bool {
    // return c:IsRace(RACE_SPELLCASTER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE,tp,zone)
    card::is_race(d, c, RACE_SPELLCASTER)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), Some(tp), Some(zone))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // local zone=e:GetHandler():GetLinkedZone(tp)
        let h = effect::get_handler(d, a.e).nil();
        let zone = card::get_linked_zone(d, h, Some(a.tp));
        // return zone~=0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,tp,zone)
        return zone != 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, zone)), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local zone=e:GetHandler():GetLinkedZone(tp)
    let h = effect::get_handler(d, a.e).nil();
    let zone = card::get_linked_zone(d, h, Some(a.tp));
    // if zone==0 then return end
    if zone == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,e,tp,zone)
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp, zone))),
        a.tp,
        LOCATION_HAND | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP_DEFENSE,zone) end
    if !g.is_empty() {
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, Some(zone));
    }
}
