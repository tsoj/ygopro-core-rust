//! Minerva, the Exalted Lightsworn
use super::prelude::*;

pub const ID: u32 = 30100551;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LIGHTSWORN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,nil,4,2)
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // discard deck & draw
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DECKDES+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DECKDES + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e1:SetTarget(s.distg)
    effect::set_target(d, e1, Some(cost_target(distg)));
    // e1:SetOperation(s.drop)
    effect::set_operation(d, e1, Some(operation(drop_op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // discard deck & destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_DECKDES+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DECKDES + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.distg(e,tp,...,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDiscardDeck(tp,3) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, a.tp, 3);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(3)
    duel::set_target_param(d, 3);
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,3)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, a.tp, 3);
    true
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_LIGHTSWORN) and c:IsLocation(LOCATION_GRAVE)
    card::is_set_card(d, c, &[SET_LIGHTSWORN as u16]) && card::is_location(d, c, LOCATION_GRAVE)
}
/// `s.drop(e,tp,...)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let (p, n) = (info[0].int() as u8, info[1].int() as u16);
    // Duel.DiscardDeck(p,d,REASON_EFFECT)
    duel::discard_deck(d, p, n, REASON_EFFECT);
    // local g=Duel.GetOperatedGroup()
    let g = duel::get_operated_group(d);
    // local ct=g:FilterCount(s.cfilter,nil)
    let ct = group::filter_count(d, &g, &cfilter, NO_EX);
    // if ct>0 then
    if ct > 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,ct,REASON_EFFECT)
        duel::draw(d, a.tp, ct as u32, REASON_EFFECT);
    }
}
/// `s.descon(e,tp,...)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsReason(REASON_BATTLE) or (rp~=tp and c:IsReason(REASON_EFFECT) and c:IsPreviousControler(tp))
    card::is_reason(d, c, REASON_BATTLE) || (a.rp != a.tp && card::is_reason(d, c, REASON_EFFECT) && card::is_previous_controler(d, c, a.tp))
}
/// `s.desop(e,tp,...)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let (p, n) = (info[0].int() as u8, info[1].int() as u16);
    // Duel.DiscardDeck(p,d,REASON_EFFECT)
    duel::discard_deck(d, p, n, REASON_EFFECT);
    // local g=Duel.GetOperatedGroup()
    let g = duel::get_operated_group(d);
    // local ct=g:FilterCount(s.cfilter,nil)
    let ct = group::filter_count(d, &g, &cfilter, NO_EX);
    // local dg=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let dg = duel::get_matching_group(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // if ct~=0 and #dg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if ct != 0 && !dg.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0)) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        // local sdg=dg:Select(tp,1,ct,nil)
        let sdg = group::select(d, &dg, a.tp, 1, ct as u16, false, NO_EX).nil();
        // Duel.HintSelection(sdg)
        duel::hint_selection(d, sdg.clone(), true);
        // Duel.Destroy(sdg,REASON_EFFECT)
        duel::destroy(d, sdg, REASON_EFFECT, None, None);
    }
}
