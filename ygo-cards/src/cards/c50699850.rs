//! Loris, Lady of Lament
use super::prelude::*;

pub const ID: u32 = 50699850;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Draw
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.drtg)
    effect::set_target(d, e1, Some(cost_target(drtg)));
    // e1:SetOperation(s.drop)
    effect::set_operation(d, e1, Some(operation(drop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Set
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_LEAVE_GRAVE+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_LEAVE_GRAVE + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_RELEASE)
    effect::set_code(d, e2, EVENT_RELEASE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.rlsetcon)
    effect::set_condition(d, e2, Some(condition(rlsetcon)));
    // e2:SetTarget(s.settg)
    effect::set_target(d, e2, Some(cost_target(settg)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCondition(s.tgsetcon)
    effect::set_condition(d, e3, Some(condition(tgsetcon)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// s.drfilter(c,e)
fn drfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:IsNormalTrap() and c:IsAbleToDeck() and c:IsCanBeEffectTarget(e)
    card::is_faceup(d, c) && card::is_normal_trap(d, c) && card::is_able_to_deck(d, c) && card::is_can_be_effect_target(d, c, Some(e))
}
// s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return #sg==3 or (#sg==6 and Duel.IsPlayerCanDraw(tp,2))
    (sg.len() == 3 || (sg.len() == 6 && duel::is_player_can_draw(d, tp, 2)), false)
}
// s.drtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_REMOVED|LOCATION_GRAVE) and chkc:IsControler(tp) and s.drfilter(chkc,e) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_REMOVED | LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && drfilter(d, chkc, a.e);
    }
    // local g=Duel.GetMatchingGroup(s.drfilter,tp,LOCATION_REMOVED|LOCATION_GRAVE,0,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| drfilter(d, c, a.e)), a.tp, LOCATION_REMOVED | LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return #g>=3 and Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return g.len() >= 3 && duel::is_player_can_draw(d, a.tp, 1);
    }
    // local dg=aux.SelectUnselectGroup(g,e,tp,3,6,s.rescon,1,tp,HINTMSG_TODECK,s.rescon)
    let dg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(3), Some(6), Some(&rescon), 1, a.tp, Some(HINTMSG_TODECK as u64), Some(&rescon), None, false).unwrap_or_default();
    // Duel.SetTargetCard(dg)
    duel::set_target_card(d, &dg);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,dg,#dg,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(dg.clone()), dg.len() as u8, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,#dg//3)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, (dg.len() / 3) as i32);
    true
}
// s.drop(e,tp,eg,ep,ev,re,r,rp)
fn drop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg<1 or Duel.SendtoDeck(tg,nil,SEQ_DECKBOTTOM,REASON_EFFECT)<0 then return end
    if tg.len() < 1 || duel::send_to_deck(d, tg, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None) < 0 {
        return;
    }
    // local g=Duel.GetOperatedGroup():Filter(Card.IsLocation,nil,LOCATION_DECK)
    let og = duel::get_operated_group(d);
    let g = group::filter(d, &og, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX);
    // local ct=#g
    let ct = g.len();
    // if ct<1 then return end
    if ct < 1 {
        return;
    }
    // Duel.SortDeckbottom(tp,tp,ct)
    duel::sort_deckbottom(d, a.tp, a.tp, ct as u16);
    // if ct>=3 then
    if ct >= 3 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,ct//3,REASON_EFFECT)
        duel::draw(d, a.tp, (ct / 3) as u32, REASON_EFFECT);
    }
}
// s.rlsetconfilter(c)
fn rlsetconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() or c:IsPreviousLocation(LOCATION_MZONE)
    card::is_monster(d, c) || card::is_previous_location(d, c, LOCATION_MZONE)
}
// s.rlsetcon(e,tp,eg,ep,ev,re,r,rp)
fn rlsetcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.rlsetconfilter,1,e:GetHandler())
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    group::is_exists(d, &eg, &|d, c| rlsetconfilter(d, c), 1, h.into())
}
// s.setfilter(c)
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsNormalTrap() and c:IsSSetable()
    card::is_normal_trap(d, c) && card::is_s_setable(d, c, false, None)
}
// s.settg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.setfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && setfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.setfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&setfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
    // local g=Duel.SelectTarget(tp,s.setfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&setfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, g, 1, 0, 0);
    true
}
// s.setop(e,tp,eg,ep,ev,re,r,rp)
fn setop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsSSetable() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_s_setable(d, tc, false, None) {
        // Duel.SSet(tp,tc)
        duel::s_set(d, a.tp, CardSet::from_card(tc), None, true);
    }
}
// s.tgsetconfilter(c,tp)
fn tgsetconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsNormalTrap() and c:IsPreviousControler(tp)
    card::is_normal_trap(d, c) && card::is_previous_controler(d, c, tp)
}
// s.tgsetcon(e,tp,eg,ep,ev,re,r,rp)
fn tgsetcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and r&REASON_EFFECT==REASON_EFFECT and eg:IsExists(s.tgsetconfilter,1,nil,tp)
    a.rp == 1 - a.tp && (a.r & REASON_EFFECT) == REASON_EFFECT && group::is_exists(d, &a.eg.clone().nil(), &|d, c| tgsetconfilter(d, c, a.tp), 1, NO_EX)
}
