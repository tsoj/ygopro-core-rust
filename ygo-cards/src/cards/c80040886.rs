//! Navy Dragon Mech
use super::prelude::*;

pub const ID: u32 = 80040886;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Synchro Summon
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Negate cards up to the number of tuners
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetTarget(s.distg)
    effect::set_target(d, e1, Some(cost_target(distg)));
    // e1:SetOperation(s.disop)
    effect::set_operation(d, e1, Some(operation(disop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 tuner monster to the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsOnField() and chkc:IsNegatable() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_on_field(d, chkc) && card::is_negatable(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsNegatable,tp,0,LOCATION_ONFIELD,1,nil)
    //     and Duel.IsExistingMatchingCard(Card.IsType,tp,LOCATION_GRAVE,0,1,nil,TYPE_TUNER) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_negatable(d, c)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_type(d, c, TYPE_TUNER)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // local ct=Duel.GetMatchingGroupCount(Card.IsType,tp,LOCATION_GRAVE,0,nil,TYPE_TUNER)
    let ct = duel::get_matching_group_count(d, Some(&|d, c| card::is_type(d, c, TYPE_TUNER)), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_NEGATE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsNegatable,tp,0,LOCATION_ONFIELD,1,ct,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_negatable(d, c)), a.tp, 0, LOCATION_ONFIELD, 1, ct as u16, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(g), n, 0, 0);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local dg=Duel.GetTargetCards(e)
    // if #dg==0 then return end
    let dg = duel::get_target_cards(d, a.e).unwrap_or_default();
    if dg.is_empty() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // for tc in dg:Iter() do
    for tc in group::iter(d, &dg) {
        // if (tc:IsFaceup() and not tc:IsDisabled()) or tc:IsType(TYPE_TRAPMONSTER) then
        if (card::is_faceup(d, tc) && !card::is_disabled(d, tc)) || card::is_type(d, tc, TYPE_TRAPMONSTER) {
            // Duel.NegateRelatedChain(tc,RESET_TURN_SET)
            duel::negate_related_chain(d, tc, RESET_TURN_SET);
            // Negate its effects
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e1, EFFECT_DISABLE);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
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
            // e2:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
            // if tc:IsType(TYPE_TRAPMONSTER) then
            if card::is_type(d, tc, TYPE_TRAPMONSTER) {
                // local e3=Effect.CreateEffect(c)
                let e3 = effect::create_effect(d, c);
                // e3:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
                // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
                // e3:SetCode(EFFECT_DISABLE_TRAPMONSTER)
                effect::set_code(d, e3, EFFECT_DISABLE_TRAPMONSTER);
                // e3:SetReset(RESETS_STANDARD_PHASE_END)
                effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
                // tc:RegisterEffect(e3)
                card::register_effect(d, tc, e3, false);
            }
        }
    }
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsReason(REASON_DESTROY) and c:IsReason(REASON_EFFECT)
    //     and c:IsPreviousLocation(LOCATION_MZONE) and c:IsSynchroSummoned()
    card::is_reason(d, c, REASON_DESTROY)
        && card::is_reason(d, c, REASON_EFFECT)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_synchro_summoned(d, c)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_TUNER) and c:IsAbleToHand()
    card::is_type(d, c, TYPE_TUNER) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && thfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::get_first_target(d).first().copied().nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, CardSet::from_card(tc), None, REASON_EFFECT, None);
    }
}
