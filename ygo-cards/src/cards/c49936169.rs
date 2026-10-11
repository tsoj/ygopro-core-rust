//! U.A. Locker Room
//! Ｕ．Ａ．ロッカールーム
use super::prelude::*;

pub const ID: u32 = 49936169;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_FA, SET_UA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_RECOVER+CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_RECOVER + CATEGORY_TODECK + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToHand() and c:IsMonster() and (c:IsSetCard(SET_FA) or c:IsSetCard(SET_UA)) and (c:IsFaceup() or c:IsLocation(LOCATION_GRAVE))
    card::is_able_to_hand(d, c, None)
        && card::is_monster(d, c)
        && (card::is_set_card(d, c, &[SET_FA as u16]) || card::is_set_card(d, c, &[SET_UA as u16]))
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_GRAVE))
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and chkc:IsControler(tp) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && thfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,500)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, 500);
    true
}
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToDeck() and c:IsMonster() and not c:IsPublic() and c:IsSetCard({SET_FA,SET_UA})
    card::is_able_to_deck(d, c) && card::is_monster(d, c) && !card::is_public(d, c) && card::is_set_card(d, c, &[SET_FA as u16, SET_UA as u16])
}
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.SendtoHand(tc,nil,REASON_EFFECT)==1 then
    let Some(tc) = tc else {
        return;
    };
    if !card::is_relate_to_effect(d, tc, a.e) || duel::send_to_hand(d, tc, None, REASON_EFFECT, None) != 1 {
        return;
    }
    // if Duel.Recover(tp,500,REASON_EFFECT)==500 then
    if duel::recover(d, a.tp, 500, REASON_EFFECT, false, None) == 500 {
        // if Duel.IsExistingMatchingCard(s.tdfilter,tp,LOCATION_HAND,0,1,nil) and Duel.IsPlayerCanDraw(tp) and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if duel::is_existing_matching_card(d, Some(&tdfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX)
            && duel::is_player_can_draw(d, a.tp, 0)
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
            // local g=Duel.SelectMatchingCard(tp,s.tdfilter,tp,LOCATION_HAND,0,1,99,nil)
            let g = duel::select_matching_card(d, a.tp, Some(&tdfilter), a.tp, LOCATION_HAND, 0, 1, 99, false, NO_EX).nil();
            // if #g>0 then
            if !g.is_empty() {
                // Duel.ConfirmCards(1-tp,g)
                duel::confirm_cards(d, 1 - a.tp, g.clone());
                // local ct=Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
                let ct = duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
                // Duel.ShuffleDeck(tp)
                duel::shuffle_deck(d, a.tp);
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Draw(tp,ct,REASON_EFFECT)
                duel::draw(d, a.tp, ct as u32, REASON_EFFECT);
            }
        }
    }
}
