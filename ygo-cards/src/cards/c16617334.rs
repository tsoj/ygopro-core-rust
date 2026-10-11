//! Performapal Rain Goat
use super::prelude::*;

pub const ID: u32 = 16617334;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PERFORMAPAL, SET_ODD_EYES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Reduce effect damage to 0
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e1:SetCondition(aux.damcon1)
    effect::set_condition(d, e1, Some(aux::damcon1()));
    // e1:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e1, Some(cost::self_discard()));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Targeted "Odd-Eyes" or "Performapal" monster cannot be destroyed by battle or card effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e2, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e2:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e2, Some(cost::self_discard()));
    // e2:SetTarget(s.target2)
    effect::set_target(d, e2, Some(cost_target(target2)));
    // e2:SetOperation(s.operation2)
    effect::set_operation(d, e2, Some(operation(operation2)));
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_MAIN_END)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local cid=Duel.GetChainInfo(ev,CHAININFO_CHAIN_ID)
    let cid = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_CHAIN_ID])[0].int();
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CHANGE_DAMAGE)
    effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetLabel(cid)
    effect::set_label(d, e1, &[cid]);
    // e1:SetValue(s.damcon)
    effect::set_value_fn(d, e1, lua_fn(damcon));
    // e1:SetReset(RESET_CHAIN)
    effect::set_reset(d, e1, RESET_CHAIN, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.damcon(e,re,val,r,rp,rc)`
fn damcon(d: &mut Duel, a: Args) -> i64 {
    let e = a.e(0);
    let val = a.int(2);
    let r = a.int(3) as u32;
    // local cc=Duel.GetCurrentChain()
    let cc = duel::get_current_chain(d, false);
    // if cc==0 or (r&REASON_EFFECT)==0 then return end
    if cc == 0 || (r & REASON_EFFECT) == 0 {
        return 0;
    }
    // local cid=Duel.GetChainInfo(0,CHAININFO_CHAIN_ID)
    let cid = duel::get_chain_info(d, 0, &[CHAININFO_CHAIN_ID])[0].int();
    // if cid==e:GetLabel() then return 0 end
    if cid == effect::get_label(d, e) {
        return 0;
    }
    // return val
    val
}
/// `function(c) return c:IsFaceup() and c:IsSetCard({SET_PERFORMAPAL,SET_ODD_EYES}) end`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_PERFORMAPAL as u16, SET_ODD_EYES as u16])
}
/// `s.target2(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsOnField() and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_on_field(d, chkc) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_ONFIELD,0,1,1,nil)
    duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX);
    true
}
/// `s.operation2(e,tp,eg,ep,ev,re,r,rp)`
fn operation2(d: &mut Duel, a: &EffArgs) {
    let (e, _tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc) {
        // Cannot be destroyed by battle or card effects
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(3008)
        effect::set_description(d, e1, 3008);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
