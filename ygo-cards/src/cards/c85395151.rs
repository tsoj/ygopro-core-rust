//! Release Brainwashing
use super::prelude::*;

pub const ID: u32 = 85395151;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Take control of a monster, or negate its effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_CONTROL+CATEGORY_TOHAND+CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_CONTROL + CATEGORY_TOHAND + CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_RELEASE)
    effect::set_code(d, e1, EVENT_RELEASE);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.effcon)
    effect::set_condition(d, e1, Some(condition(effcon)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.effconfilter(c)
fn effconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPreviousLocation(LOCATION_MZONE|LOCATION_HAND) and c:IsMonster()
    card::is_previous_location(d, c, LOCATION_MZONE | LOCATION_HAND) && card::is_monster(d, c)
}
// function s.effcon(e,tp,eg,ep,ev,re,r,rp)
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.effconfilter,1,nil)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| effconfilter(d, c), 1, NO_EX)
}
// function s.tgfilter(c,tp)
fn tgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and ((c:IsOwner(tp) and c:IsAbleToChangeControler()) or (c:IsOwner(1-tp) and c:IsNegatableMonster()))
    card::is_faceup(d, c)
        && ((card::is_owner(d, c, tp) && card::is_able_to_change_controler(d, c))
            || (card::is_owner(d, c, 1 - tp) && card::is_negatable_monster(d, c)))
}
// function s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.tgfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && tgfilter(d, chkc, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,0,LOCATION_MZONE,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, a.tp)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local tc=Duel.SelectTarget(tp,s.tgfilter,tp,0,LOCATION_MZONE,1,1,nil,tp):GetFirst()
    let sel = duel::select_target(d, a.tp, Some(&|d, c| tgfilter(d, c, a.tp)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel).nil();
    // if tc:IsOwner(tp) then
    if card::is_owner(d, tc, a.tp) {
        // Duel.SetOperationInfo(0,CATEGORY_CONTROL,tc,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(CardSet::from_card(tc)), 1, a.tp, 0);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,tc,1,tp,0)
        duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(tc)), 1, a.tp, 0);
    } else {
        // Duel.SetOperationInfo(0,CATEGORY_DISABLE,tc,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(CardSet::from_card(tc)), 1, a.tp, 0);
    }
    true
}
// function s.delayed_return(ag) Duel.SendtoHand(ag,nil,REASON_EFFECT) end
fn delayed_return(d: &mut Duel, ag: &CardSet, _a: &EffArgs) -> bool {
    duel::send_to_hand(d, ag.clone(), None, REASON_EFFECT, None);
    true
}
// function s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and tc:IsFaceup()) then return end
    if !(card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc)) {
        return;
    }
    // if tc:IsOwner(tp) then
    if card::is_owner(d, tc, a.tp) {
        // if not Duel.GetControl(tc,tp) then return end
        // (a Lua integer is always true: the check never returns)
        duel::get_control(d, tc, a.tp, 0, 0, None, None);
        // Return it to the hand during the End Phase
        // aux.DelayedOperation(tc,PHASE_END,id,e,tp,function(ag) Duel.SendtoHand(ag,nil,REASON_EFFECT) end,nil,0,0,aux.Stringid(id,1))
        aux::delayed_operation(
            d,
            tc,
            PHASE_END,
            ID,
            a.e,
            a.tp,
            Some(std::sync::Arc::new(delayed_return)),
            None,
            Some(0),
            Some(0),
            Some(aux::stringid(ID, 1)),
            None,
        );
    } else {
        // tc:NegateEffects(e:GetHandler(),RESET_PHASE|PHASE_END)
        let h = effect::get_handler(d, a.e).nil();
        card::negate_effects(d, tc, h, Some(RESET_PHASE | PHASE_END), false, None);
    }
}
