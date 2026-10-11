//! Borrel Reboot
//! Scripted by ahtelel
use super::prelude::*;

pub const ID: u32 = 87607094;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ROKKET, SET_BORREL],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate the activation of an opponent's Spell/Trap Card, and if you do, Set that card
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
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You can activate this card from your hand by paying half your LP
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_TRAP_ACT_IN_HAND)
    effect::set_code(d, e2, EFFECT_TRAP_ACT_IN_HAND);
    // e2:SetValue(function(e,c) e:SetLabel(1) end)
    effect::set_value_fn(
        d,
        e2,
        lua_fn(|d, a: Args| -> Value {
            effect::set_label(d, a.e(0), &[1]);
            Value::Nil
        }),
    );
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // e1:SetLabelObject(e2)
    effect::set_label_object(d, e1, Value::Effect(e2));
}
/// `s.condition(e,tp,...)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let re = a.re.nil();
    // return rp==1-tp and re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsSpellTrapEffect() and Duel.IsChainNegatable(ev)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,{SET_ROKKET,SET_BORREL}),tp,LOCATION_MZONE,0,1,nil)
    a.rp == 1 - tp
        && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
        && effect::is_spell_trap_effect(d, re)
        && duel::is_chain_negatable(d, a.ev as u8)
        && duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ROKKET as u16, SET_BORREL as u16])),
            tp,
            LOCATION_MZONE,
            0,
            1,
            NO_EX,
        )
}
/// `s.cost(e,tp,...,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    // if chk==0 then e:GetLabelObject():SetLabel(0) return true end
    if a.chk == Some(0) {
        effect::set_label(d, lo, &[0]);
        return true;
    }
    // if e:GetLabelObject():GetLabel()>0 then
    if effect::get_label(d, lo) > 0 {
        // e:GetLabelObject():SetLabel(0)
        effect::set_label(d, lo, &[0]);
        // Duel.PayLPCost(tp,Duel.GetLP(tp)//2)
        let lp = duel::get_lp(d, a.tp);
        duel::pay_lp_cost(d, a.tp, lp.div_euclid(2) as u32);
    }
    true
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, a.tp, 0);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // if Duel.NegateActivation(ev) and rc:IsRelateToEffect(re) and rc:IsSSetable(true) then
    if duel::negate_activation(d, a.ev as u8) && card::is_relate_to_effect(d, rc, re) && card::is_s_setable(d, rc, true, None) {
        // rc:CancelToGrave()
        card::cancel_to_grave(d, rc, true);
        // Duel.ChangePosition(rc,POS_FACEDOWN)
        duel::change_position(d, rc, POS_FACEDOWN as u8, None, None, None, false);
        // rc:SetStatus(STATUS_ACTIVATE_DISABLED,false)
        card::set_status(d, rc, STATUS_ACTIVATE_DISABLED, false);
        // rc:SetStatus(STATUS_SET_TURN,false)
        card::set_status(d, rc, STATUS_SET_TURN, false);
        // Duel.RaiseEvent(rc,EVENT_SSET,e,REASON_EFFECT,tp,tp,0)
        duel::raise_event(d, rc, EVENT_SSET, Some(a.e), REASON_EFFECT, tp, tp, 0);
        // --It cannot be activated this turn
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(aux.Stringid(id,2))
        effect::set_description(d, e1, aux::stringid(ID, 2));
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_TRIGGER)
        effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
        // e1:SetReset(RESET_EVENT|RESETS_CANNOT_ACT|RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_CANNOT_ACT | RESET_PHASE | PHASE_END, 1);
        // rc:RegisterEffect(e1)
        card::register_effect(d, rc, e1, false);
    }
}
