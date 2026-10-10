//! Twilight Twin Dragons
use super::prelude::*;

pub const ID: u32 = 60431417;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LIGHTSWORN as u32], listed_names: &[19959563, 57774843], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_DECKDES)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_DECKDES);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // to hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCode(19959563)
    card::is_faceup(d, c) && card::is_code(d, c, &[19959563])
}
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&|d, x| cfilter(d, x)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(57774843) and c:IsAbleToHand()
    card::is_code(d, c, &[57774843]) && card::is_able_to_hand(d, c, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil)
    //   and Duel.IsPlayerCanDiscardDeck(tp,4) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| filter(d, x)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_player_can_discard_deck(d, tp, 4);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, x| filter(d, x)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,4)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, tp, 4);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc:IsRelateToEffect(e) and Duel.SendtoHand(tc,nil,REASON_EFFECT)>0 then
    if card::is_relate_to_effect(d, tc.nil(), e) && duel::send_to_hand(d, tc.nil(), None, REASON_EFFECT, None) > 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.DiscardDeck(tp,4,REASON_EFFECT)
        duel::discard_deck(d, tp, 4, REASON_EFFECT);
    }
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousLocation(LOCATION_DECK) and re:IsMonsterEffect()
    //   and re:GetHandler():IsSetCard(SET_LIGHTSWORN) and (r&REASON_EFFECT)~=0
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_DECK)
        && effect::is_monster_effect(d, a.re.nil())
        && card::is_set_card(d, effect::get_handler(d, a.re.nil()).nil(), &[SET_LIGHTSWORN as u16])
        && (a.r & REASON_EFFECT) != 0
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(19959563) and c:IsAbleToHand()
    card::is_code(d, c, &[19959563]) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && thfilter(d, chkc);
    }
    // local rg=Duel.GetDecktopGroup(tp,4)
    let rg = duel::get_decktop_group(d, tp, 4);
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,LOCATION_GRAVE,0,1,nil)
    //   and rg:FilterCount(Card.IsAbleToRemove,nil)==4 end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| thfilter(d, x)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && group::filter_count(d, &rg, &|d, x| card::is_able_to_remove(d, x, None, None, None), NO_EX) == 4;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, x| thfilter(d, x)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,rg,4,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(rg), 4, 0, 0);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc:IsRelateToEffect(e) and Duel.SendtoHand(tc,nil,REASON_EFFECT)>0 then
    if card::is_relate_to_effect(d, tc.nil(), e) && duel::send_to_hand(d, tc.nil(), None, REASON_EFFECT, None) > 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local rg=Duel.GetDecktopGroup(tp,4)
        let rg = duel::get_decktop_group(d, tp, 4);
        // Duel.Remove(rg,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, rg, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
