//! Ultimate Baseball Kid
use super::prelude::*;

pub const ID: u32 = 67934141;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atkup
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_value_fn(d, e1, effect_card_value(val));
    card::register_effect(d, c, e1, false);
    // damage
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_cost(d, e2, Some(cost_target(damcost)));
    effect::set_target(d, e2, Some(cost_target(damtg)));
    effect::set_operation(d, e2, Some(operation(damop)));
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttribute(ATTRIBUTE_FIRE)
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_FIRE)
}
/// `s.val(e,c)`
fn val(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(s.filter,c:GetControler(),LOCATION_MZONE,LOCATION_MZONE,c)*1000
    let p = card::get_controler(d, c);
    duel::get_matching_group_count(d, Some(&filter), p, LOCATION_MZONE, LOCATION_MZONE, c.into()) as i64 * 1000
}
/// `s.costfilter(c)`
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttribute(ATTRIBUTE_FIRE) and c:IsAbleToGraveAsCost()
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_FIRE) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.damcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_MZONE,0,1,e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&costfilter), a.tp, LOCATION_MZONE, 0, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_MZONE,0,1,1,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::select_matching_card(d, a.tp, Some(&costfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, h.into()).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(500)
    duel::set_target_param(d, 500);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,0,0,1-tp,500)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 500);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let (p, dd) = (info[0].int() as u8, info[1].int());
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, p, dd, REASON_EFFECT, false, None);
}
