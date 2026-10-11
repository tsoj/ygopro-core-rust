//! Litmus Doom Ritual
use super::prelude::*;

pub const ID: u32 = 8955148;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[72566043], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual.AddProcGreaterCode(c,8,nil,72566043)
    ritual::add_proc_greater_code(d, c, Some(ritual::Lv::N(8)), None, &[72566043]);
    // to deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.tdtg)
    effect::set_target(d, e1, Some(cost_target(tdtg)));
    // e1:SetOperation(s.tdop)
    effect::set_operation(d, e1, Some(operation(tdop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.costfilter(c)`
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(72566043) and c:IsAbleToDeck()
    card::is_code(d, c, &[72566043]) && card::is_able_to_deck(d, c)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local h = e:GetHandler()
    let h = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsAbleToDeck() and chkc:IsControler(tp)
    //   and chkc~=e:GetHandler() and Duel.IsPlayerCanDraw(tp,1) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE)
            && card::is_able_to_deck(d, chkc)
            && card::is_controler(d, chkc, tp)
            && chkc != h
            && duel::is_player_can_draw(d, tp, 1);
    }
    // if chk==0 then return e:GetHandler():IsAbleToDeck()
    //   and Duel.IsExistingTarget(s.costfilter,tp,LOCATION_GRAVE,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return card::is_able_to_deck(d, h)
            && duel::is_existing_target(d, Some(&costfilter), tp, LOCATION_GRAVE, 0, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.costfilter,tp,LOCATION_GRAVE,0,1,1,e:GetHandler())
    let mut g = duel::select_target(d, tp, Some(&costfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, h.into()).nil();
    // g:AddCard(e:GetHandler())
    group::add_card(&mut g, &CardSet::from_card(h));
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), 2, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) && card::is_relate_to_effect(d, tc.nil(), a.e) {
        // local g=Group.FromCards(c,tc)
        let mut g = CardSet::from_card(c);
        group::add_card(&mut g, &CardSet::from_card(tc.nil()));
        // Duel.SendtoDeck(g,nil,SEQ_DECKTOP,REASON_EFFECT)
        duel::send_to_deck(d, &g, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, tp);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, tp, 1, REASON_EFFECT);
    }
}
