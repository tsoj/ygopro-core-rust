//! Tilted Try
use super::prelude::*;

pub const ID: u32 = 20216608;

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
    // Normal draw
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_DRAW)
    effect::set_code(d, e2, EVENT_DRAW);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetLabel(0)
    effect::set_label(d, e2, &[0]);
    // e2:SetCondition(s.drcon)
    effect::set_condition(d, e2, Some(condition(drcon)));
    // e2:SetTarget(s.drtg)
    effect::set_target(d, e2, Some(cost_target(drtg)));
    // e2:SetOperation(s.drop)
    effect::set_operation(d, e2, Some(operation(drop_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Effect draw
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW+CATEGORY_TOGRAVE)
    effect::set_category(d, e3, CATEGORY_TODECK + CATEGORY_DRAW + CATEGORY_TOGRAVE);
    // e3:SetLabel(1)
    effect::set_label(d, e3, &[1]);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if not (ep==tp and eg:IsExists(aux.NOT(Card.IsPublic),1,nil)) then return false end
    let eg = a.eg.clone().nil();
    if !(a.ep == a.tp && group::is_exists(d, &eg, &|d, c| !card::is_public(d, c), 1, NO_EX)) {
        return false;
    }
    // local mode=e:GetLabel()
    let mode = effect::get_label(d, a.e);
    // return (mode==0 and r==REASON_RULE) or (mode==1 and r==REASON_EFFECT)
    (mode == 0 && a.r == REASON_RULE) || (mode == 1 && a.r == REASON_EFFECT)
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local mode=e:GetLabel()
    let mode = effect::get_label(d, e);
    // local g=eg:Filter(aux.NOT(Card.IsPublic),nil)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| !card::is_public(d, c), NO_EX);
    // if chk==0 then return (mode==0 or c:IsAbleToGrave()) and ep==tp and #g>0 end
    if a.chk == Some(0) {
        return (mode == 0 || card::is_able_to_grave(d, c, None)) && a.ep == tp && !g.is_empty();
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local sg=g:Select(tp,1,1,nil):GetFirst()
    let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    let sg = group::get_first(d, &sel).nil();
    // Duel.ConfirmCards(1-tp,sg)
    duel::confirm_cards(d, 1 - tp, sg);
    // Duel.SetTargetCard(sg)
    duel::set_target_card(d, sg);
    // if mode==1 then
    if mode == 1 {
        // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,c,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(CardSet::from_card(c)), 1, 0, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,sg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(sg)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // if e:GetLabel()==1 and Duel.SendtoGrave(c,REASON_EFFECT)==0 then return end
    if effect::get_label(d, e) == 1 && duel::send_to_grave(d, c, REASON_EFFECT, None, None) == 0 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    // if not (tc and tc:IsRelateToEffect(e)) then return end
    let Some(tc) = duel::first_target(d) else { return };
    if !card::is_relate_to_effect(d, tc, e) {
        return;
    }
    // if Duel.SendtoDeck(tc,nil,SEQ_DECKBOTTOM,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_DECK) then
    if duel::send_to_deck(d, tc, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None) > 0 && card::is_location(d, tc, LOCATION_DECK) {
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, tp, 1, REASON_EFFECT);
    }
}
