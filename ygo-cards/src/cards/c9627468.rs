//! Humongous Hive Hegemon - Zexstagger
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 9627468;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Gain 300 ATK for every other Insect monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.hspcon)
    effect::set_condition(d, e2, Some(condition(hspcon)));
    // e2:SetTarget(s.hsptg)
    effect::set_target(d, e2, Some(cost_target(hsptg)));
    // e2:SetOperation(s.hspop)
    effect::set_operation(d, e2, Some(operation(hspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Each player can summon 1 Insect monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // return Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsRace,RACE_INSECT),0,LOCATION_MZONE,LOCATION_MZONE,e:GetHandler())*300
    duel::get_matching_group_count(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_INSECT)), 0, LOCATION_MZONE, LOCATION_MZONE, c.into()) as i64 * 300
}
/// `s.hspcon(e,tp,eg,ep,ev,re,r,rp)`
fn hspcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(aux.FaceupFilter(Card.IsRace,RACE_INSECT),1,nil)
    group::is_exists(d, &a.eg.clone().nil(), &|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_INSECT), 1, NO_EX)
}
/// `s.hsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn hsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.hspop(e,tp,eg,ep,ev,re,r,rp)`
fn hspop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,PLAYER_ALL,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, PLAYER_ALL as u8, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_INSECT) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_INSECT) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.insectsp(e,p)`
fn insectsp(d: &mut Duel, e: EffectId, p: u8) {
    // if Duel.GetLocationCount(p,LOCATION_MZONE)==0 then return end
    if duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return;
    }
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),p,LOCATION_HAND|LOCATION_GRAVE,0,nil,e,p)
    let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, e, p))), p, LOCATION_HAND | LOCATION_GRAVE, 0, NO_EX);
    // if #g==0 or not Duel.SelectYesNo(p,aux.Stringid(id,2)) then return end
    if g.is_empty() || !duel::select_yes_no(d, p, aux::stringid(ID, 2)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,p,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_SPSUMMON as u64);
    // local tc=g:Select(p,1,1,nil):GetFirst()
    let sel = group::select(d, &g, p, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if tc and Duel.SpecialSummonStep(tc,0,p,p,false,false,POS_FACEUP) then
    if let Some(tc) = tc {
        if duel::special_summon_step(d, tc, 0, p, p, false, false, POS_FACEUP as u8, None) {
            // Negate its effects
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
            effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
            // e1:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e1, EFFECT_DISABLE);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_DISABLE_EFFECT)
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            // e2:SetValue(RESET_TURN_SET)
            effect::set_value(d, e2, RESET_TURN_SET as i64);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local turn_p=Duel.GetTurnPlayer()
    let turn_p = duel::get_turn_player(d);
    // s.insectsp(e,turn_p)
    insectsp(d, a.e, turn_p);
    // s.insectsp(e,1-turn_p)
    insectsp(d, a.e, 1 - turn_p);
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
