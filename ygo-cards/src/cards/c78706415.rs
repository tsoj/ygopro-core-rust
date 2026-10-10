//! Fiber Jar
use super::prelude::*;

pub const ID: u32 = 78706415;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetFieldGroup(tp,LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE,LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE)
    let loc = LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE;
    let g = duel::get_field_group(d, a.tp, loc, loc);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, a.e);
    // local g=Duel.GetFieldGroup(tp,LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE,LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE)
    let loc = LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE;
    let mut g = duel::get_field_group(d, tp, loc, loc);
    // g:Remove(Card.IsStatus,nil,STATUS_BATTLE_DESTROYED)
    group::remove(d, &mut g, &|d, c| card::is_status(d, c, STATUS_BATTLE_DESTROYED), NO_EX);
    // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
    duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    // local tg=Duel.GetOperatedGroup():Filter(Card.IsLocation,nil,LOCATION_DECK)
    let og = duel::get_operated_group(d);
    let tg = group::filter(d, &og, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX);
    // if tg:IsExists(Card.IsControler,1,nil,tp) then Duel.ShuffleDeck(tp) end
    if group::is_exists(d, &tg, &|d, c| card::is_controler(d, c, tp), 1, NO_EX) {
        duel::shuffle_deck(d, tp);
    }
    // if tg:IsExists(Card.IsControler,1,nil,1-tp) then Duel.ShuffleDeck(1-tp) end
    if group::is_exists(d, &tg, &|d, c| card::is_controler(d, c, 1 - tp), 1, NO_EX) {
        duel::shuffle_deck(d, 1 - tp);
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.Draw(tp,5,REASON_EFFECT)
    duel::draw(d, tp, 5, REASON_EFFECT);
    // Duel.Draw(1-tp,5,REASON_EFFECT)
    duel::draw(d, 1 - tp, 5, REASON_EFFECT);
}
