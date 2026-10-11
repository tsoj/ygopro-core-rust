//! The Weather Snowy Canvas
use super::prelude::*;

pub const ID: u32 = 80577258;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_THE_WEATHER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // Activate
    // local e1=Effect.CreateEffect(c)
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    // e1:SetCode(EVENT_FREE_CHAIN)
    // c:RegisterEffect(e1)
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // effect gain
    // local e2=Effect.CreateEffect(c)
    // e2:SetDescription(aux.Stringid(id,0))
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    // e2:SetCode(EVENT_FREE_CHAIN)
    // e2:SetRange(LOCATION_MZONE)
    // e2:SetCost(s.announcecost)
    // e2:SetTarget(s.thtg)
    // e2:SetOperation(s.thop)
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_cost(d, e2, Some(cost_target(announcecost)));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    // local e3=Effect.CreateEffect(c)
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_GRANT)
    // e3:SetRange(LOCATION_SZONE)
    // e3:SetTargetRange(LOCATION_MZONE,0)
    // e3:SetTarget(s.eftg)
    // e3:SetLabelObject(e2)
    // c:RegisterEffect(e3)
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_GRANT);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    effect::set_target(d, e3, Some(effect_card_fn(eftg)));
    effect::set_label_object(d, e3, Value::Effect(e2));
    card::register_effect(d, c, e3, false);
}
// function s.eftg(e,c)
fn eftg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local g=e:GetHandler():GetColumnGroup(1,1)
    let h = effect::get_handler(d, e).nil();
    let g = card::get_column_group(d, h, 1, 1);
    // return c:IsType(TYPE_EFFECT) and c:IsSetCard(SET_THE_WEATHER) and c:GetSequence()<5 and g:IsContains(c)
    card::is_type(d, c, TYPE_EFFECT) && card::is_set_card(d, c, &[SET_THE_WEATHER as u16]) && card::get_sequence(d, c) < 5 && group::is_contains(&g, c)
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_THE_WEATHER) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_THE_WEATHER as u16]) && card::is_able_to_hand(d, c, None)
}
// function s.announcecost(e,tp,eg,ep,ev,re,r,rp,chk)
fn announcecost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return aux.bfgcost(e,tp,eg,ep,ev,re,r,rp,chk) end
    if a.chk == Some(0) {
        return bfgcost_call(d, a, 0);
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, desc);
    // aux.bfgcost(e,tp,eg,ep,ev,re,r,rp,chk)
    bfgcost_call(d, a, 1)
}
/// `aux.bfgcost(e,tp,eg,ep,ev,re,r,rp,chk)` with the arguments of the current call
fn bfgcost_call(d: &mut Duel, a: &EffArgs, chk: i64) -> bool {
    let args = [
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        Value::effect(a.re),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
        Value::Int(chk),
    ];
    aux::bfgcost().call(d, &args).truthy()
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, 0, 0);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
    let h = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(e:GetHandler())
    // e1:SetType(EFFECT_TYPE_FIELD)
    // e1:SetCode(EFFECT_CANNOT_TO_HAND)
    // e1:SetTargetRange(LOCATION_DECK,0)
    // e1:SetReset(RESET_PHASE|PHASE_END)
    // Duel.RegisterEffect(e1,tp)
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_TO_HAND);
    effect::set_target_range(d, e1, LOCATION_DECK, 0);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
    // local e2=Effect.CreateEffect(e:GetHandler())
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT+EFFECT_FLAG_OATH)
    // e2:SetDescription(aux.Stringid(id,1))
    // e2:SetReset(RESET_PHASE|PHASE_END)
    // e2:SetTargetRange(1,0)
    // Duel.RegisterEffect(e2,tp)
    let e2 = effect::create_effect(d, h);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT + EFFECT_FLAG_OATH, 0);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    effect::set_target_range(d, e2, 1, 0);
    duel::register_effect(d, e2, tp);
}
