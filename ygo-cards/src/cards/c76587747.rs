//! Time Thief Retrograde
//! クロノダイバー・レトログラード
use super::prelude::*;

pub const ID: u32 = 76587747;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TIME_THIEF], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // negate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_NEGATE)
    effect::set_category(d, e1, CATEGORY_NEGATE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_TIME_THIEF) and c:IsType(TYPE_XYZ)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_TIME_THIEF as u16]) && card::is_type(d, c, TYPE_XYZ)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return re:IsHasType(EFFECT_TYPE_ACTIVATE) and Duel.IsChainNegatable(ev)
    //     and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
    effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE)
        && duel::is_chain_negatable(d, a.ev as u8)
        && duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    let negated = duel::negate_activation(d, a.ev as u8);
    if negated {
        let re = a.re.nil();
        let rc = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rc, re) {
            // local rc=re:GetHandler()
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
            // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE,0,nil):Filter(aux.NOT(Card.IsStatus),nil,STATUS_BATTLE_DESTROYED)
            let g0 = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_MZONE, 0, NO_EX);
            let g = group::filter(d, &g0, &|d, c| !card::is_status(d, c, STATUS_BATTLE_DESTROYED), NO_EX);
            // local tc=g:Select(tp,1,1,nil):GetFirst()
            let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            let tc = group::get_first(d, &sel);
            // if tc then
            if let Some(tc) = tc {
                // rc:CancelToGrave()
                card::cancel_to_grave(d, rc, true);
                // Duel.Overlay(tc,rc)
                duel::overlay(d, tc, rc, false);
            }
        }
    }
}
