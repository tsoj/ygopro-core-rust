//! Elemental HERO Honest Neos
use super::prelude::*;

pub const ID: u32 = 14124483;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_HERO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Make 1 "HERO" monster on the field gain 2500 ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function() return not (Duel.IsPhase(PHASE_DAMAGE) and Duel.IsDamageCalculated()) end)
    effect::set_condition(d, e1, Some(condition(not_damage_calc)));
    // e1:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e1, Some(cost::self_discard()));
    // e1:SetTarget(s.heroatktg)
    effect::set_target(d, e1, Some(cost_target(heroatktg)));
    // e1:SetOperation(s.heroatkop)
    effect::set_operation(d, e1, Some(operation(heroatkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Make this card gain ATK equal to the ATK of a "HERO" monster you discard
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e2, TIMING_DAMAGE_STEP, None);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function() return not (Duel.IsPhase(PHASE_DAMAGE) and Duel.IsDamageCalculated()) end)
    effect::set_condition(d, e2, Some(condition(not_damage_calc)));
    // e2:SetTarget(s.selfatkcost)
    effect::set_target(d, e2, Some(cost_target(selfatkcost)));
    // e2:SetOperation(s.selfatkop)
    effect::set_operation(d, e2, Some(operation(selfatkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function() return not (Duel.IsPhase(PHASE_DAMAGE) and Duel.IsDamageCalculated()) end`
fn not_damage_calc(d: &mut Duel, _a: &EffArgs) -> bool {
    !(duel::is_phase(d, PHASE_DAMAGE) && duel::is_damage_calculated(d))
}
/// `function(c) return c:IsFaceup() and c:IsSetCard(SET_HERO) end` (`aux.FaceupFilter(Card.IsSetCard,SET_HERO)`)
fn hero_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_HERO as u16])
}
/// `s.heroatktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn heroatktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsSetCard(SET_HERO) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_set_card(d, chkc, &[SET_HERO as u16]) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsSetCard,SET_HERO),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&hero_faceup), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATKDEF as u64);
    // Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsSetCard,SET_HERO),tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, a.tp, Some(&hero_faceup), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.heroatkop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn heroatkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(2500)
        effect::set_value(d, e1, 2500);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.selfatkcostfilter(c)`
fn selfatkcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_HERO) and c:GetAttack()>0 and c:IsDiscardable()
    card::is_set_card(d, c, &[SET_HERO as u16]) && card::get_attack(d, c) > 0 && card::is_discardable(d, c, None, None)
}
/// `s.selfatkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfatkcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.selfatkcostfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&selfatkcostfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DISCARD as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.selfatkcostfilter,tp,LOCATION_HAND,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&selfatkcostfilter), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel).nil();
    // e:SetLabel(sc:GetAttack())
    let atk = card::get_attack(d, sc);
    effect::set_label(d, a.e, &[atk as i64]);
    // Duel.SendtoGrave(sc,REASON_COST|REASON_DISCARD)
    duel::send_to_grave(d, CardSet::from_card(sc), REASON_COST | REASON_DISCARD, None, None);
    true
}
/// `s.selfatkop(e,tp,eg,ep,ev,re,r,rp)`
fn selfatkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(e:GetLabel())
        let label = effect::get_label(d, a.e);
        effect::set_value(d, e1, label);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
