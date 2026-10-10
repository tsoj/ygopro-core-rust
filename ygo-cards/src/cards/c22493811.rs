//! Multiplication of Ants
use super::prelude::*;

pub const ID: u32 = 22493811;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[22493812], ..ScriptInfo::DEFAULT };

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
// s.cfilter(c,ft,tp)
fn cfilter(d: &mut Duel, c: CardId, ft: i32, tp: u8) -> bool {
    // return c:IsRace(RACE_INSECT) and (ft>1 or (c:GetSequence()<5 and c:IsControler(tp)))
    card::is_race(d, c, RACE_INSECT) && (ft > 1 || (card::get_sequence(d, c) < 5 && card::is_controler(d, c, tp)))
}
// s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,nil,nil,ft,tp) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&|d, x| cfilter(d, x, ft, tp)), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,nil,nil,ft,tp)
    let g = duel::select_release_group_cost(d, tp, Some(&|d, x| cfilter(d, x, ft, tp)), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
// s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local check=not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,500,1200,4,RACE_INSECT,ATTRIBUTE_EARTH)
        let check = duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::is_player_can_special_summon_monster(
                d,
                tp,
                ID + 1,
                duel::MonsterData {
                    setcodes: Some(vec![0]),
                    type_: Some(TYPES_TOKEN),
                    attack: Some(500),
                    defense: Some(1200),
                    level: Some(4),
                    race: Some(RACE_INSECT),
                    attribute: Some(ATTRIBUTE_EARTH),
                },
                None,
                None,
                0,
            );
        // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
        let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
        // if e:GetLabel()==1 then
        if effect::get_label(d, a.e) == 1 {
            // e:SetLabel(0)
            effect::set_label(d, a.e, &[0]);
            // return ft>0 and check
            return ft > 0 && check;
        } else {
            // return ft>1 and check
            return ft > 1 && check;
        }
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 2, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, 0);
    true
}
// s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and Duel.GetLocationCount(tp,LOCATION_MZONE)>1
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,500,1200,4,RACE_INSECT,ATTRIBUTE_EARTH) then
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 1
        && duel::is_player_can_special_summon_monster(
            d,
            tp,
            ID + 1,
            duel::MonsterData {
                setcodes: Some(vec![0]),
                type_: Some(TYPES_TOKEN),
                attack: Some(500),
                defense: Some(1200),
                level: Some(4),
                race: Some(RACE_INSECT),
                attribute: Some(ATTRIBUTE_EARTH),
            },
            None,
            None,
            0,
        )
    {
        // for i=1,2 do
        for _ in 0..2 {
            // local token=Duel.CreateToken(tp,id+1)
            let token = duel::create_token(d, tp, ID + 1).nil();
            // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UNRELEASABLE_SUM)
            effect::set_code(d, e1, EFFECT_UNRELEASABLE_SUM);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetValue(1)
            effect::set_value(d, e1, 1);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // token:RegisterEffect(e1,true)
            card::register_effect(d, token, e1, true);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
