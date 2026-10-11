//! Fluffal Dolphin
//! ファーニマル・ドルフィン
use super::prelude::*;

pub const ID: u32 = 82896870;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[30068120, 70245411],
    listed_series: &[SET_FLUFFAL, SET_FUSION],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Set 1 "Toy Vendor" and send "Fluffal" to the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.settg)
    effect::set_target(d, e1, Some(cost_target(settg)));
    // e1:SetOperation(s.setop)
    effect::set_operation(d, e1, Some(operation(setop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Send to Deck 1 "Fusion" Spell
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.tdcon)
    effect::set_condition(d, e2, Some(condition(tdcon)));
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(70245411) and c:IsSSetable()
    card::is_code(d, c, &[70245411]) && card::is_s_setable(d, c, false, None)
}
/// `s.gyfilter(c)`
fn gyfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and (c:IsCode(30068120) or c:IsSetCard(SET_FLUFFAL)) and c:IsAbleToGrave()
    card::is_monster(d, c) && (card::is_code(d, c, &[30068120]) || card::is_set_card(d, c, &[SET_FLUFFAL as u16])) && card::is_able_to_grave(d, c, None)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.setfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && setfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.setfilter,tp,LOCATION_GRAVE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(s.gyfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&setfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&gyfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local g=Duel.SelectTarget(tp,s.setfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&setfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, g, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and tc:IsSSetable() and Duel.SSet(tp,tc)~=0 then
    let Some(tc) = tc else {
        return;
    };
    if card::is_relate_to_effect(d, tc, a.e) && card::is_s_setable(d, tc, false, None) && duel::s_set(d, tp, tc, None, true) != 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.gyfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&gyfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then Duel.SendtoGrave(g,REASON_EFFECT) end
        if g.len() > 0 {
            duel::send_to_grave(d, g, REASON_EFFECT, None, None);
        }
    }
}
/// `s.tdcon(e,tp,eg,ep,ev,re,r,rp)`
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLocation(LOCATION_GRAVE) and r==REASON_FUSION
    let h = effect::get_handler(d, a.e).nil();
    card::is_location(d, h, LOCATION_GRAVE) && a.r == REASON_FUSION
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_FUSION) and c:IsSpell() and (c:IsFaceup() or not c:IsLocation(LOCATION_REMOVED)) and c:IsAbleToDeck()
    card::is_set_card(d, c, &[SET_FUSION as u16])
        && card::is_spell(d, c)
        && (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_REMOVED))
        && card::is_able_to_deck(d, c)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tdfilter,tp,LOCATION_REMOVED|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tdfilter), tp, LOCATION_REMOVED + LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_REMOVED|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, (LOCATION_REMOVED + LOCATION_GRAVE) as i32);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tdfilter,tp,LOCATION_REMOVED|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&tdfilter), tp, LOCATION_REMOVED + LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if g.len() > 0 {
        // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
