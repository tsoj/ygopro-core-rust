//! Generaider Boss Fight
use super::prelude::*;

pub const ID: u32 = 37931734;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GENERAIDER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GENERAIDER) and c:IsType(TYPE_FIELD) and c:GetActivateEffect():IsActivatable(tp,true,true)
    if !card::is_set_card(d, c, &[SET_GENERAIDER as u16]) || !card::is_type(d, c, TYPE_FIELD) {
        return false;
    }
    let ae = card::get_activate_effect(d, c).first().copied().nil();
    effect::is_activatable(d, ae, tp, true, true)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,tp)
    //     and Duel.IsPlayerCanDraw(1-tp,1) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.tp)), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_player_can_draw(d, 1 - a.tp, 1);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,1-tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, 1 - a.tp, 1);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 0));
    // local tc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.filter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,tp):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(|d, c| filter(d, c, a.tp))), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if not Duel.ActivateFieldSpell(tc,e,tp,eg,ep,ev,re,r,rp) then return end
    if !duel::activate_field_spell(d, tc, a, None) {
        return;
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.Draw(1-tp,1,REASON_EFFECT)
    duel::draw(d, 1 - a.tp, 1, REASON_EFFECT);
}
