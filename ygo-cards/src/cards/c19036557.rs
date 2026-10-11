//! Elementsaber Makani
//! エレメントセイバー・マカニ
use super::prelude::*;

pub const ID: u32 = 19036557;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ELEMENTSABER, SET_ELEMENTAL_LORD],
    listed_names: &[],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // search
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(s.thcost)
    effect::set_cost(d, e1, Some(cost_target(thcost)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // att change
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.atttg)
    effect::set_target(d, e2, Some(cost_target(atttg)));
    // e2:SetOperation(s.attop)
    effect::set_operation(d, e2, Some(operation(attop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.costfilter(c,tp)`
fn costfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ELEMENTSABER) and c:IsMonster() and c:IsAbleToGraveAsCost()
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,c)
    card::is_set_card(d, c, &[SET_ELEMENTSABER as u16])
        && card::is_monster(d, c)
        && card::is_able_to_grave_as_cost(d, c)
        && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, c.into())
}
/// `s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local fg=Group.CreateGroup()
    let mut fg = CardSet::new();
    // for i,pe in ipairs({Duel.IsPlayerAffectedByEffect(tp,61557074)}) do fg:AddCard(pe:GetHandler()) end
    if let Some(pe) = duel::is_player_affected_by_effect(d, tp, 61557074) {
        let h = effect::get_handler(d, pe).nil();
        group::add_card(&mut fg, &CardSet::from_card(h));
    }
    // local loc=LOCATION_HAND
    let mut loc = LOCATION_HAND;
    // if #fg>0 then loc=LOCATION_HAND|LOCATION_DECK end
    if !fg.is_empty() {
        loc = LOCATION_HAND | LOCATION_DECK;
    }
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,loc,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| costfilter(d, c, tp)), tp, loc, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.costfilter,tp,loc,0,1,1,nil,tp):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| costfilter(d, c, tp)), tp, loc, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel).nil();
    // if tc:IsLocation(LOCATION_DECK) then
    if card::is_location(d, tc, LOCATION_DECK) {
        // local fc=nil
        // if #fg==1 then fc=fg:GetFirst() else fc=fg:Select(tp,1,1,nil) end
        // (fg holds at most one card: IsPlayerAffectedByEffect returns one effect, so the
        // Select branch of the Lua never runs)
        let fc = group::get_first(d, &fg).nil();
        // Duel.Hint(HINT_CARD,0,fc:GetCode())
        let code = card::get_code(d, fc)[0];
        duel::hint(d, HINT_CARD as u8, 0, code as u64);
        // fc:RegisterFlagEffect(61557074,RESETS_STANDARD_PHASE_END,0,0)
        card::register_flag_effect(d, fc, 61557074, RESETS_STANDARD_PHASE_END, 0, 0, 0, 0);
    }
    // Duel.SendtoGrave(tc,REASON_COST)
    duel::send_to_grave(d, tc, REASON_COST, None, None);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and not c:IsCode(id) and (c:IsSetCard(SET_ELEMENTSABER) or c:IsSetCard(SET_ELEMENTAL_LORD)) and c:IsAbleToHand()
    card::is_monster(d, c)
        && !card::is_code(d, c, &[ID])
        && (card::is_set_card(d, c, &[SET_ELEMENTSABER as u16]) || card::is_set_card(d, c, &[SET_ELEMENTAL_LORD as u16]))
        && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
/// `s.atttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atttg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local att=c:AnnounceAnotherAttribute(tp)
    let att = card::announce_another_attribute(d, c, tp);
    // e:SetLabel(att)
    effect::set_label(d, e, &[att as i64]);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,c,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.attop(e,tp,eg,ep,ev,re,r,rp)`
fn attop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CHANGE_ATTRIBUTE)
    effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
    // e1:SetValue(e:GetLabel())
    let label = effect::get_label(d, e);
    effect::set_value(d, e1, label);
    // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
