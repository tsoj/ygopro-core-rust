//! Skilled White Magician
use super::prelude::*;

pub const ID: u32 = 46363422;
pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_SPELL], listed_names: &[CARD_BUSTER_BLADER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_SPELL)
    card::enable_counter_permit(d, c, COUNTER_SPELL as u16, None, None);
    // c:SetCounterLimit(COUNTER_SPELL,3)
    card::set_counter_limit(d, c, COUNTER_SPELL as u16, 3);
    // add counter
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e0, EVENT_CHAINING);
    effect::set_range(d, e0, LOCATION_MZONE);
    effect::set_operation(d, e0, Some(aux::chainreg()));
    card::register_effect(d, c, e0, false);
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_operation(d, e1, Some(operation(acop)));
    card::register_effect(d, c, e1, false);
    // special summon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
/// `s.acop(e,...)`
fn acop(d: &mut Duel, a: &EffArgs) {
    // if re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsSpellEffect() and e:GetHandler():GetFlagEffect(1)>0 then
    let re = a.re.nil();
    if effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && effect::is_spell_effect(d, re) {
        let h = effect::get_handler(d, a.e).nil();
        if card::get_flag_effect(d, h, 1) > 0 {
            // e:GetHandler():AddCounter(COUNTER_SPELL,1)
            let h = effect::get_handler(d, a.e).nil();
            card::add_counter(d, h, COUNTER_SPELL as u16, 1, false);
        }
    }
}
/// `s.spcost(e,...,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // return e:GetHandler():GetCounter(COUNTER_SPELL)==3 and e:GetHandler():IsReleasable()
        let h = effect::get_handler(d, a.e).nil();
        return card::get_counter(d, h, COUNTER_SPELL as u16) == 3 && card::is_releasable(d, h, None, None);
    }
    // Duel.Release(e:GetHandler(),REASON_COST)
    let h = effect::get_handler(d, a.e).nil();
    duel::release(d, h, REASON_COST, None);
    true
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_BUSTER_BLADER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[CARD_BUSTER_BLADER]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,...,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if e:GetHandler():GetSequence()<5 then ft=ft+1 end
    let h = effect::get_handler(d, a.e).nil();
    if card::get_sequence(d, h) < 5 {
        ft += 1;
    }
    if a.chk == Some(0) {
        // return ft>0 and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK|LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,tp)
        return ft > 0
            && duel::is_existing_matching_card(
                d,
                Some(&|d, c| filter(d, c, a.e, a.tp)),
                a.tp,
                LOCATION_DECK | LOCATION_HAND | LOCATION_GRAVE,
                0,
                1,
                NO_EX,
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,0,tp,LOCATION_DECK|LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 0, a.tp, (LOCATION_DECK | LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
/// `s.spop(e,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.filter),tp,LOCATION_DECK|LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,e,tp)
    let f = aux::necro_valley_filter(|d, c| filter(d, c, a.e, a.tp));
    let g = duel::select_matching_card(d, a.tp, Some(&f), a.tp, LOCATION_DECK | LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
