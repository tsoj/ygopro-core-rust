//! ヘイト・クレバス
//! Icy Crevasse
use super::prelude::*;

pub const ID: u32 = 20721759;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // tograve
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_DAMAGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=eg:GetFirst()
    let eg = a.eg();
    let ec = group::get_first(d, &eg);
    // return rp~=tp and #eg==1 and ec:IsPreviousLocation(LOCATION_MZONE) and ec:IsPreviousControler(tp)
    //     and ec:IsReason(REASON_DESTROY) and ec:IsReason(REASON_EFFECT)
    if a.rp == a.tp || eg.len() != 1 {
        return false;
    }
    let ec = ec.nil();
    card::is_previous_location(d, ec, LOCATION_MZONE)
        && card::is_previous_controler(d, ec, a.tp)
        && card::is_reason(d, ec, REASON_DESTROY)
        && card::is_reason(d, ec, REASON_EFFECT)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_MZONE);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_target(d, a.tp, None, a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, g.clone(), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,g:GetFirst():GetBaseAttack())
    let tc = group::get_first(d, &g.nil()).nil();
    let atk = card::get_base_attack(d, tc);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, atk);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // Duel.SendtoGrave(tc,REASON_EFFECT)
    duel::send_to_grave(d, tc, REASON_EFFECT, None, None);
    // if not tc:IsLocation(LOCATION_GRAVE) then return end
    if !card::is_location(d, tc, LOCATION_GRAVE) {
        return;
    }
    // local atk=tc:GetBaseAttack()
    // if atk<0 then atk=0 end
    let atk = card::get_base_attack(d, tc).max(0);
    // Duel.Damage(1-tp,atk,REASON_EFFECT)
    duel::damage(d, 1 - a.tp, atk as i64, REASON_EFFECT, false, None);
}
