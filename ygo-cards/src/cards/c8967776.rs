//! Sephylon, the Ultimate Timelord
use super::prelude::*;

pub const ID: u32 = 8967776;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // cannot special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(aux.FALSE)
    effect::set_value_fn(d, e1, aux_false());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // spsummon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // spsummon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    let tp = card::get_controler(d, c);
    // return Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(Card.IsMonster,c:GetControler(),LOCATION_GRAVE,0,10,nil)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, cc| card::is_monster(d, cc)), tp, LOCATION_GRAVE, 0, 10, NO_EX)
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_FAIRY) and c:GetLevel()>=8 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_FAIRY) && card::get_level(d, c) >= 8 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_GRAVE|LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, tp)), tp, LOCATION_GRAVE | LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE|LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_GRAVE | LOCATION_HAND) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.filter),tp,LOCATION_GRAVE|LOCATION_HAND,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(|d, cc| filter(d, cc, a.e, tp))), tp, LOCATION_GRAVE | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e1, EFFECT_DISABLE);
    // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_DISABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
    // e2:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
    effect::set_reset(d, e2, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
    // tc:RegisterEffect(e2)
    card::register_effect(d, tc, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_SET_ATTACK)
    effect::set_code(d, e3, EFFECT_SET_ATTACK);
    // e3:SetValue(4000)
    effect::set_value(d, e3, 4000);
    // e3:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
    effect::set_reset(d, e3, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
    // tc:RegisterEffect(e3)
    card::register_effect(d, tc, e3, false);
    // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
}
