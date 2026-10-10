//! Evolzar Lars
use super::prelude::*;

pub const ID: u32 = 35103106;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 6 monsters
    // Xyz.AddProcedure(c,nil,6,2)
    xyz::add_procedure(d, c, None, Some(6), 2, None, None, None, None, false, None);
    // While this card has material, your opponent cannot target it with monster effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(function(e) return e:GetHandler():GetOverlayCount()>0 end)
    effect::set_condition(d, e1, Some(condition(overlay_cond)));
    // e1:SetValue(function(e,re,rp) return rp==1-e:GetHandlerPlayer() and re:IsMonsterEffect() end)
    effect::set_value_fn(d, e1, lua_fn(value1));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate the effects of 1 face-up card your opponent controls until the end of this turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e2, CATEGORY_DISABLE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp end)
    effect::set_condition(d, e2, Some(condition(opponent_cond)));
    // e2:SetCost(Cost.DetachFromSelf(s.discostmin,2))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::F(std::sync::Arc::new(discostmin)), Some(cost::Count::N(2)), None)));
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e) return e:GetHandler():GetOverlayCount()>0 end`
fn overlay_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_overlay_count(d, h) > 0
}
/// `function(e,re,rp) return rp==1-e:GetHandlerPlayer() and re:IsMonsterEffect() end`
fn value1(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let re = a.e(1);
    let rp = a.p(2);
    rp == 1 - effect::get_handler_player(d, e) && effect::is_monster_effect(d, re)
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp end`
fn opponent_cond(_d: &mut Duel, a: &EffArgs) -> bool {
    a.rp == 1 - a.tp
}
/// `s.discostmin(e,tp,eg,ep,ev,re,r,rp,chk)`
fn discostmin(d: &mut Duel, e: EffectId, _tp: u8) -> i64 {
    // local g=e:GetHandler():GetOverlayGroup()
    let h = effect::get_handler(d, e).nil();
    let g = card::get_overlay_group(d, h);
    // return #g==g:FilterCount(Card.IsRace,nil,RACE_REPTILE|RACE_DINOSAUR) and 1 or 2
    let cnt = group::filter_count(d, &g, &|d, c| card::is_race(d, c, RACE_REPTILE | RACE_DINOSAUR), NO_EX);
    if g.len() == cnt { 1 } else { 2 }
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsOnField() and chkc:IsNegatable() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_on_field(d, chkc) && card::is_negatable(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsNegatable,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_negatable(d, c)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_NEGATE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsNegatable,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_negatable(d, c)), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, g, 1, a.tp, 0);
    true
}
/// `s.disop(e,tp,...)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // tc:NegateEffects(e:GetHandler(),RESET_PHASE|PHASE_END,true)
        let h = effect::get_handler(d, a.e).nil();
        card::negate_effects(d, tc, h, Some(RESET_PHASE | PHASE_END), true, None);
    }
}
