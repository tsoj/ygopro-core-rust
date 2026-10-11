//! Barrage Blast
use super::prelude::*;

pub const ID: u32 = 51369889;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy any number of cards on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_END_PHASE));
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCost(s.rmcost)
    effect::set_cost(d, e2, Some(cost_target(rmcost)));
    // e2:SetTarget(s.rmtg)
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    // e2:SetOperation(s.rmop)
    effect::set_operation(d, e2, Some(operation(rmop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Inflict damage to the opponent
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCondition(s.damcon)
    effect::set_condition(d, e3, Some(condition(damcon)));
    // e3:SetCost(s.damcost)
    effect::set_cost(d, e3, Some(cost_target(damcost)));
    // e3:SetTarget(s.damtg)
    effect::set_target(d, e3, Some(cost_target(damtg)));
    // e3:SetOperation(s.damop)
    effect::set_operation(d, e3, Some(operation(damop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.rmfilter(c)
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_MACHINE) and c:IsType(TYPE_XYZ)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_MACHINE) && card::is_type(d, c, TYPE_XYZ)
}
// function s.rmcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn rmcost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    // return true
    true
}
// function s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsOnField() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc);
    }
    // local dg=Duel.GetMatchingGroup(s.rmfilter,tp,LOCATION_MZONE,0,nil)
    let dg = duel::get_matching_group(d, Some(&rmfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()==100 then
        if effect::get_label(d, e) == 100 {
            // e:SetLabel(0)
            effect::set_label(d, e, &[0]);
            // return Duel.CheckRemoveOverlayCard(tp,0,0,1,REASON_COST,dg) and Duel.IsExistingTarget(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,e:GetHandler())
            let c = effect::get_handler(d, e).nil();
            return duel::check_remove_overlay_card(d, tp, 0, 0, 1, REASON_COST, Some(&dg))
                && duel::is_existing_target(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
        } else {
            // return false
            return false;
        }
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVEXYZ)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVEXYZ as u64);
    // local rt=Duel.GetTargetCount(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler())
    let c = effect::get_handler(d, e).nil();
    let rt = duel::get_target_count(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, c.into());
    // local count=Duel.RemoveOverlayCard(tp,0,0,1,rt,REASON_COST,dg)
    let count = duel::remove_overlay_card(d, tp, 0, 0, 1, rt as u16, REASON_COST, Some(&dg));
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESREPLACE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESREPLACE as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,count,count,nil)
    let g = duel::select_target(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, count as u16, count as u16, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
// function s.rmop(e,tp,eg,ep,ev,re,r,rp)
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
// function s.desfilter(c,e,tp)
fn desfilter(d: &mut Duel, c: CardId, _e: EffectId, tp: u8) -> bool {
    // return (c:IsReason(REASON_BATTLE) or (c:IsReason(REASON_EFFECT) and c:GetReasonPlayer()==1-tp))
    //     and c:IsPreviousPosition(POS_FACEUP) and c:IsType(TYPE_XYZ) and c:IsRace(RACE_MACHINE)
    //     and c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousControler(tp)
    (card::is_reason(d, c, REASON_BATTLE) || (card::is_reason(d, c, REASON_EFFECT) && card::get_reason_player(d, c) == 1 - tp))
        && card::is_previous_position(d, c, POS_FACEUP)
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_race(d, c, RACE_MACHINE)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_controler(d, c, tp)
}
// function s.damcon(e,tp,eg,ep,ev,re,r,rp)
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.desfilter,1,nil,e,tp)
    let eg = a.eg.clone().nil();
    let (e, tp) = (a.e, a.tp);
    group::is_exists(d, &eg, &|d, c| desfilter(d, c, e, tp), 1, NO_EX)
}
// function s.damfilter(c)
fn damfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_XYZ) and c:IsRace(RACE_MACHINE) and c:IsAbleToRemoveAsCost() and c:GetRank()>0
    card::is_type(d, c, TYPE_XYZ) && card::is_race(d, c, RACE_MACHINE) && card::is_able_to_remove_as_cost(d, c, None) && card::get_rank(d, c) > 0
}
// function s.damcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn damcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return c:IsAbleToRemoveAsCost()
    //     and Duel.IsExistingMatchingCard(s.damfilter,tp,LOCATION_GRAVE,0,1,c) end
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, c, None)
            && duel::is_existing_matching_card(d, Some(&damfilter), tp, LOCATION_GRAVE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.damfilter,tp,LOCATION_GRAVE,0,1,1,c)
    let mut g = duel::select_matching_card(d, tp, Some(&damfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, c.into()).nil();
    // e:SetLabel(g:GetFirst():GetRank())
    let first = group::get_first(d, &g).nil();
    let rank = card::get_rank(d, first);
    effect::set_label(d, e, &[rank as i64]);
    // g:AddCard(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
// function s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - tp);
    // Duel.SetTargetParam(e:GetLabel()*200)
    let dmg = effect::get_label(d, e) * 200;
    duel::set_target_param(d, dmg as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,e:GetLabel()*200)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, dmg as i32);
    true
}
// function s.damop(e,tp,eg,ep,ev,re,r,rp)
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
