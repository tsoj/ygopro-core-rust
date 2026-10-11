//! D - Time
use super::prelude::*;

pub const ID: u32 = 99075257;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ELEMENTAL_HERO, SET_DESTINY_HERO],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e1, EVENT_LEAVE_FIELD);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    // local tc=eg:GetFirst()
    let tc = group::get_first(d, &eg).nil();
    // local lv=tc:GetLevel()
    let _lv = card::get_level(d, tc);
    // return #eg==1 and tc:IsSetCard(SET_ELEMENTAL_HERO) and tc:IsPreviousPosition(POS_FACEUP) and tc:IsPreviousControler(tp)
    eg.len() == 1
        && card::is_set_card(d, tc, &[SET_ELEMENTAL_HERO as u16])
        && card::is_previous_position(d, tc, POS_FACEUP)
        && card::is_previous_controler(d, tc, tp)
}
/// `s.filter(c,lv)`
fn filter(d: &mut Duel, c: CardId, lv: u32) -> bool {
    // return c:IsLevelBelow(lv) and c:IsSetCard(SET_DESTINY_HERO) and c:IsAbleToHand()
    card::is_level_below(d, c, lv) && card::is_set_card(d, c, &[SET_DESTINY_HERO as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,eg:GetFirst():GetLevel()) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        let tc = group::get_first(d, &eg).nil();
        let lv = card::get_level(d, tc);
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, lv)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // e:SetLabel(eg:GetFirst():GetLevel())
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg).nil();
    let lv = card::get_level(d, tc);
    effect::set_label(d, a.e, &[lv as i64]);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil,e:GetLabel())
    let lv = effect::get_label(d, a.e) as u32;
    let g = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, lv)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g~=0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
