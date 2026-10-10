//! Secret Password
//! Scripted by AlphaKretin and Naim
use super::prelude::*;

pub const ID: u32 = 61976639;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_KI_SIKIL, SET_LIL_LA, SET_EVIL_TWIN, SET_LIVE_TWIN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `local add=g:IsExists(Card.IsSetCard,1,nil,SET_KI_SIKIL) and g:IsExists(Card.IsSetCard,1,nil,SET_LIL_LA)`
/// with `g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,nil)`
fn add_flag(d: &mut Duel, tp: u8) -> bool {
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, NO_EX);
    group::is_exists(d, &g, &|d, c| card::is_set_card(d, c, &[SET_KI_SIKIL as u16]), 1, NO_EX)
        && group::is_exists(d, &g, &|d, c| card::is_set_card(d, c, &[SET_LIL_LA as u16]), 1, NO_EX)
}
/// `s.thfilter(c,add)`
fn thfilter(d: &mut Duel, c: CardId, add: bool) -> bool {
    // local c1=(c:IsSetCard(SET_EVIL_TWIN) or c:IsSetCard(SET_LIVE_TWIN)) and c:IsSpellTrap()
    let c1 = (card::is_set_card(d, c, &[SET_EVIL_TWIN as u16]) || card::is_set_card(d, c, &[SET_LIVE_TWIN as u16])) && card::is_spell_trap(d, c);
    // local c2=c:IsSetCard(SET_EVIL_TWIN) and c:IsMonster()
    let c2 = card::is_set_card(d, c, &[SET_EVIL_TWIN as u16]) && card::is_monster(d, c);
    // return (c1 or (add and c2)) and c:IsAbleToHand()
    (c1 || (add && c2)) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=...; local add=...
        let add = add_flag(d, a.tp);
        // return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,add)
        return duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c, add)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=...; local add=...
    let add = add_flag(d, tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,add)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| thfilter(d, c, add)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if group::get_count(&g) > 0 {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
