//! Sinister Shadow Games
use super::prelude::*;

pub const ID: u32 = 77505534;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SHADDOLL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send 1 "Shaddoll" card from deck to GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_POSITION)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_POSITION);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_ATTACK|TIMING_BATTLE_START)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_ATTACK | TIMING_BATTLE_START));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SHADDOLL) and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_SHADDOLL as u16]) && card::is_able_to_grave(d, c, None)
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.posfilter(c)`
fn posfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() and c:IsSetCard(SET_SHADDOLL)
    card::is_facedown(d, c) && card::is_set_card(d, c, &[SET_SHADDOLL as u16])
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&filter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SendtoGrave(g,REASON_EFFECT)>0 and g:GetFirst():IsLocation(LOCATION_GRAVE) then
    if !g.is_empty() && duel::send_to_grave(d, g.clone(), REASON_EFFECT, None, None) > 0 {
        let first = group::get_first(d, &g).nil();
        if card::is_location(d, first, LOCATION_GRAVE) {
            // local tg=Duel.GetMatchingGroup(s.posfilter,tp,LOCATION_MZONE,0,nil)
            let tg = duel::get_matching_group(d, Some(&posfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
            // if #tg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
            if !tg.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_POSCHANGE as u64);
                // local sg=tg:Select(tp,1,#tg,nil)
                let sg = group::select(d, &tg, a.tp, 1, tg.len() as u16, false, NO_EX).nil();
                // Duel.ChangePosition(sg,POS_FACEUP_DEFENSE)
                duel::change_position(d, sg, POS_FACEUP_DEFENSE as u8, None, None, None, false);
            }
        }
    }
}
