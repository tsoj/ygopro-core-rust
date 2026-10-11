//! Key Man the Key Warrior
//! 鍵戦士キーマン
use super::prelude::*;

pub const ID: u32 = 23168060;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Make this card Level 3 until the End Phase
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_LVCHANGE)
    effect::set_category(d, e1, CATEGORY_LVCHANGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.lvcon)
    effect::set_condition(d, e1, Some(condition(lvcon)));
    // e1:SetOperation(s.lvop)
    effect::set_operation(d, e1, Some(operation(lvop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.lvcon(e,tp,eg,ep,ev,re,r,rp)
fn lvcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:HasLevel() and not c:IsLevel(3)
    card::has_level(d, c) && !card::is_level(d, c, &[3])
}
// function s.lvop(e,tp,eg,ep,ev,re,r,rp)
fn lvop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_LEVEL)
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
        // e1:SetValue(3)
        effect::set_value(d, e1, 3);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
