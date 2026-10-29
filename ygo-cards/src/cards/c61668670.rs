//! G Golem Crystal Heart
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 61668670;
pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[0x20c], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0x20c)
    card::enable_counter_permit(d, c, 0x20c, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_CYBERSE),2)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_CYBERSE, sc, st, Some(tp)))),
        2,
        None,
        None,
        None,
        None,
    );
    // Special Summon 1 EARTH Link monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_COUNTER);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Increase ATK of co-linked EARTH monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetTarget(s.ecfilter)
    effect::set_target(d, e2, Some(effect_card_fn(ecfilter)));
    // e2:SetValue(s.atkvalue)
    effect::set_value_fn(d, e2, effect_value(atkvalue));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Additional attack to co-linked EARTH monsters
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_EXTRA_ATTACK)
    effect::set_code(d, e3, EFFECT_EXTRA_ATTACK);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    // e3:SetTarget(s.ecfilter)
    effect::set_target(d, e3, Some(effect_card_fn(ecfilter)));
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Grants piercing damage to co-linked EARTH monsters
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e4, EFFECT_PIERCE);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e4, LOCATION_MZONE, LOCATION_MZONE);
    // e4:SetTarget(s.ecfilter)
    effect::set_target(d, e4, Some(effect_card_fn(ecfilter)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.spfilter(c,e,tp,zone)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, zone: u32) -> bool {
    // return c:IsLinkMonster() and c:IsAttribute(ATTRIBUTE_EARTH) and zone~=0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,tp,zone)
    card::is_link_monster(d, c)
        && card::is_attribute(d, c, ATTRIBUTE_EARTH)
        && zone != 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(zone))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local zone=c:GetFreeLinkedZone()&ZONES_MMZ
    let zone = card::get_free_linked_zone(d, c, None) & ZONES_MMZ;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.spfilter(chkc,e,tp,zone) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, e, tp, zone);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp,zone) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, x| spfilter(d, x, e, tp, zone)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp,zone)
    let g = duel::select_target(d, tp, Some(&|d, x| spfilter(d, x, e, tp, zone)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // local zone=e:GetHandler():GetFreeLinkedZone()&ZONES_MMZ
    let zone = card::get_free_linked_zone(d, c, None) & ZONES_MMZ;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and zone~=0 and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP,zone)>0 then
    if card::is_relate_to_effect(d, tc, e) && zone != 0 && duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, Some(zone)) > 0 {
        // c:AddCounter(0x20c,1)
        card::add_counter(d, c, 0x20c, 1, false);
    }
}
/// `s.ecfilter(e,c)`
fn ecfilter(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return e:GetHandler():GetMutualLinkedGroup():IsContains(c) and c:IsAttribute(ATTRIBUTE_EARTH)
    let handler = effect::get_handler(d, e).nil();
    let g = card::get_mutual_linked_group(d, handler);
    group::is_contains(&g, c) && card::is_attribute(d, c, ATTRIBUTE_EARTH)
}
/// `s.atkvalue(e,c)`
fn atkvalue(d: &mut Duel, e: EffectId) -> i64 {
    // return e:GetHandler():GetCounter(0x20c)*600
    let handler = effect::get_handler(d, e).nil();
    card::get_counter(d, handler, 0x20c) as i64 * 600
}
