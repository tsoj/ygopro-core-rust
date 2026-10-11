//! Dewloren, Tiger King of the Ice Barrier
use super::prelude::*;

pub const ID: u32 = 70583986;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Synchro summon procedure
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTunerEx(Card.IsAttribute,ATTRIBUTE_WATER),1,99)
    synchro::add_procedure(
        d,
        c,
        None,
        1,
        1,
        Some(synchro::non_tuner_ex(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_WATER, sc, st, Some(tp)))),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Return cards you control to hand, gain 500 ATK/returned card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAbleToHand()
    card::is_faceup(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsControler(tp) and s.filter(chkc) and chkc~=e:GetHandler() end
    if let Some(chkc) = a.chkc() {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, tp) && filter(d, chkc) && chkc != h;
    }
    // local h=e:GetHandler()
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_ONFIELD,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_ONFIELD, 0, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_ONFIELD,0,1,12,e:GetHandler())
    let g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_ONFIELD, 0, 1, 12, false, h.into());
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,0,0)
    let n = g.as_ref().map_or(0, group::get_count) as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, n, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tg=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let tg = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // local rg=tg:Filter(Card.IsRelateToEffect,nil,e)
    let rg = group::filter(d, &tg, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    // Duel.SendtoHand(rg,nil,REASON_EFFECT)
    duel::send_to_hand(d, rg, None, REASON_EFFECT, None);
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local og=Duel.GetOperatedGroup()
        let og = duel::get_operated_group(d);
        // local ct=og:FilterCount(Card.IsLocation,nil,LOCATION_HAND)
        let ct = group::filter_count(d, &og, &|d, c| card::is_location(d, c, LOCATION_HAND), NO_EX);
        // if ct>0 then
        if ct > 0 {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
            // e1:SetValue(ct*500)
            effect::set_value(d, e1, ct as i64 * 500);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
    }
}
