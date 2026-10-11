//! 銃砲撃 (Proton Blast)
use super::prelude::*;

pub const ID: u32 = 49511705;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Apply effects based on the number of heads
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.regcon)
    effect::set_condition(d, e2, Some(condition(regcon)));
    // e2:SetOperation(s.regop)
    effect::set_operation(d, e2, Some(operation(regop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Treat all results as heads
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCondition(s.coincon1)
    effect::set_condition(d, e3, Some(condition(coincon1)));
    // e3:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e3, Some(cost::self_banish()));
    // e3:SetOperation(s.coinop1)
    effect::set_operation(d, e3, Some(operation(coinop1)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.regcon(e,tp,eg,ep,ev,re,r,rp)`
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetOperationInfo(ev,CATEGORY_COIN)
    duel::get_operation_info(d, a.ev, CATEGORY_COIN).is_some()
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Register the results of coin tosses
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_TOSS_COIN)
    effect::set_code(d, e1, EVENT_TOSS_COIN);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetOperation(s.coinregop)
    effect::set_operation(d, e1, Some(operation(coinregop)));
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Apply effects based on the number of heads
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e2, EVENT_CHAIN_SOLVED);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.effcond)
    effect::set_condition(d, e2, Some(condition(effcond)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // e2:SetLabelObject(re)
    effect::set_label_object(d, e2, a.re.map_or(Value::Nil, Value::Effect));
    // e2:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.coinregop(e,tp,eg,ep,ev,re,r,rp)`
fn coinregop(d: &mut Duel, a: &EffArgs) {
    // local ct=aux.GetCoinHeadsFromEv(ev)
    let ct = aux::get_coin_heads_from_ev(a.ev);
    // for i=1,ct do
    for _ in 0..ct {
        // Register a flag for every head
        // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        let c = effect::get_handler(d, a.e).nil();
        card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    }
}
/// `s.effcond(e,tp,eg,ep,ev,re,r,rp)`
fn effcond(d: &mut Duel, a: &EffArgs) -> bool {
    // return re==e:GetLabelObject()
    a.re == effect::get_label_object(d, a.e).as_effect()
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Card hint (the effect always applies, even with no heads in the results)
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // local ct=e:GetHandler():GetFlagEffect(id)
    let c = effect::get_handler(d, a.e).nil();
    let ct = card::get_flag_effect(d, c, ID);
    // if ct<=0 then return end
    if ct == 0 {
        return;
    }
    // if ct>0 then Duel.Damage(1-tp,500,REASON_EFFECT) end
    duel::damage(d, 1 - tp, 500, REASON_EFFECT, false, None);
    // if ct>1 then
    if ct > 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectMatchingCard(tp,nil,tp,0,LOCATION_ONFIELD,1,1,nil)
        let g = duel::select_matching_card(d, tp, None, tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // Duel.Destroy(g,REASON_EFFECT)
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    }
    // if ct>2 then
    if ct > 2 {
        // local hg=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
        let hg = duel::get_field_group(d, tp, 0, LOCATION_HAND);
        // if #hg>0 then
        if !hg.is_empty() {
            // Duel.ConfirmCards(tp,hg)
            duel::confirm_cards(d, tp, &hg);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
            // local sg=hg:Select(tp,1,1,nil)
            let sg = group::select(d, &hg, tp, 1, 1, false, NO_EX).nil();
            // Duel.SendtoGrave(sg,REASON_EFFECT|REASON_DISCARD)
            duel::send_to_grave(d, sg, REASON_EFFECT | REASON_DISCARD, None, None);
            // Duel.ShuffleHand(1-tp)
            duel::shuffle_hand(d, 1 - tp);
        }
    }
}
/// `s.coincon1(e,tp,eg,ep,ev,re,r,rp)`
fn coincon1(d: &mut Duel, a: &EffArgs) -> bool {
    // local ex,eg,et,cp,ct=Duel.GetOperationInfo(ev,CATEGORY_COIN)
    // if ex and ct>1 then
    match duel::get_operation_info(d, a.ev, CATEGORY_COIN) {
        Some(info) if info.op_param > 1 => {
            // e:SetLabelObject(re)
            effect::set_label_object(d, a.e, a.re.map_or(Value::Nil, Value::Effect));
            // return true
            true
        }
        // else return false end
        _ => false,
    }
}
/// `s.coinop1(e,tp,eg,ep,ev,re,r,rp)`
fn coinop1(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local e1=Effect.CreateEffect(e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_TOSS_COIN_NEGATE)
    effect::set_code(d, e1, EVENT_TOSS_COIN_NEGATE);
    // e1:SetCondition(s.coincon2)
    effect::set_condition(d, e1, Some(condition(coincon2)));
    // e1:SetOperation(s.coinop2)
    effect::set_operation(d, e1, Some(operation(coinop2)));
    // e1:SetLabel(ev)
    effect::set_label(d, e1, &[a.ev as i64]);
    // e1:SetLabelObject(e:GetLabelObject())
    let lo = effect::get_label_object(d, a.e);
    effect::set_label_object(d, e1, lo);
    // e1:SetReset(RESET_CHAIN)
    effect::set_reset(d, e1, RESET_CHAIN, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.coincon2(e,tp,eg,ep,ev,re,r,rp)`
fn coincon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return re==e:GetLabelObject() and Duel.GetCurrentChain()==e:GetLabel()
    a.re == effect::get_label_object(d, a.e).as_effect() && duel::get_current_chain(d, false) as i64 == effect::get_label(d, a.e)
}
/// `s.coinop2(e,tp,eg,ep,ev,re,r,rp)`
fn coinop2(d: &mut Duel, a: &EffArgs) {
    // local res={}
    // for i=1,ev do table.insert(res,COIN_HEADS) end
    let res: Vec<u8> = vec![COIN_HEADS as u8; a.ev as usize];
    // Duel.SetCoinResult(table.unpack(res))
    duel::set_coin_result(d, &res);
}
