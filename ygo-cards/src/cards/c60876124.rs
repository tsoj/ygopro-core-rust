//! Rare Value
use super::prelude::*;

pub const ID: u32 = 60876124;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_CRYSTAL_BEAST],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
// s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_CRYSTAL_BEAST)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16])
}
// s.condition
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_SZONE,0,2,nil)
    duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_SZONE, 0, 2, NO_EX)
}
// s.target
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 2);
    }
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
    true
}
// s.activate
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_SZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_SZONE, 0, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_TOGRAVE as u64);
        // local sg=g:Select(1-tp,1,1,nil)
        let sg = group::select(d, &g, 1 - a.tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoGrave(sg,REASON_EFFECT)
        duel::send_to_grave(d, sg.clone(), REASON_EFFECT, None, None);
        // if sg:GetFirst():IsLocation(LOCATION_GRAVE) then
        let first = group::get_first(d, &sg).nil();
        if card::is_location(d, first, LOCATION_GRAVE) {
            // Duel.Draw(tp,2,REASON_EFFECT)
            duel::draw(d, a.tp, 2, REASON_EFFECT);
        }
    }
}
