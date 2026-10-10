//! Fiendsmith's Agnumday
use super::prelude::*;

pub const ID: u32 = 32991300;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2+ monsters, including a LIGHT Fiend monster
    // Link.AddProcedure(c,nil,2,3,s.lcheck)
    link::add_procedure(d, c, None, 2, Some(3), Some(std::sync::Arc::new(lcheck)), None, None);
    // Special Summon 1 LIGHT non-Link Fiend monster in your GY and equip it with this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.matfilter(c,lc,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT,lc,sumtype,tp) and c:IsRace(RACE_FIEND,lc,sumtype,tp)
    card::is_attribute_ctx(d, c, ATTRIBUTE_LIGHT, Some(lc), sumtype, Some(tp)) && card::is_race_ctx(d, c, RACE_FIEND, Some(lc), sumtype, Some(tp))
}
/// `s.lcheck(g,lc,sumtype,tp)`
fn lcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(s.matfilter,1,nil,lc,sumtype,tp)
    group::is_exists(d, g, &|d, c| matfilter(d, c, lc, sumtype, tp), 1, NO_EX)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and not c:IsLinkMonster() and c:IsRace(RACE_FIEND)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && !card::is_link_monster(d, c) && card::is_race(d, c, RACE_FIEND) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,...,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && spfilter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(h)), 1, a.tp, 0);
    true
}
/// `s.atkvalfilter(c)`
fn atkvalfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonsterCard() and c:IsOriginalType(TYPE_LINK) and c:IsFaceup()
    card::is_monster_card(d, c) && card::is_original_type(d, c, TYPE_LINK) && card::is_faceup(d, c)
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, a: Args) -> i64 {
    // return 600*c:GetEquipGroup():Match(s.atkvalfilter,nil):GetSum(Card.GetLink)
    let c = a.c(1);
    let mut g = card::get_equip_group(d, c);
    group::match_(d, &mut g, &|d, c| atkvalfilter(d, c), NO_EX);
    600 * group::get_sum(d, &g, &|d, c| card::get_link(d, c) as i64)
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)>0
    //     and c:IsRelateToEffect(e) and c:IsControler(tp) and Duel.Equip(tp,c,tc) then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0
        && card::is_relate_to_effect(d, c, a.e)
        && card::is_controler(d, c, a.tp)
        && duel::equip(d, a.tp, c, tc, true, false)
    {
        // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,1))
        card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 1));
        // The equipped monster gains ATK equal to total Link Rating of Link Monsters equipped to it x 600
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_EQUIP)
        effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(s.atkval)
        effect::set_value_fn(d, e1, lua_fn(atkval));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // The equipped monster inflicts piercing battle damage
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_EQUIP)
        effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
        // e2:SetCode(EFFECT_PIERCE)
        effect::set_code(d, e2, EFFECT_PIERCE);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
        // Equip limit
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e3:SetCode(EFFECT_EQUIP_LIMIT)
        effect::set_code(d, e3, EFFECT_EQUIP_LIMIT);
        // e3:SetValue(function(e,c) return c==tc end)
        effect::set_value_fn(d, e3, lua_fn(move |_d, a: Args| a.card(1) == Some(tc)));
        // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e3)
        card::register_effect(d, c, e3, false);
    }
}
