//! Evilswarm Coppelia
use super::prelude::*;

pub const ID: u32 = 77841719;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:AddCannotBeSpecialSummoned()
    card::add_cannot_be_special_summoned(d, c);
    // Take control of 1 face-up monster your opponent controls until your next End Phase
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_CONTROL);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e1, EVENT_LEAVE_FIELD);
    // e1:SetCondition(s.controlcon)
    effect::set_condition(d, e1, Some(condition(controlcon)));
    // e1:SetTarget(s.controltg)
    effect::set_target(d, e1, Some(cost_target(controltg)));
    // e1:SetOperation(s.controlop)
    effect::set_operation(d, e1, Some(operation(controlop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.controlcon(e,tp,eg,ep,ev,re,r,rp)`
fn controlcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousControler(tp) and rp==1-tp
    card::is_previous_location(d, c, LOCATION_MZONE) && card::is_previous_controler(d, c, a.tp) && a.rp == 1 - a.tp
}
/// `s.controltg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn controltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsControlerCanBeChanged() and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, 1 - tp)
            && card::is_controler_can_be_changed(d, chkc, false, None)
            && card::is_faceup(d, chkc);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsControlerCanBeChanged),tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(
        d,
        tp,
        Some(&aux::faceup_filter(|d, c| card::is_controler_can_be_changed(d, c, false, None))),
        tp,
        0,
        LOCATION_MZONE,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(g), 1, tp, 0);
    }
    true
}
/// `s.controlop(e,tp,eg,ep,ev,re,r,rp)`
fn controlop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
            // local ct=(Duel.IsTurnPlayer(1-tp) and 2) or (Duel.IsPhase(PHASE_END) and 3) or 1
            let ct: u8 = if duel::is_turn_player(d, 1 - tp) {
                2
            } else if duel::is_phase(d, PHASE_END) {
                3
            } else {
                1
            };
            // Duel.GetControl(tc,tp,PHASE_END,ct)
            duel::get_control(d, tc, tp, PHASE_END as u16, ct, None, None);
        }
    }
}
