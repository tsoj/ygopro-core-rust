//! Red Reboot
use super::prelude::*;

pub const ID: u32 = 23002292;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate the activation of a Trap Card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Can be activated from the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_TRAP_ACT_IN_HAND)
    effect::set_code(d, e2, EFFECT_TRAP_ACT_IN_HAND);
    // e2:SetValue(function(e,c) e:SetLabel(1) end)
    effect::set_value_fn(d, e2, effect_value(value_fn));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // e1:SetLabelObject(e2)
    effect::set_label_object(d, e1, Value::Effect(e2));
}
/// `function(e,c) e:SetLabel(1) end`
fn value_fn(d: &mut Duel, e: EffectId) -> i64 {
    // e:SetLabel(1)
    effect::set_label(d, e, &[1]);
    0
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and re:IsTrapEffect() and re:IsHasType(EFFECT_TYPE_ACTIVATE) and Duel.IsChainNegatable(ev)
    a.rp == 1 - a.tp
        && effect::is_trap_effect(d, a.re.nil())
        && effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE)
        && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.setfilter(c,tp)`
fn setfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsTrap() and c:IsSSetable(false,1-tp)
    card::is_trap(d, c) && card::is_s_setable(d, c, false, Some(1 - tp))
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then e:GetLabelObject():SetLabel(0) return true end
    if a.chk == Some(0) {
        let lo = effect::get_label_object(d, a.e).as_effect().nil();
        effect::set_label(d, lo, &[0]);
        return true;
    }
    // if e:GetLabelObject():GetLabel()>0 then
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    if effect::get_label(d, lo) > 0 {
        // e:GetLabelObject():SetLabel(0)
        effect::set_label(d, lo, &[0]);
        // Duel.PayLPCost(tp,Duel.GetLP(tp)//2)
        let half = duel::get_lp(d, a.tp).div_euclid(2);
        duel::pay_lp_cost(d, a.tp, half as u32);
    }
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, a.re.nil()).nil();
    // if Duel.NegateActivation(ev) and rc:IsRelateToEffect(re) and rc:IsSSetable(true) then
    if duel::negate_activation(d, a.ev as u8) && card::is_relate_to_effect(d, rc, a.re.nil()) && card::is_s_setable(d, rc, true, None) {
        // rc:CancelToGrave()
        card::cancel_to_grave(d, rc, true);
        // Duel.ChangePosition(rc,POS_FACEDOWN)
        duel::change_position(d, rc, POS_FACEDOWN as u8, None, None, None, false);
        // rc:SetStatus(STATUS_ACTIVATE_DISABLED,false)
        card::set_status(d, rc, STATUS_ACTIVATE_DISABLED, false);
        // rc:SetStatus(STATUS_SET_TURN,false)
        card::set_status(d, rc, STATUS_SET_TURN, false);
        // Duel.RaiseEvent(rc,EVENT_SSET,e,REASON_EFFECT,tp,tp,0)
        duel::raise_event(d, rc, EVENT_SSET, Some(a.e), REASON_EFFECT, a.tp, a.tp, 0);
        // local g=Duel.GetMatchingGroup(s.setfilter,tp,0,LOCATION_DECK,nil,tp)
        let g = duel::get_matching_group(d, Some(&|d, c| setfilter(d, c, a.tp)), a.tp, 0, LOCATION_DECK, NO_EX);
        // if #g>0 and Duel.GetLocationCount(1-tp,LOCATION_SZONE)>0 and Duel.SelectYesNo(1-tp,aux.Stringid(id,2)) then
        if !g.is_empty()
            && duel::get_location_count(d, 1 - a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::select_yes_no(d, 1 - a.tp, aux::stringid(ID, 2))
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
            // local sg=g:Select(1-tp,1,1,nil)
            let sg = group::select(d, &g, 1 - a.tp, 1, 1, false, NO_EX).nil();
            // Duel.SSet(1-tp,sg:GetFirst())
            let first = group::get_first(d, &sg).nil();
            duel::s_set(d, 1 - a.tp, first, None, true);
        }
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e1, aux::stringid(ID, 3));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetValue(s.aclimit)
    effect::set_value_fn(d, e1, lua_fn(aclimit));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // return re:GetHandler():IsTrap() and re:IsHasType(EFFECT_TYPE_ACTIVATE)
    let re = a.e(1);
    let rc = effect::get_handler(d, re).nil();
    card::is_trap(d, rc) && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
}
