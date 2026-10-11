//! Bamboo Scrap
use super::prelude::*;

pub const ID: u32 = 28062325;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[28062326], ..ScriptInfo::DEFAULT };

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
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// s.cfilter(c,ft,tp)
fn cfilter(d: &mut Duel, c: CardId, ft: i32, tp: u8) -> bool {
    // return c:IsRace(RACE_PLANT) and (ft>1 or (c:GetSequence()<5 and c:IsControler(1-tp)))
    //   and (c:IsFaceup() or c:IsControler(tp))
    card::is_race(d, c, RACE_PLANT as u64)
        && (ft > 1 || (card::get_sequence(d, c) < 5 && card::is_controler(d, c, 1 - tp)))
        && (card::is_faceup(d, c) || card::is_controler(d, c, tp))
}
// s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // local ft=Duel.GetLocationCount(1-tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,nil,nil,ft,tp) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&|d, c| cfilter(d, c, ft, a.tp)), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,nil,nil,ft,tp)
    let g = duel::select_release_group_cost(d, a.tp, Some(&|d, c| cfilter(d, c, ft, a.tp)), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
// s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()==0 and Duel.GetLocationCount(1-tp,LOCATION_MZONE)<=1 then return false end
        if effect::get_label(d, a.e) == 0 && duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 1 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
        //   and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,800,500,1,RACE_PLANT,ATTRIBUTE_EARTH,POS_FACEUP_DEFENSE,1-tp)
        return !duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() && can_summon_token(d, a.tp);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 2, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, 0);
    true
}
// Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,800,500,1,RACE_PLANT,ATTRIBUTE_EARTH,POS_FACEUP_DEFENSE,1-tp)
fn can_summon_token(d: &mut Duel, tp: u8) -> bool {
    let md = duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(800),
        defense: Some(500),
        level: Some(1),
        race: Some(RACE_PLANT),
        attribute: Some(ATTRIBUTE_EARTH),
    };
    duel::is_player_can_special_summon_monster(d, tp, ID + 1, md, Some(POS_FACEUP_DEFENSE as u8), Some(1 - tp), 0)
}
// s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) or Duel.GetLocationCount(1-tp,LOCATION_MZONE)<2
    //   or not Duel.IsPlayerCanSpecialSummonMonster(...) then return end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()
        || duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 < 2
        || !can_summon_token(d, tp)
    {
        return;
    }
    // for i=1,2 do
    for _ in 1..=2 {
        // local token=Duel.CreateToken(tp,id+1)
        let token = duel::create_token(d, tp, ID + 1).nil();
        // Duel.SpecialSummonStep(token,0,tp,1-tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon_step(d, token, 0, tp, 1 - tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
