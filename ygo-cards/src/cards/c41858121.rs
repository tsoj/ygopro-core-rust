//! Vylon Matter
use super::prelude::*;

pub const ID: u32 = 41858121;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_DRAW + CATEGORY_DESTROY);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_EQUIP) and c:IsAbleToDeck()
    card::is_type(d, c, TYPE_EQUIP) && card::is_able_to_deck(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,3,nil)
    //     and ((Duel.IsPlayerCanDraw(tp) and Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>0)
    //     or Duel.IsExistingMatchingCard(aux.TRUE,tp,0,LOCATION_ONFIELD,1,nil)) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_GRAVE, 0, 3, NO_EX)
            && ((duel::is_player_can_draw(d, tp, 0) && duel::get_field_group_count(d, tp, LOCATION_DECK, 0) > 0)
                || duel::is_existing_matching_card(d, None, tp, 0, LOCATION_ONFIELD, 1, NO_EX));
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,3,3,nil)
    let g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_GRAVE, 0, 3, 3, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,3,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 3, 0, 0);
    true
}
/// `s.tgfilter(c,e)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return not c:IsRelateToEffect(e)
    !card::is_relate_to_effect(d, c, e)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS as u32]).first().and_then(|v| v.as_group().cloned()).nil();
    // if g:IsExists(s.tgfilter,1,nil,e) then return end
    if group::is_exists(d, &g, &|d, c| tgfilter(d, c, e), 1, NO_EX) {
        return;
    }
    // Duel.SendtoDeck(g,nil,SEQ_DECKTOP,REASON_EFFECT)
    duel::send_to_deck(d, g, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    // Duel.ShuffleDeck(tp)
    duel::shuffle_deck(d, tp);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // local b1=Duel.IsPlayerCanDraw(tp,1)
    let b1 = duel::is_player_can_draw(d, tp, 1);
    // local b2=Duel.IsExistingMatchingCard(aux.TRUE,tp,0,LOCATION_ONFIELD,1,nil)
    let b2 = duel::is_existing_matching_card(d, None, tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,0)
    duel::hint(d, HINT_SELECTMSG as u8, tp, 0);
    // local op=0
    // if b1 and b2 then op=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1))
    let op = if b1 && b2 {
        duel::select_option(d, tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)])
    } else if b1 {
        // elseif b1 then op=Duel.SelectOption(tp,aux.Stringid(id,0))
        duel::select_option(d, tp, true, &[aux::stringid(ID, 0)])
    } else if b2 {
        // elseif b2 then Duel.SelectOption(tp,aux.Stringid(id,1)) op=1
        duel::select_option(d, tp, true, &[aux::stringid(ID, 1)]);
        1
    } else {
        // else return end
        return;
    };
    if op == 0 {
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, tp, 1, REASON_EFFECT);
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local dg=Duel.SelectMatchingCard(tp,aux.TRUE,tp,0,LOCATION_ONFIELD,1,1,nil)
        let dg = duel::select_matching_card(d, tp, None, tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
        // Duel.Destroy(dg,REASON_EFFECT)
        duel::destroy(d, dg, REASON_EFFECT, None, None);
    }
}
