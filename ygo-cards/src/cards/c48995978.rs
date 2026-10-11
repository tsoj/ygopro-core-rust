//! No.88: Gimmick Puppet of Leo
use super::prelude::*;

pub const ID: u32 = 48995978;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[0x2b], xyz_number: Some(88), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // c:EnableCounterPermit(0x2b)
    card::enable_counter_permit(d, c, 0x2b, None, None);
    // Xyz Summon Procedure
    // Xyz.AddProcedure(c,nil,8,3)
    xyz::add_procedure(d, c, None, Some(8), 3, None, None, None, None, false, None);
    // Place 1 Destiny Counter on this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_COUNTER);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(function(e,tp) return Duel.GetFieldGroupCount(tp,LOCATION_STZONE,0)==0 end)
    effect::set_condition(d, e1, Some(condition(cond1)));
    // e1:SetCost(s.ctcost)
    effect::set_cost(d, e1, Some(cost_target(ctcost)));
    // e1:SetTarget(s.cttg)
    effect::set_target(d, e1, Some(cost_target(cttg)));
    // e1:SetOperation(s.ctop)
    effect::set_operation(d, e1, Some(operation(ctop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If 3 Destiny counters are on this card, you win the Duel
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetOperation(s.winop)
    effect::set_operation(d, e2, Some(operation(winop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,tp) return Duel.GetFieldGroupCount(tp,LOCATION_STZONE,0)==0 end`
fn cond1(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_field_group_count(d, a.tp, LOCATION_STZONE, 0) == 0
}
/// `s.ctcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn ctcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPhase(PHASE_MAIN1) end
    if a.chk == Some(0) {
        return duel::is_phase(d, PHASE_MAIN1);
    }
    // Cannot conduct your Battle Phase this turn
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_BP)
    effect::set_code(d, e1, EFFECT_CANNOT_BP);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    true
}
/// `s.cttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:CheckRemoveOverlayCard(tp,1,REASON_EFFECT) and c:IsCanAddCounter(0x2b,1) end
    if a.chk == Some(0) {
        return card::check_remove_overlay_card(d, c, tp, 1, REASON_EFFECT) && card::is_can_add_counter(d, c, 0x2b, 1, false, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,1,tp,0x2b)
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, 1, tp, 0x2b);
    true
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() and c:RemoveOverlayCard(tp,1,1,REASON_EFFECT)>0 then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) && card::remove_overlay_card(d, c, tp, 1, 1, REASON_EFFECT) > 0 {
        // c:AddCounter(0x2b,1)
        card::add_counter(d, c, 0x2b, 1, false);
    }
}
/// `s.winop(e,tp,eg,ep,ev,re,r,rp)`
fn winop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:GetCounter(0x2b)==3 then
    if card::get_counter(d, c, 0x2b) == 3 {
        // Duel.Win(tp,WIN_REASON_PUPPET_LEO)
        duel::win(d, a.tp, WIN_REASON_PUPPET_LEO);
    }
}
