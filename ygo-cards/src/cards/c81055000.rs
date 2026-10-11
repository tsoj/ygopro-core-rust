//! Performapal Handstandaccoon
use super::prelude::*;

pub const ID: u32 = 81055000;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PERFORMAPAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // to hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.thtg1)
    effect::set_target(d, e1, Some(cost_target(thtg1)));
    // e1:SetOperation(s.thop1)
    effect::set_operation(d, e1, Some(operation(thop1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // to hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    // e2:SetTarget(s.thtg2)
    effect::set_target(d, e2, Some(cost_target(thtg2)));
    // e2:SetOperation(s.thop2)
    effect::set_operation(d, e2, Some(operation(thop2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_PERFORMAPAL) and c:IsAbleToHand()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_PERFORMAPAL as u16]) && card::is_able_to_hand(d, c, None)
}
// s.thtg1(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn thtg1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsControler(tp) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, a.tp) && thfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_ONFIELD,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 1, 0, 0);
    true
}
// s.thop1(e,tp,eg,ep,ev,re,r,rp)
fn thop1(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SendtoHand(tc,nil,REASON_EFFECT)~=0 and tc:IsLocation(LOCATION_HAND) then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::send_to_hand(d, CardSet::from_card(tc), None, REASON_EFFECT, None) != 0
        && card::is_location(d, tc, LOCATION_HAND)
    {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
        effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetLabel(tc:GetCode())
        let code = card::get_code(d, tc)[0] as i64;
        effect::set_label(d, e1, &[code]);
        // e1:SetValue(s.aclimit)
        effect::set_value_fn(d, e1, lua_fn(aclimit));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
// s.aclimit(e,re,tp)
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // return re:GetHandler():IsCode(e:GetLabel())
    let e = a.e(0);
    let re = a.e(1);
    let rh = effect::get_handler(d, re).nil();
    let label = effect::get_label(d, e) as u32;
    card::is_code(d, rh, &[label])
}
// s.thtg2(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn thtg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsAbleToHand() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_able_to_hand(d, chkc, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 1, 0, 0);
    true
}
// s.thop2(e,tp,eg,ep,ev,re,r,rp)
fn thop2(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, CardSet::from_card(tc), None, REASON_EFFECT, None);
    }
}
