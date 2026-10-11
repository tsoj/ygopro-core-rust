//! Mid Shield Gardna
use super::prelude::*;

pub const ID: u32 = 75487237;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // turn set
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_QUICK_F)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_F);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL+EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL + EFFECT_FLAG_SET_AVAILABLE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.negcon)
    effect::set_condition(d, e2, Some(condition(negcon)));
    // e2:SetOperation(s.negop)
    effect::set_operation(d, e2, Some(operation(negop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanTurnSet() and c:GetFlagEffect(id)==0 end
    if a.chk == Some(0) {
        return card::is_can_turn_set(d, c) && card::get_flag_effect(d, c, ID) == 0;
    }
    // c:RegisterFlagEffect(id,RESET_EVENT|(RESETS_STANDARD_PHASE_END&~RESET_TURN_SET),0,1)
    card::register_flag_effect(d, c, ID, RESET_EVENT | (RESETS_STANDARD_PHASE_END & !RESET_TURN_SET), 0, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,c,1,tp,POS_FACEDOWN_DEFENSE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(CardSet::from_card(c)), 1, a.tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // Duel.ChangePosition(c,POS_FACEDOWN_DEFENSE)
        duel::change_position(d, c, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
    }
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    // if re:IsSpellEffect() and re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then
    if effect::is_spell_effect(d, re) && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        // local tg=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
        let tg = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
        let h = effect::get_handler(d, a.e).nil();
        // return #tg==1 and tg:GetFirst()==e:GetHandler() and e:GetHandler():IsFacedown()
        tg.len() == 1 && group::get_first(d, &tg) == Some(h) && card::is_facedown(d, h)
    } else {
        // return false
        false
    }
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    // if Duel.NegateActivation(ev) and re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
        let rc = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rc, re) {
            // Duel.SendtoGrave(eg,REASON_EFFECT)
            duel::send_to_grave(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
    // Duel.ChangePosition(e:GetHandler(),POS_FACEUP_DEFENSE)
    let h = effect::get_handler(d, a.e).nil();
    duel::change_position(d, h, POS_FACEUP_DEFENSE as u8, None, None, None, false);
}
