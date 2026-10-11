//! EMキャスト・チェンジ
//! Performapal Recasting
use super::prelude::*;

pub const ID: u32 = 71705144;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PERFORMAPAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_PERFORMAPAL) and c:IsMonster() and c:IsAbleToDeck() and not c:IsPublic()
    card::is_set_card(d, c, &[SET_PERFORMAPAL as u16]) && card::is_monster(d, c) && card::is_able_to_deck(d, c) && !card::is_public(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp)
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return duel::is_player_can_draw(d, a.tp, 0) && duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_HAND, 0, 1, c.into());
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
fn activate(d: &mut Duel, _a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // Duel.Hint(HINT_SELECTMSG,p,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(p,s.filter,p,LOCATION_HAND,0,1,63,nil)
    let g = duel::select_matching_card(d, p, Some(&filter), p, LOCATION_HAND, 0, 1, 63, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.ConfirmCards(1-p,g)
        duel::confirm_cards(d, 1 - p, g.clone());
        // local ct=Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        let ct = duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        // Duel.ShuffleDeck(p)
        duel::shuffle_deck(d, p);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(p,ct+1,REASON_EFFECT)
        duel::draw(d, p, (ct + 1) as u32, REASON_EFFECT);
    }
}
