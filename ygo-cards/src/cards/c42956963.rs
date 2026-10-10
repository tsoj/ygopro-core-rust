//! Nightmare Archfiends
//! ナイトメア・デーモンズ
use super::prelude::*;

pub const ID: u32 = 42956963;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[42956964], ..ScriptInfo::DEFAULT };

/// The Token: "Archfiend Token" data (`Duel.IsPlayerCanSpecialSummonMonster` arguments)
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_ARCHFIEND as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(2000),
        defense: Some(2000),
        level: Some(6),
        race: Some(RACE_FIEND),
        attribute: Some(ATTRIBUTE_DARK),
    }
}

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,nil,1,false,nil,nil) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, None, 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,nil,1,1,false,nil,nil)
    let g = duel::select_release_group_cost(d, tp, None, 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and Duel.GetLocationCount(1-tp,LOCATION_MZONE)>2
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_ARCHFIEND,TYPES_TOKEN,2000,2000,6,RACE_FIEND,ATTRIBUTE_DARK,POS_FACEUP_ATTACK,1-tp) end
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 2
            && duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), Some(POS_FACEUP_ATTACK as u8), Some(1 - tp), 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,3,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 3, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,3,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 3, tp, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) or Duel.GetLocationCount(1-tp,LOCATION_MZONE)<3
    //     or not Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_ARCHFIEND,TYPES_TOKEN,2000,2000,6,RACE_FIEND,ATTRIBUTE_DARK,POS_FACEUP_ATTACK,1-tp) then return end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()
        || duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 < 3
        || !duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), Some(POS_FACEUP_ATTACK as u8), Some(1 - tp), 0)
    {
        return;
    }
    // for i=1,3 do
    for _i in 1..=3 {
        // local token=Duel.CreateToken(tp,id+1)
        let token = duel::create_token(d, tp, ID + 1).nil();
        // if Duel.SpecialSummonStep(token,0,tp,1-tp,false,false,POS_FACEUP_ATTACK) then
        if duel::special_summon_step(d, token, 0, tp, 1 - tp, false, false, POS_FACEUP_ATTACK as u8, None) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
            // e1:SetCode(EVENT_LEAVE_FIELD)
            effect::set_code(d, e1, EVENT_LEAVE_FIELD);
            // e1:SetOperation(s.damop)
            effect::set_operation(d, e1, Some(operation(damop)));
            // token:RegisterEffect(e1,true)
            card::register_effect(d, token, e1, true);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsReason(REASON_DESTROY) then
    if card::is_reason(d, c, REASON_DESTROY) {
        // Duel.Damage(c:GetPreviousControler(),800,REASON_EFFECT)
        let p = card::get_previous_controler(d, c);
        duel::damage(d, p, 800, REASON_EFFECT, false, None);
    }
    // e:Reset()
    effect::reset(d, a.e);
}
