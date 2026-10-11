//! Advanced Crystal Beast Cobalt Eagle
//! Scripted by Naim
use super::prelude::*;

pub const ID: u32 = 45236142;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ADVANCED_DARK], listed_series: &[SET_ADVANCED_CRYSTAL_BEAST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Duel.EnableGlobalFlag(GLOBALFLAG_SELF_TOGRAVE)
    duel::enable_global_flag(d, GLOBALFLAG_SELF_TOGRAVE);
    // Send itself to the GY if "Adanced Dark" is not face-up in the Field Spell Zone
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_SELF_TOGRAVE)
    effect::set_code(d, e1, EFFECT_SELF_TOGRAVE);
    // e1:SetCondition(s.tgcon)
    effect::set_condition(d, e1, Some(condition(tgcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Place itself in the S/T instead of sending it to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_TO_GRAVE_REDIRECT_CB)
    effect::set_code(d, e2, EFFECT_TO_GRAVE_REDIRECT_CB);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetCondition(s.replacecon)
    effect::set_condition(d, e2, Some(condition(replacecon)));
    // e2:SetOperation(s.replaceop)
    effect::set_operation(d, e2, Some(operation(replaceop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Add 1 "Advanced Dark" to the hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_HAND|LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_HAND | LOCATION_MZONE);
    // e3:SetCost(Cost.SelfToGrave)
    effect::set_cost(d, e3, Some(cost::self_to_grave()));
    // e3:SetTarget(s.searchtg)
    effect::set_target(d, e3, Some(cost_target(searchtg)));
    // e3:SetOperation(s.searchop)
    effect::set_operation(d, e3, Some(operation(searchop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Target 1 "Advanced Crystal Beast" card and add it to hand or to the deck
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_TOHAND+CATEGORY_TODECK)
    effect::set_category(d, e4, CATEGORY_TOHAND + CATEGORY_TODECK);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetTarget(s.thdtg)
    effect::set_target(d, e4, Some(cost_target(thdtg)));
    // e4:SetOperation(s.thdop)
    effect::set_operation(d, e4, Some(operation(thdop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.tgcon(e)`
fn tgcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return not Duel.IsEnvironment(CARD_ADVANCED_DARK)
    !duel::is_environment(d, CARD_ADVANCED_DARK, None, None)
}
/// `s.replacecon(e)`
fn replacecon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsFaceup() and c:IsLocation(LOCATION_MZONE) and c:IsReason(REASON_DESTROY)
    card::is_faceup(d, c) && card::is_location(d, c, LOCATION_MZONE) && card::is_reason(d, c, REASON_DESTROY)
}
/// `s.replaceop(e,tp,eg,ep,ev,re,r,rp)`
fn replaceop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCode(EFFECT_CHANGE_TYPE)
    effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD&~RESET_TURN_SET)
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
    // e1:SetValue(TYPE_SPELL+TYPE_CONTINUOUS)
    effect::set_value(d, e1, (TYPE_SPELL + TYPE_CONTINUOUS) as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Duel.RaiseEvent(c,EVENT_CUSTOM+CARD_CRYSTAL_TREE,e,0,tp,0,0)
    duel::raise_event(d, CardSet::from_card(c), EVENT_CUSTOM + CARD_CRYSTAL_TREE, Some(a.e), 0, a.tp, 0, 0);
}
/// `s.sfilter(c)`
fn sfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_ADVANCED_DARK) and c:IsAbleToHand()
    card::is_code(d, c, &[CARD_ADVANCED_DARK]) && card::is_able_to_hand(d, c, None)
}
/// `s.searchtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn searchtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.sfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&sfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.searchop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn searchop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.sfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&sfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, &g);
    }
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_ADVANCED_CRYSTAL_BEAST) and (c:IsAbleToHand() or c:IsAbleToDeck())
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ADVANCED_CRYSTAL_BEAST as u16]) && (card::is_able_to_hand(d, c, None) || card::is_able_to_deck(d, c))
}
/// `s.thdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsControler(tp) and s.cfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, a.tp) && cfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.cfilter,tp,LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&cfilter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.cfilter,tp,LOCATION_ONFIELD,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&cfilter), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,g,1,tp,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, Some(g.clone()), 1, a.tp, LOCATION_ONFIELD as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,g,1,tp,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, Some(g), 1, a.tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.thdop(e,tp,eg,ep,ev,re,r,rp)`
fn thdop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // aux.ToHandOrElse(tc,tp, function(c) return c:IsAbleToDeck() end, function(tc) Duel.SendtoDeck(tc,nil,SEQ_DECKTOP,REASON_EFFECT) end, aux.Stringid(id,2))
        aux::to_hand_or_else(
            d,
            tc,
            a.tp,
            Some(&|d, c| card::is_able_to_deck(d, c)),
            Some(&|d, g| duel::send_to_deck(d, g, None, SEQ_DECKTOP as i32, REASON_EFFECT, None)),
            Some(aux::stringid(ID, 2)),
        );
    }
}
