//! Vertical Landing
use super::prelude::*;

pub const ID: u32 = 904185;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_MECHA_PHANTOM_BEAST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.rfilter(c,ft,tp)
fn rfilter(d: &mut Duel, c: CardId, ft: i32, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WIND) and not c:IsType(TYPE_TOKEN)
    //     and (ft>0 or (c:IsControler(tp) and c:GetSequence()<5)) and (c:IsControler(tp) or c:IsFaceup())
    card::is_attribute(d, c, ATTRIBUTE_WIND)
        && !card::is_type(d, c, TYPE_TOKEN)
        && (ft > 0 || (card::is_controler(d, c, tp) && card::get_sequence(d, c) < 5))
        && (card::is_controler(d, c, tp) || card::is_faceup(d, c))
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then return ft>-1 and Duel.CheckReleaseGroup(tp,s.rfilter,1,nil,ft,tp) end
    if a.chk == Some(0) {
        return ft > -1
            && duel::check_release_group(d, tp, Some(&|d, c| rfilter(d, c, ft, tp)), 1, duel::ReleaseArgs::default(), NO_EX, false);
    }
    // local maxc=10
    let mut maxc = 10;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then maxc=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        maxc = 1;
    }
    // local g=Duel.SelectReleaseGroup(tp,s.rfilter,1,maxc,nil,ft,tp)
    let g = duel::select_release_group(d, tp, Some(&|d, c| rfilter(d, c, ft, tp)), 1, maxc, duel::ReleaseArgs::default(), NO_EX, false).nil();
    // e:SetLabel(#g)
    effect::set_label(d, e, &[g.len() as i64]);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_MECHA_PHANTOM_BEAST,SET_MECHA_PHANTOM_BEAST,TYPES_TOKEN,0,0,3,RACE_MACHINE,ATTRIBUTE_WIND) end
    if a.chk == Some(0) {
        return can_summon_token(d, tp);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,e:GetLabel(),0,0)
    let label = effect::get_label(d, a.e);
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, label as u8, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,e:GetLabel(),tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, label as u8, tp, 0);
    true
}
/// `Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_MECHA_PHANTOM_BEAST,SET_MECHA_PHANTOM_BEAST,TYPES_TOKEN,0,0,3,RACE_MACHINE,ATTRIBUTE_WIND)`
fn can_summon_token(d: &mut Duel, tp: u8) -> bool {
    duel::is_player_can_special_summon_monster(
        d,
        tp,
        TOKEN_MECHA_PHANTOM_BEAST,
        duel::MonsterData {
            setcodes: Some(vec![SET_MECHA_PHANTOM_BEAST as u16]),
            type_: Some(TYPES_TOKEN),
            attack: Some(0),
            defense: Some(0),
            level: Some(3),
            race: Some(RACE_MACHINE),
            attribute: Some(ATTRIBUTE_WIND),
        },
        None,
        None,
        0,
    )
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // if ft<e:GetLabel() then return end
    let label = effect::get_label(d, e);
    if (ft as i64) < label {
        return;
    }
    // if Duel.IsPlayerCanSpecialSummonMonster(...) then
    if can_summon_token(d, tp) {
        // for i=1,e:GetLabel() do
        for _ in 0..label {
            // local token=Duel.CreateToken(tp,TOKEN_MECHA_PHANTOM_BEAST)
            let token = duel::create_token(d, tp, TOKEN_MECHA_PHANTOM_BEAST).nil();
            // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
