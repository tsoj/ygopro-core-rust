//! Trickstar Foxglove Witch
use super::prelude::*;

pub const ID: u32 = 86750474;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TRICKSTAR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_FAIRY),2)
    link::add_procedure(d, c, Some(mat_filter(|d, x, sc, st, tp| card::is_race_ctx(d, x, RACE_FAIRY, sc, st, Some(tp)))), 2, None, None, None, None);
    // damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.damtg)
    effect::set_target(d, e1, Some(cost_target(damtg)));
    // e1:SetOperation(s.damop)
    effect::set_operation(d, e1, Some(operation(damop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.damcon)
    effect::set_condition(d, e2, Some(condition(damcon)));
    // e2:SetTarget(s.damtg2)
    effect::set_target(d, e2, Some(cost_target(damtg2)));
    // e2:SetOperation(s.damop2)
    effect::set_operation(d, e2, Some(operation(damop2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD) > 0;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,200)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 200);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local d=Duel.GetFieldGroupCount(p,LOCATION_ONFIELD,0)*200
    let dmg = duel::get_field_group_count(d, p, LOCATION_ONFIELD, 0) as i64 * 200;
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, p, dmg, REASON_EFFECT, false, None);
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return (r&REASON_EFFECT+REASON_BATTLE)~=0 and c:IsLinkSummoned() and c:IsPreviousLocation(LOCATION_MZONE)
    (a.r & (REASON_EFFECT + REASON_BATTLE)) != 0 && card::is_link_summoned(d, c) && card::is_previous_location(d, c, LOCATION_MZONE)
}
/// `s.damfilter(c,e,tp)`
fn damfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_TRICKSTAR) and c:IsLinkBelow(2) and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_TRICKSTAR as u16])
        && card::is_link_below(d, c, 2)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.damtg2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.damfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let (e, tp) = (a.e, a.tp);
        return duel::is_existing_matching_card(d, Some(&|d, c| damfilter(d, c, e, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,200)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 200);
    true
}
/// `s.damop2(e,tp,eg,ep,ev,re,r,rp)`
fn damop2(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, e);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.damfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| damfilter(d, c, e, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)~=0 then
    if !g.is_empty() && duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None) != 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
        let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
        // local d=Duel.GetFieldGroupCount(p,LOCATION_ONFIELD,0)*200
        let dmg = duel::get_field_group_count(d, p, LOCATION_ONFIELD, 0) as i64 * 200;
        // Duel.Damage(p,d,REASON_EFFECT)
        duel::damage(d, p, dmg, REASON_EFFECT, false, None);
    }
}
