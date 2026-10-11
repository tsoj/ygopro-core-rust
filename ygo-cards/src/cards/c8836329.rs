//! Raging Storm Dragon - Beaufort IX
use super::prelude::*;

pub const ID: u32 = 8836329;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[44221928], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Negate attack and effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.nacon)
    effect::set_condition(d, e1, Some(condition(nacon)));
    // e1:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e1, Some(cost::self_discard()));
    // e1:SetTarget(s.natg)
    effect::set_target(d, e1, Some(cost_target(natg)));
    // e1:SetOperation(s.naop)
    effect::set_operation(d, e1, Some(operation(naop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return both to hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.nacon(e,tp,eg,ep,ev,re,r,rp)`
fn nacon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttacker():IsControler(1-tp) and Duel.GetAttackTarget()==nil
    let at = duel::get_attacker(d).nil();
    card::is_controler(d, at, 1 - a.tp) && duel::get_attack_target(d).is_none()
}
/// `s.natg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn natg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetAttacker():IsOnField() end
    if a.chk == Some(0) {
        let at = duel::get_attacker(d).nil();
        return card::is_on_field(d, at);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,Duel.GetAttacker(),1,0,0)
    let at = duel::get_attacker(d);
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, at.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.naop(e,tp,eg,ep,ev,re,r,rp)`
fn naop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local a=Duel.GetAttacker()
    let at = duel::get_attacker(d);
    // if Duel.NegateAttack() and a and a:IsFaceup() and not a:IsDisabled() then
    if duel::negate_attack(d) {
        if let Some(at) = at {
            if card::is_faceup(d, at) && !card::is_disabled(d, at) {
                // Duel.NegateRelatedChain(a,RESET_TURN_SET)
                duel::negate_related_chain(d, at, RESET_TURN_SET);
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
                // e1:SetCode(EFFECT_DISABLE)
                effect::set_code(d, e1, EFFECT_DISABLE);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // a:RegisterEffect(e1)
                card::register_effect(d, at, e1, false);
                // local e2=Effect.CreateEffect(c)
                let e2 = effect::create_effect(d, c);
                // e2:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
                // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
                // e2:SetCode(EFFECT_DISABLE_EFFECT)
                effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
                // e2:SetValue(RESET_TURN_SET)
                effect::set_value(d, e2, RESET_TURN_SET as i64);
                // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
                // a:RegisterEffect(e2)
                card::register_effect(d, at, e2, false);
            }
        }
    }
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsFaceup() and chkc:IsAbleToHand() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, 1 - a.tp)
            && card::is_faceup(d, chkc)
            && card::is_able_to_hand(d, chkc, None);
    }
    // if chk==0 then return c:IsAbleToHand() and Duel.IsExistingTarget(aux.FaceupFilter(Card.IsAbleToHand),tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None)
            && duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c) && card::is_able_to_hand(d, c, None)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsAbleToHand),tp,0,LOCATION_MZONE,1,1,nil)
    let mut g = duel::select_target(
        d,
        a.tp,
        Some(&|d, c| card::is_faceup(d, c) && card::is_able_to_hand(d, c, None)),
        a.tp,
        0,
        LOCATION_MZONE,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // g:AddCard(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 2, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) && card::is_relate_to_effect(d, tc.nil(), a.e) {
        // local rg=Group.FromCards(c,tc)
        let mut rg = CardSet::from_card(c);
        group::add_card(&mut rg, &CardSet::from_card(tc.nil()));
        // Duel.SendtoHand(rg,nil,REASON_EFFECT)
        duel::send_to_hand(d, rg, None, REASON_EFFECT, None);
    }
}
