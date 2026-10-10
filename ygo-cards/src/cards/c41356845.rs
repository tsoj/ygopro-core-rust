//! Acid Trap Hole
use super::prelude::*;

pub const ID: u32 = 41356845;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_DESTROY + CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown()
    card::is_facedown(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, g, 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.ChangePosition(tc,POS_FACEUP_DEFENSE)~=0 then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && duel::change_position(d, tc, POS_FACEUP_DEFENSE as u8, None, None, None, false) != 0 {
            // if tc:IsDefenseBelow(2000) then
            if card::is_defense_below(d, tc, 2000) {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Destroy(tc,REASON_EFFECT)
                duel::destroy(d, tc, REASON_EFFECT, None, None);
            } else {
                // Duel.ConfirmCards(1-tc:GetControler(),tc)
                let p = 1 - card::get_controler(d, tc);
                duel::confirm_cards(d, p, tc);
                // Duel.ChangePosition(tc,POS_FACEDOWN_DEFENSE)
                duel::change_position(d, tc, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
            }
        }
    }
}
