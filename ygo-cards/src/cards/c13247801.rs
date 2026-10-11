//! Machina Defense Perimeter
use super::prelude::*;

pub const ID: u32 = 13247801;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // cannot target level <=6
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(s.cannottgeff)
    effect::set_target(d, e2, Some(effect_card_fn(cannottgeff)));
    // e2:SetCondition(s.cannottgcon)
    effect::set_condition(d, e2, Some(condition(cannottgcon)));
    // e2:SetValue(aux.tgoval)
    effect::set_value_fn(d, e2, aux::tgoval());
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // cannot attack level <=6
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e3, 0, LOCATION_MZONE);
    // e3:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e3, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e3:SetCondition(s.cannottgcon)
    effect::set_condition(d, e3, Some(condition(cannottgcon)));
    // e3:SetValue(s.cannotatk)
    effect::set_value_fn(d, e3, effect_card_fn(cannotatk));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Add to Hand
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e4, CATEGORY_TOHAND);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e4:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e4, EVENT_DESTROYED);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetCondition(s.tohandcon)
    effect::set_condition(d, e4, Some(condition(tohandcon)));
    // e4:SetTarget(s.tohandtg)
    effect::set_target(d, e4, Some(cost_target(tohandtg)));
    // e4:SetOperation(s.tohandop)
    effect::set_operation(d, e4, Some(operation(tohandop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_MACHINE) and c:IsLevelAbove(7)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_MACHINE) && card::is_level_above(d, c, 7)
}
/// `s.cannottgcon(e)`
fn cannottgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,e:GetHandlerPlayer(),LOCATION_MZONE,0,1,e:GetHandler())
    let p = effect::get_handler_player(d, a.e);
    let h = effect::get_handler(d, a.e).nil();
    duel::is_existing_matching_card(d, Some(&cfilter), p, LOCATION_MZONE, 0, 1, h.into())
}
/// `s.cannottgeff(e,c)`
fn cannottgeff(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsLevelBelow(6) and c:IsRace(RACE_MACHINE)
    card::is_level_below(d, c, 6) && card::is_race(d, c, RACE_MACHINE)
}
/// `s.cannotatk(e,c)`
fn cannotatk(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsLevelBelow(6) and c:IsRace(RACE_MACHINE)
    card::is_faceup(d, c) && card::is_level_below(d, c, 6) && card::is_race(d, c, RACE_MACHINE)
}
/// `s.tohandconfilter(c,tp)`
fn tohandconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:GetPreviousControler()==tp and c:IsPreviousLocation(LOCATION_MZONE) and c:IsReason(REASON_DESTROY)
    //   and (c:IsReason(REASON_BATTLE) or c:IsReason(REASON_EFFECT))
    //   and c:IsRace(RACE_MACHINE) and c:IsPreviousPosition(POS_FACEUP)
    card::get_previous_controler(d, c) == tp
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_reason(d, c, REASON_DESTROY)
        && (card::is_reason(d, c, REASON_BATTLE) || card::is_reason(d, c, REASON_EFFECT))
        && card::is_race(d, c, RACE_MACHINE)
        && card::is_previous_position(d, c, POS_FACEUP)
}
/// `s.tohandcon(e,tp,eg,ep,ev,re,r,rp)`
fn tohandcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.tohandconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| tohandconfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.tohandfilter(c)`
fn tohandfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToHand() and c:IsRace(RACE_MACHINE)
    card::is_able_to_hand(d, c, None) && card::is_race(d, c, RACE_MACHINE)
}
/// `s.tohandtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tohandtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.tohandfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && tohandfilter(d, chkc);
    }
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,e:GetHandler()) end
    // (`s.filter` is not defined in this script: nil, so the filter is absent.)
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, LOCATION_GRAVE, 0, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.tohandfilter,tp,LOCATION_GRAVE,0,1,1,e:GetHandler())
    let g = duel::select_target(d, tp, Some(&tohandfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, h.into()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,0,0)
    let n = group::get_count(&g) as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, 0, 0);
    true
}
/// `s.tohandop(e,tp,eg,ep,ev,re,r,rp)`
fn tohandop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SendtoHand(tc,nil,REASON_EFFECT)
            duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        }
    }
}
