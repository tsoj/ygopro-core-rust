//! Swarm of Centipedes
use super::prelude::*;

pub const ID: u32 = 6116731;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If this card is flipped face-up: You can target 1 other monster on the field; change it to face-up Attack Position or face-down Defense Position. You can only use this effect of "Swarm of Centipedes" once per turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FLIP)
    effect::set_code(d, e1, EVENT_FLIP);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.postg)
    effect::set_target(d, e1, Some(cost_target(postg)));
    // e1:SetOperation(s.posop)
    effect::set_operation(d, e1, Some(operation(posop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Once per turn, during your Main Phase: You can change this card to face-down Defense Position
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_POSITION + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.selfpostg)
    effect::set_target(d, e2, Some(cost_target(selfpostg)));
    // e2:SetOperation(s.selfposop)
    effect::set_operation(d, e2, Some(operation(selfposop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.posfilter(c)`
fn posfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCanTurnSet() or not c:IsPosition(POS_FACEUP_ATTACK)
    card::is_can_turn_set(d, c) || !card::is_position(d, c, POS_FACEUP_ATTACK)
}
/// `s.postg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.posfilter(chkc) and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && posfilter(d, chkc) && chkc != c;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.posfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&posfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_POSCHANGE as u64);
    // local g=Duel.SelectTarget(tp,s.posfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,c)
    let g = duel::select_target(d, a.tp, Some(&posfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,1,tp,POS_FACEUP_ATTACK|POS_FACEDOWN_DEFENSE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, g, 1, a.tp, (POS_FACEUP_ATTACK | POS_FACEDOWN_DEFENSE) as i32);
    true
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local opt=0
    let mut opt = 0;
    // if (tc:IsPosition(POS_FACEDOWN) or tc:IsType(TYPE_TOKEN)) then
    if card::is_position(d, tc, POS_FACEDOWN) || card::is_type(d, tc, TYPE_TOKEN) {
        // opt=POS_FACEUP_ATTACK
        opt = POS_FACEUP_ATTACK;
    } else if card::is_position(d, tc, POS_FACEUP_ATTACK) {
        // opt=POS_FACEDOWN_DEFENSE
        opt = POS_FACEDOWN_DEFENSE;
    } else if card::is_position(d, tc, POS_FACEUP_DEFENSE) {
        // opt=POS_FACEUP_ATTACK|POS_FACEDOWN_DEFENSE
        opt = POS_FACEUP_ATTACK | POS_FACEDOWN_DEFENSE;
    }
    // if opt==0 then return end
    if opt == 0 {
        return;
    }
    // local pos=Duel.SelectPosition(tp,tc,opt)
    let pos = duel::select_position(d, a.tp, tc, opt as u8);
    // if pos==0 then return end
    if pos == 0 {
        return;
    }
    // Duel.ChangePosition(tc,pos)
    duel::change_position(d, tc, pos as u8, None, None, None, false);
}
/// `s.selfpostg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfpostg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanTurnSet() and not c:HasFlagEffect(id) end
    if a.chk == Some(0) {
        return card::is_can_turn_set(d, c) && !card::has_flag_effect(d, c, ID, None);
    }
    // c:RegisterFlagEffect(id,RESET_EVENT|(RESETS_STANDARD&~RESET_TURN_SET)|RESET_PHASE|PHASE_END,0,1)
    card::register_flag_effect(d, c, ID, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET) | RESET_PHASE | PHASE_END, 0, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,c,1,tp,POS_FACEDOWN_DEFENSE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(CardSet::from_card(c)), 1, a.tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
/// `s.selfposop(e,tp,eg,ep,ev,re,r,rp)`
fn selfposop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.ChangePosition(c,POS_FACEDOWN_DEFENSE)
        duel::change_position(d, c, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
    }
}
