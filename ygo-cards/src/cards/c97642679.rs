//! Dark Master - Zorc
use super::prelude::*;

pub const ID: u32 = 97642679;
pub const INFO: ScriptInfo = ScriptInfo { roll_dice: true, listed_names: &[96420087], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // dice
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DICE);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DICE,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, a.tp, 1);
    // local g1=Duel.GetMatchingGroup(aux.TRUE,tp,0,LOCATION_MZONE,nil)
    let mut g1 = duel::get_matching_group(d, None, a.tp, 0, LOCATION_MZONE, NO_EX);
    // local g2=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_MZONE,0,nil)
    let g2 = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, 0, NO_EX);
    // if #g1~=0 and #g2~=0 then
    if !g1.is_empty() && !g2.is_empty() {
        // g1:Merge(g2)
        group::add_card(&mut g1, &g2);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g1,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g1), 1, 0, 0);
    }
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    // local d=Duel.TossDice(tp,1)
    let dice = duel::toss_dice(d, a.tp, 1, 0)[0];
    if dice == 1 || dice == 2 {
        // local g=Duel.GetMatchingGroup(aux.TRUE,tp,0,LOCATION_MZONE,nil)
        let g = duel::get_matching_group(d, None, a.tp, 0, LOCATION_MZONE, NO_EX);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    } else if dice == 6 {
        // local g=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_MZONE,0,nil)
        let g = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, 0, NO_EX);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.TRUE,tp,0,LOCATION_MZONE,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, None, a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
