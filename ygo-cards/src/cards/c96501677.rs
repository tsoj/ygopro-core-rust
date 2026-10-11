//! Catnipped Kitty
use super::prelude::*;

pub const ID: u32 = 96501677;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot be battle target
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_CANNOT_BE_BATTLE_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_BATTLE_TARGET);
    // e1:SetCondition(s.ccon)
    effect::set_condition(d, e1, Some(lua_fn(|d, a: Args| ccon(d, a.e(0)))));
    // e1:SetValue(aux.imval1)
    effect::set_value_fn(d, e1, aux::imval1());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // defdown
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DEFCHANGE)
    effect::set_category(d, e2, CATEGORY_DEFCHANGE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.deftg)
    effect::set_target(d, e2, Some(cost_target(deftg)));
    // e2:SetOperation(s.defop)
    effect::set_operation(d, e2, Some(operation(defop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.ccon(e)`
fn ccon(d: &mut Duel, e: EffectId) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_BEAST),e:GetHandlerPlayer(),LOCATION_MZONE,0,1,e:GetHandler())
    let p = effect::get_handler_player(d, e);
    let h = effect::get_handler(d, e).nil();
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_BEAST)), p, LOCATION_MZONE, 0, 1, h.into())
}
/// `s.deftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn deftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:HasNonZeroDefense() and chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) end
    if let Some(chkc) = a.chkc() {
        return card::has_non_zero_defense(d, chkc) && card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_MZONE);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.HasNonZeroDefense,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&card::has_non_zero_defense), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,Card.HasNonZeroDefense,tp,0,LOCATION_MZONE,1,1,nil)
    let _ = duel::select_target(d, tp, Some(&card::has_non_zero_defense), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.defop(e,tp,eg,ep,ev,re,r,rp)`
fn defop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_DEFENSE_FINAL)
        effect::set_code(d, e1, EFFECT_SET_DEFENSE_FINAL);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // e1:SetValue(0)
        effect::set_value(d, e1, 0);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
