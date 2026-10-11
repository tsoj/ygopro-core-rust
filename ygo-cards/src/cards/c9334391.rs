//! Crystal Bond
use super::prelude::*;

pub const ID: u32 = 9334391;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_CRYSTAL_BEAST],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 "Crystal Beast" monster from deck
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TOHAND | CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
// s.thfilter(c,tp)
fn thfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_CRYSTAL_BEAST) and c:IsMonster() and c:IsAbleToHand()
    //     and Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_DECK,0,1,nil,c:GetCode())
    let code = card::get_code(d, c)[0];
    card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16])
        && card::is_monster(d, c)
        && card::is_able_to_hand(d, c, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| plfilter(d, x, code)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
// s.plfilter(c,code)
fn plfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return c:IsSetCard(SET_CRYSTAL_BEAST) and c:IsMonster() and not c:IsCode(code) and not c:IsForbidden()
    card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16])
        && card::is_monster(d, c)
        && !card::is_code(d, c, &[code])
        && !card::is_forbidden(d, c)
}
// s.target
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let mut ct = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0;
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) and not e:GetHandler():IsLocation(LOCATION_SZONE) then ct=ct-1 end
    if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        let h = effect::get_handler(d, a.e).nil();
        if !card::is_location(d, h, LOCATION_SZONE) {
            ct -= 1;
        }
    }
    if a.chk == Some(0) {
        return ct > 0 && duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// s.activate
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g1=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,tp)
    let g1 = duel::select_matching_card(d, a.tp, Some(&|d, c| thfilter(d, c, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g1<=0 then return end
    if g1.is_empty() {
        return;
    }
    // Duel.SendtoHand(g1,nil,REASON_EFFECT)
    duel::send_to_hand(d, g1.clone(), None, REASON_EFFECT, None);
    // if not Duel.GetOperatedGroup():GetFirst():IsLocation(LOCATION_HAND) then return end
    let operated = duel::get_operated_group(d);
    let op_first = group::get_first(d, &operated).nil();
    if !card::is_location(d, op_first, LOCATION_HAND) {
        return;
    }
    // Duel.ConfirmCards(1-tp,g1)
    duel::confirm_cards(d, 1 - a.tp, g1.clone());
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
    // local g2=Duel.SelectMatchingCard(tp,s.plfilter,tp,LOCATION_DECK,0,1,1,nil,g1:GetFirst():GetCode())
    let g1_first = group::get_first(d, &g1).nil();
    let code = card::get_code(d, g1_first)[0];
    let g2 = duel::select_matching_card(d, a.tp, Some(&|d, c| plfilter(d, c, code)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // local tc=g2:GetFirst()
    // if tc then
    if let Some(tc) = group::get_first(d, &g2) {
        // Duel.MoveToField(tc,tp,tp,LOCATION_SZONE,POS_FACEUP,true)
        duel::move_to_field(d, tc, a.tp, a.tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_value(d, e1, (TYPE_SPELL + TYPE_CONTINUOUS) as i64);
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // Duel.RaiseEvent(tc,EVENT_CUSTOM+CARD_CRYSTAL_TREE,e,0,tp,0,0)
        duel::raise_event(d, CardSet::from_card(tc), EVENT_CUSTOM + CARD_CRYSTAL_TREE, Some(a.e), 0, a.tp, 0, 0);
    }
}
