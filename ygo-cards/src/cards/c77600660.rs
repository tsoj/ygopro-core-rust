//! Psychic Emperor
use super::prelude::*;

pub const ID: u32 = 77600660;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // recover
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e1, CATEGORY_RECOVER);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTarget(s.rectg)
    effect::set_target(d, e1, Some(cost_target(rectg)));
    // e1:SetOperation(s.recop)
    effect::set_operation(d, e1, Some(operation(recop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.rectg(e,tp,...,chk)`
fn rectg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local val=Duel.GetMatchingGroupCount(Card.IsRace,tp,LOCATION_GRAVE,0,nil,RACE_PSYCHIC)*500
    let val = duel::get_matching_group_count(d, Some(&|d, c| card::is_race(d, c, RACE_PSYCHIC)), a.tp, LOCATION_GRAVE, 0, NO_EX) as i64 * 500;
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(val)
    duel::set_target_param(d, val as u32);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,val)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, val as i32);
    true
}
/// `s.recop(e,tp,...)`
fn recop(d: &mut Duel, a: &EffArgs) {
    // local val=Duel.GetMatchingGroupCount(Card.IsRace,tp,LOCATION_GRAVE,0,nil,RACE_PSYCHIC)*500
    let val = duel::get_matching_group_count(d, Some(&|d, c| card::is_race(d, c, RACE_PSYCHIC)), a.tp, LOCATION_GRAVE, 0, NO_EX) as i64 * 500;
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // Duel.Recover(p,val,REASON_EFFECT)
    duel::recover(d, p, val, REASON_EFFECT, false, None);
}
