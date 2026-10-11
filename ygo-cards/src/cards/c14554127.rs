//! Tiki Peace
use super::prelude::*;

pub const ID: u32 = 14554127;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card as an Effect Monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.selfsptg)
    effect::set_target(d, e1, Some(cost_target(selfsptg)));
    // e1:SetOperation(s.selfspop)
    effect::set_operation(d, e1, Some(operation(selfspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_EFFECT,1800,1800,4,RACE_ROCK,ATTRIBUTE_EARTH)`
fn can_selfsp(d: &mut Duel, tp: u8) -> bool {
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_player_can_special_summon_monster(
            d,
            tp,
            ID,
            duel::MonsterData {
                setcodes: Some(vec![0]),
                type_: Some(TYPE_MONSTER | TYPE_EFFECT),
                attack: Some(1800),
                defense: Some(1800),
                level: Some(4),
                race: Some(RACE_ROCK),
                attribute: Some(ATTRIBUTE_EARTH),
            },
            None,
            None,
            0,
        )
}
/// `s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsPlayerCanSpecialSummonMonster(...) end
    if a.chk == Some(0) {
        return can_selfsp(d, tp);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, tp, 0);
    true
}
/// `s.selfspop(e,tp,eg,ep,ev,re,r,rp)`
fn selfspop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) and s.selfsptg(e,tp,eg,ep,ev,re,r,rp,0) then
    if card::is_relate_to_effect(d, c, e) && can_selfsp(d, tp) {
        // c:AddMonsterAttribute(TYPE_EFFECT|TYPE_TRAP)
        card::add_monster_attribute(d, c, TYPE_EFFECT | TYPE_TRAP, 0, 0, 0, 0, 0);
        // Duel.SpecialSummonStep(c,0,tp,tp,true,false,POS_FACEUP)
        duel::special_summon_step(d, c, 0, tp, tp, true, false, POS_FACEUP as u8, None);
        // Special Summon 1 Continuous Trap, except "Tiki Peace", from your GY or banishment as a Normal Monster and its name becomes "Tiki Peace"
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,1))
        effect::set_description(d, e1, aux::stringid(ID, 1));
        // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
        // e1:SetType(EFFECT_TYPE_QUICK_O)
        effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
        // e1:SetCode(EVENT_FREE_CHAIN)
        effect::set_code(d, e1, EVENT_FREE_CHAIN);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetCountLimit(1,id)
        effect::set_count_limit(d, e1, 1, ID, 0, 0);
        // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
        effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
        // e1:SetCost(Cost.PayLP(800))
        effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(800))));
        // e1:SetTarget(s.sptg)
        effect::set_target(d, e1, Some(cost_target(sptg)));
        // e1:SetOperation(s.spop)
        effect::set_operation(d, e1, Some(operation(spop)));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD, 1);
        // c:RegisterEffect(e1,true)
        card::register_effect(d, c, e1, true);
        // c:AddMonsterAttributeComplete()
        card::add_monster_attribute_complete(d, c);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.spfilter(c,tp)`
fn spfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsContinuousTrap() and c:IsFaceup() and not c:IsCode(id)
    //   and Duel.IsPlayerCanSpecialSummonMonster(tp,id,nil,TYPE_MONSTER|TYPE_NORMAL,1000,1000,4,RACE_ROCK,ATTRIBUTE_EARTH)
    card::is_continuous_trap(d, c)
        && card::is_faceup(d, c)
        && !card::is_code(d, c, &[ID])
        && duel::is_player_can_special_summon_monster(
            d,
            tp,
            ID,
            duel::MonsterData {
                setcodes: None,
                type_: Some(TYPE_MONSTER | TYPE_NORMAL),
                attack: Some(1000),
                defense: Some(1000),
                level: Some(4),
                race: Some(RACE_ROCK),
                attribute: Some(ATTRIBUTE_EARTH),
            },
            None,
            None,
            0,
        )
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, tp)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
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
    // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil,tp):GetFirst()
    let nv = aux::necro_valley_filter(|d, c| spfilter(d, c, tp));
    let g = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    // if not sc then return end
    let Some(sc) = group::get_first(d, &g) else { return };
    // sc:AssumeProperty(ASSUME_CODE,id)
    card::assume_property(d, sc, ASSUME_CODE, ID as u64);
    // if Duel.SpecialSummonStep(sc,0,tp,tp,true,false,POS_FACEUP) then
    if duel::special_summon_step(d, sc, 0, tp, tp, true, false, POS_FACEUP as u8, None) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_CHANGE_TYPE)
        effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
        // e1:SetValue(TYPE_NORMAL|TYPE_MONSTER)
        effect::set_value(d, e1, (TYPE_NORMAL | TYPE_MONSTER) as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD&~RESET_TURN_SET)
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
        // sc:RegisterEffect(e1,true)
        card::register_effect(d, sc, e1, true);
        // local e2=e1:Clone()
        // e2:SetCode(EFFECT_CHANGE_RACE)
        // e2:SetValue(RACE_ROCK)
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EFFECT_CHANGE_RACE);
        effect::set_value(d, e2, RACE_ROCK as i64);
        // sc:RegisterEffect(e2,true)
        card::register_effect(d, sc, e2, true);
        // local e3=e1:Clone()
        // e3:SetCode(EFFECT_CHANGE_ATTRIBUTE)
        // e3:SetValue(ATTRIBUTE_EARTH)
        let e3 = effect::clone(d, e1);
        effect::set_code(d, e3, EFFECT_CHANGE_ATTRIBUTE);
        effect::set_value(d, e3, ATTRIBUTE_EARTH as i64);
        // sc:RegisterEffect(e3,true)
        card::register_effect(d, sc, e3, true);
        // local e4=e1:Clone()
        // e4:SetCode(EFFECT_CHANGE_LEVEL)
        // e4:SetValue(4)
        let e4 = effect::clone(d, e1);
        effect::set_code(d, e4, EFFECT_CHANGE_LEVEL);
        effect::set_value(d, e4, 4);
        // sc:RegisterEffect(e4,true)
        card::register_effect(d, sc, e4, true);
        // local e5=e1:Clone()
        // e5:SetCode(EFFECT_SET_BASE_ATTACK)
        // e5:SetValue(1000)
        let e5 = effect::clone(d, e1);
        effect::set_code(d, e5, EFFECT_SET_BASE_ATTACK);
        effect::set_value(d, e5, 1000);
        // sc:RegisterEffect(e5,true)
        card::register_effect(d, sc, e5, true);
        // local e6=e1:Clone()
        // e6:SetCode(EFFECT_SET_BASE_DEFENSE)
        // e6:SetValue(1000)
        let e6 = effect::clone(d, e1);
        effect::set_code(d, e6, EFFECT_SET_BASE_DEFENSE);
        effect::set_value(d, e6, 1000);
        // sc:RegisterEffect(e6,true)
        card::register_effect(d, sc, e6, true);
        // local e7=e1:Clone()
        // e7:SetCode(EFFECT_CHANGE_CODE)
        // e7:SetValue(id)
        let e7 = effect::clone(d, e1);
        effect::set_code(d, e7, EFFECT_CHANGE_CODE);
        effect::set_value(d, e7, ID as i64);
        // sc:RegisterEffect(e7,true)
        card::register_effect(d, sc, e7, true);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
