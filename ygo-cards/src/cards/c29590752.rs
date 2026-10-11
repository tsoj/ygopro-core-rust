//! Gladiator Beast Octavius
use super::prelude::*;

pub const ID: u32 = 29590752;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(aux.gbspcon)
    effect::set_condition(d, e1, Some(aux::gbspcon()));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // discard or return
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_HANDES+CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_HANDES + CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_PHASE|PHASE_BATTLE)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_BATTLE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.dcon)
    effect::set_condition(d, e2, Some(condition(dcon)));
    // e2:SetTarget(s.dtg)
    effect::set_target(d, e2, Some(cost_target(dtg)));
    // e2:SetOperation(s.dop)
    effect::set_operation(d, e2, Some(operation(dop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.desfilter(c)
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() and c:GetSequence()~=5
    card::is_facedown(d, c) && card::get_sequence(d, c) != 5
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_SZONE) and s.desfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_SZONE) && desfilter(d, chkc);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.desfilter,tp,LOCATION_SZONE,LOCATION_SZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&desfilter), a.tp, LOCATION_SZONE, LOCATION_SZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsFacedown() and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_facedown(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
        }
    }
}
// function s.dcon(e,tp,eg,ep,ev,re,r,rp)
fn dcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetBattledGroupCount()>0 and Duel.IsTurnPlayer(tp)
    let c = effect::get_handler(d, a.e).nil();
    card::get_battled_group_count(d, c) > 0 && duel::is_turn_player(d, a.tp)
}
// function s.dtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn dtg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():GetFlagEffect(id+1)==0 end
    if a.chk == Some(0) {
        return card::get_flag_effect(d, c, ID + 1) == 0;
    }
    // e:GetHandler():RegisterFlagEffect(id+1,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE,EFFECT_FLAG_OATH,1)
    card::register_flag_effect(d, c, ID + 1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, EFFECT_FLAG_OATH, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 1, a.tp, 0);
    true
}
// function s.dop(e,tp,eg,ep,ev,re,r,rp)
fn dop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsFaceup() or not c:IsRelateToEffect(e) then return end
    if !card::is_faceup(d, c) || !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) > 0 && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
        // Duel.DiscardHand(tp,aux.TRUE,1,1,REASON_EFFECT|REASON_DISCARD)
        duel::discard_hand(d, a.tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
    } else {
        // Duel.SendtoDeck(c,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, CardSet::from_card(c), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
