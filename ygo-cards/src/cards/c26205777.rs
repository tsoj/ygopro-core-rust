//! Thestalos the Firestorm Monarch
use super::prelude::*;

pub const ID: u32 = 26205777;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // handes
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_HANDES + CATEGORY_DAMAGE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsTributeSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_tribute_summoned(d, h)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,1-tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - a.tp, 1);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_HAND);
    // if #g>0 then
    if !g.is_empty() {
        // local sg=g:RandomSelect(1-tp,1)
        let sg = group::random_select(d, &g, 1 - a.tp, 1);
        // Duel.SendtoGrave(sg,REASON_EFFECT|REASON_DISCARD)
        duel::send_to_grave(d, sg.clone(), REASON_EFFECT | REASON_DISCARD, None, None);
        // local tc=sg:GetFirst()
        let tc = group::get_first(d, &sg).nil();
        // if tc:IsMonster() then
        if card::is_monster(d, tc) {
            // Duel.Damage(1-tp,tc:GetLevel()*100,REASON_EFFECT)
            let lv = card::get_level(d, tc);
            duel::damage(d, 1 - a.tp, lv as i64 * 100, REASON_EFFECT, false, None);
        }
    }
}
