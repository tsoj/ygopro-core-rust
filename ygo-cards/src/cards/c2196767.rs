//! Gambler of Legend
use super::prelude::*;

pub const ID: u32 = 2196767;
pub const INFO: ScriptInfo = ScriptInfo { toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Toss 3 coins and apply the appropriate effect
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_HANDES + CATEGORY_COIN);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_target(d, e1, Some(cost_target(destg)));
    effect::set_operation(d, e1, Some(operation(desop)));
    card::register_effect(d, c, e1, false);
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_COIN,nil,0,tp,3)
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, a.tp, 3);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c1,c2,c3=Duel.TossCoin(tp,3)
    let coins = duel::toss_coin(d, a.tp, 3);
    // local total_heads=Duel.CountHeads(c1,c2,c3)
    let total_heads = duel::count_heads(&coins);
    if total_heads == 3 {
        // local g=Duel.GetMatchingGroup(nil,tp,0,LOCATION_MZONE,nil)
        let g = duel::get_matching_group(d, None, a.tp, 0, LOCATION_MZONE, NO_EX);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    } else if total_heads == 2 {
        // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND):RandomSelect(tp,1)
        let fg = duel::get_field_group(d, a.tp, 0, LOCATION_HAND);
        let g = group::random_select(d, &fg, a.tp, 1);
        // Duel.SendtoGrave(g,REASON_EFFECT|REASON_DISCARD)
        duel::send_to_grave(d, g, REASON_EFFECT | REASON_DISCARD, None, None);
    } else if total_heads == 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectMatchingCard(tp,nil,tp,LOCATION_ONFIELD,0,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, None, a.tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    } else if duel::count_tails(&coins) == 3 {
        // local g=Duel.GetFieldGroup(tp,LOCATION_HAND,0)
        let g = duel::get_field_group(d, a.tp, LOCATION_HAND, 0);
        // Duel.SendtoGrave(g,REASON_EFFECT|REASON_DISCARD)
        duel::send_to_grave(d, g, REASON_EFFECT | REASON_DISCARD, None, None);
    }
}
