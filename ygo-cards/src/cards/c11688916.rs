//! Solfachord Primoa
use super::prelude::*;

pub const ID: u32 = 11688916;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_SOLFACHORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Return 1 "Solfachord" card in your Pendulum Zone to the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCondition(function(e,tp,eg) return eg:IsExists(s.pendthconfilter,1,nil,tp) end)
    effect::set_condition(d, e1, Some(condition(|d, a| group::is_exists(d, &a.eg.clone().nil(), &|d, c| pendthconfilter(d, c, a.tp), 1, NO_EX))));
    // e1:SetTarget(s.pendthtg)
    effect::set_target(d, e1, Some(cost_target(pendthtg)));
    // e1:SetOperation(s.pendthop)
    effect::set_operation(d, e1, Some(operation(pendthop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // While you have "Solfachord" cards with even and odd Pendulum Scales in your Pendulum Zone, your activated "Solfachord" cards' effects cannot be negated
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_DISEFFECT)
    effect::set_code(d, e2, EFFECT_CANNOT_DISEFFECT);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.immcon)
    effect::set_condition(d, e2, Some(lua_fn(|d, a: Args| immcon(d, a.e(0)))));
    // e2:SetValue(s.immval)
    effect::set_value_fn(d, e2, lua_fn(|d, a: Args| immval(d, a)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Add 1 "Solfachord" card from your Deck to your hand, except "Solfachord Primoria"
    // local e3a=Effect.CreateEffect(c)
    let e3a = effect::create_effect(d, c);
    // e3a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3a, aux::stringid(ID, 1));
    // e3a:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND)
    effect::set_category(d, e3a, CATEGORY_SEARCH + CATEGORY_TOHAND);
    // e3a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3a:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3a, EFFECT_FLAG_DELAY, 0);
    // e3a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3a, EVENT_SUMMON_SUCCESS);
    // e3a:SetCountLimit(1,id)
    effect::set_count_limit(d, e3a, 1, ID, 0, 0);
    // e3a:SetTarget(s.deckthtg)
    effect::set_target(d, e3a, Some(cost_target(deckthtg)));
    // e3a:SetOperation(s.deckthop)
    effect::set_operation(d, e3a, Some(operation(deckthop)));
    // c:RegisterEffect(e3a)
    card::register_effect(d, c, e3a, false);
    // local e3b=e3a:Clone()
    let e3b = effect::clone(d, e3a);
    // e3b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3b)
    card::register_effect(d, c, e3b, false);
    // Add 1 "Solfachord" card from your GY or face-up Extra Deck to your hand
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e4, CATEGORY_TOHAND);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    // e4:SetCondition(function(e,tp,eg) return eg:IsExists(s.gyedthconfilter,1,nil,tp) end)
    effect::set_condition(d, e4, Some(condition(|d, a| group::is_exists(d, &a.eg.clone().nil(), &|d, c| gyedthconfilter(d, c, a.tp), 1, NO_EX))));
    // e4:SetTarget(s.gyedthtg)
    effect::set_target(d, e4, Some(cost_target(gyedthtg)));
    // e4:SetOperation(s.gyedthop)
    effect::set_operation(d, e4, Some(operation(gyedthop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.pendthconfilter(c,tp)`
fn pendthconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSummonPlayer(tp) and c:IsPendulumSummoned()
    card::is_summon_player(d, c, tp) && card::is_pendulum_summoned(d, c)
}
/// `s.pendthfilter(c)`
fn pendthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SOLFACHORD) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_SOLFACHORD as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.pendthtg(e,tp,...,chk,chkc)`
fn pendthtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_PZONE) and s.pendthfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_PZONE) && pendthfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.pendthfilter,tp,LOCATION_PZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&pendthfilter), tp, LOCATION_PZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.pendthfilter,tp,LOCATION_PZONE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&pendthfilter), tp, LOCATION_PZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, tp, 0);
    true
}
/// `s.pendthop(e,tp,...)`
fn pendthop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
/// `s.immcon(e)`
fn immcon(d: &mut Duel, e: EffectId) -> bool {
    // local g=Duel.GetMatchingGroup(Card.IsSetCard,e:GetHandlerPlayer(),LOCATION_PZONE,0,nil,SET_SOLFACHORD)
    let p = effect::get_handler_player(d, e);
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_set_card(d, c, &[SET_SOLFACHORD as u16])), p, LOCATION_PZONE, 0, NO_EX);
    // local a,b=g:GetFirst(),g:GetNext()
    let order = group::iter(d, &g);
    // return a and b and ((a:GetScale()+b:GetScale())%2==1)
    match (order.first().copied(), order.get(1).copied()) {
        (Some(a), Some(b)) => (card::get_scale(d, a) + card::get_scale(d, b)) % 2 == 1,
        _ => false,
    }
}
/// `s.immval(e,ct)`
fn immval(d: &mut Duel, a: Args) -> Value {
    let e = a.e(0);
    let ct = a.int(1) as u8;
    // return Chain.IsTriggeringPlayer(ct,e:GetHandlerPlayer()) and Chain.IsSetcode(ct,SET_SOLFACHORD)
    let p = effect::get_handler_player(d, e);
    let first = chain::is_triggering_player(d, ct, p);
    if !first.truthy() {
        return first;
    }
    chain::is_triggering(d, ct, chain::Prop::SetCard, &[SET_SOLFACHORD as i64])
}
/// `s.deckthfilter(c)`
fn deckthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SOLFACHORD) and not c:IsCode(id) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_SOLFACHORD as u16]) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
/// `s.deckthtg(e,tp,...,chk)`
fn deckthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.deckthfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&deckthfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.deckthop(e,tp,...)`
fn deckthop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.deckthfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&deckthfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
/// `s.gyedthconfilter(c,tp)`
fn gyedthconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsLinkSummoned() and c:IsSetCard(SET_SOLFACHORD) and c:IsSummonPlayer(tp)
    card::is_link_summoned(d, c) && card::is_set_card(d, c, &[SET_SOLFACHORD as u16]) && card::is_summon_player(d, c, tp)
}
/// `s.gyedthfilter(c)`
fn gyedthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SOLFACHORD) and c:IsFaceup() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_SOLFACHORD as u16]) && card::is_faceup(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.gyedthtg(e,tp,...,chk)`
fn gyedthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.gyedthfilter,tp,LOCATION_GRAVE|LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&gyedthfilter), a.tp, LOCATION_GRAVE | LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE|LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_GRAVE | LOCATION_EXTRA) as i32);
    true
}
/// `s.gyedthop(e,tp,...)`
fn gyedthop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.gyedthfilter),tp,LOCATION_GRAVE|LOCATION_EXTRA,0,1,1,nil)
    let nv = aux::necro_valley_filter(|d, c| gyedthfilter(d, c));
    let g = duel::select_matching_card(d, a.tp, Some(&nv), a.tp, LOCATION_GRAVE | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
