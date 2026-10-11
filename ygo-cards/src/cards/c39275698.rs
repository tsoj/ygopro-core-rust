//! Harpie's Feather Rest
//! scripted by Logical Nonsense
//! Substitute ID
use super::prelude::*;

pub const ID: u32 = 39275698;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_HARPIE as u32], listed_names: &[CARD_HARPIE_LADY, CARD_HARPIE_LADY_SISTERS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_DRAW);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.drtg)
    effect::set_target(d, e1, Some(cost_target(drtg)));
    // e1:SetOperation(s.drop)
    effect::set_operation(d, e1, Some(operation(drop_op)));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.drfilter(c)`: "Harpie Lady" or "Harpie Sisters"
fn drfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_HARPIE_LADY,CARD_HARPIE_LADY_SISTERS) and c:IsAbleToDeck()
    card::is_code(d, c, &[CARD_HARPIE_LADY, CARD_HARPIE_LADY_SISTERS]) && card::is_able_to_deck(d, c)
}
/// `s.filter(c)`: a face-up Level 5 or higher "Harpie"
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_HARPIE) and c:IsLevelAbove(5)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_HARPIE as u16]) && card::is_level_above(d, c, 5)
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local ct=1
    // if Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE,0,1,nil) then ct=2 end
    let ct: u32 = if duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX) { 2 } else { 1 };
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.drfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && drfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,ct) and Duel.IsExistingTarget(s.drfilter,tp,LOCATION_GRAVE,0,3,nil) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, ct) && duel::is_existing_target(d, Some(&drfilter), tp, LOCATION_GRAVE, 0, 3, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.drfilter,tp,LOCATION_GRAVE,0,3,3,nil)
    let g = duel::select_target(d, tp, Some(&drfilter), tp, LOCATION_GRAVE, 0, 3, 3, false, NO_EX).nil();
    // e:SetLabel(ct)
    effect::set_label(d, e, &[ct as i64]);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,ct)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, ct as i32);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`: shuffle and draw
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tg=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let tg = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned();
    // if not tg or tg:FilterCount(Card.IsRelateToEffect,nil,e)~=3 then return end
    let Some(tg) = tg else { return };
    if group::filter_count(d, &tg, &|d, c| card::is_relate_to_effect(d, c, e), NO_EX) != 3 {
        return;
    }
    // Duel.SendtoDeck(tg,nil,SEQ_DECKTOP,REASON_EFFECT)
    duel::send_to_deck(d, tg, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    // local g=Duel.GetOperatedGroup()
    let g = duel::get_operated_group(d);
    // if g:IsExists(Card.IsLocation,1,nil,LOCATION_DECK) then Duel.ShuffleDeck(tp) end
    if group::is_exists(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK), 1, NO_EX) {
        duel::shuffle_deck(d, tp);
    }
    // local ct=g:FilterCount(Card.IsLocation,nil,LOCATION_DECK|LOCATION_EXTRA)
    let ct = group::filter_count(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK | LOCATION_EXTRA), NO_EX);
    // if ct==3 then
    if ct == 3 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,e:GetLabel(),REASON_EFFECT)
        let label = effect::get_label(d, e) as u32;
        duel::draw(d, tp, label, REASON_EFFECT);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
        effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
        // e1:SetDescription(aux.Stringid(id,2))
        effect::set_description(d, e1, aux::stringid(ID, 2));
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetTarget(s.splimit)
        effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_attribute(d, c, ATTRIBUTE_WIND))));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
}
