//! Copy Knight
use super::prelude::*;

pub const ID: u32 = 21843307;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,...)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=eg:GetFirst()
    let ec = group::get_first(d, &a.eg.clone().nil()).nil();
    // return ep==tp and ec:IsLevelBelow(4) and ec:IsRace(RACE_WARRIOR)
    a.ep == a.tp && card::is_level_below(d, ec, 4) && card::is_race(d, ec, RACE_WARRIOR)
}
/// `Duel.IsPlayerCanSpecialSummonMonster(tp,ec:GetCode(),0,TYPE_MONSTER|TYPE_NORMAL,0,0,ec:GetLevel(),RACE_WARRIOR,ATTRIBUTE_LIGHT)`
fn can_summon_copy(d: &mut Duel, tp: u8, ec: CardId) -> bool {
    let code = card::get_code(d, ec)[0];
    let lv = card::get_level(d, ec);
    duel::is_player_can_special_summon_monster(
        d,
        tp,
        code,
        duel::MonsterData {
            setcodes: Some(vec![0]),
            type_: Some(TYPE_MONSTER | TYPE_NORMAL),
            attack: Some(0),
            defense: Some(0),
            level: Some(lv),
            race: Some(RACE_WARRIOR),
            attribute: Some(ATTRIBUTE_LIGHT),
        },
        None,
        None,
        0,
    )
}
/// `s.target(e,tp,eg,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=eg:GetFirst()
    let ec = group::get_first(d, &a.eg.clone().nil()).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and
    //     Duel.IsPlayerCanSpecialSummonMonster(...) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && can_summon_copy(d, a.tp, ec);
    }
    // ec:CreateEffectRelation(e)
    card::create_effect_relation(d, ec, a.e);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local ec=eg:GetFirst()
    let ec = group::get_first(d, &a.eg.clone().nil()).nil();
    // if not ec:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, ec, a.e) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0
    //     or not Duel.IsPlayerCanSpecialSummonMonster(...) then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 || !can_summon_copy(d, a.tp, ec) {
        return;
    }
    let lv = card::get_level(d, ec);
    let code = card::get_code(d, ec)[0];
    // c:AddMonsterAttribute(TYPE_NORMAL|TYPE_TRAP,0,0,ec:GetLevel(),0,0)
    card::add_monster_attribute(d, c, TYPE_NORMAL | TYPE_TRAP, 0, 0, lv, 0, 0);
    // Duel.SpecialSummonStep(c,0,tp,tp,true,false,POS_FACEUP)
    duel::special_summon_step(d, c, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None);
    // c:AddMonsterAttributeComplete()
    card::add_monster_attribute_complete(d, c);
    // change code
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetValue(ec:GetCode())
    effect::set_value(d, e1, code as i64);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e1,true)
    card::register_effect(d, c, e1, true);
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
