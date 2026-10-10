//! Numinous Healer
use super::prelude::*;

pub const ID: u32 = 2130625;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // recover
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_category(d, e1, CATEGORY_RECOVER);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e1, EVENT_DAMAGE);
    effect::set_condition(d, e1, Some(condition(reccon)));
    effect::set_target(d, e1, Some(cost_target(rectg)));
    effect::set_operation(d, e1, Some(operation(recop)));
    card::register_effect(d, c, e1, false);
}
fn reccon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==tp
    a.ep == a.tp
}
fn rectg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(1000)
    duel::set_target_param(d, 1000);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,1000)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, 1000);
    true
}
fn recop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let (p, amount) = (info[0].int() as u8, info[1].int());
    // Duel.Recover(p,d,REASON_EFFECT)
    duel::recover(d, p, amount, REASON_EFFECT, false, None);
    // local gc=Duel.GetMatchingGroupCount(Card.IsCode,p,LOCATION_GRAVE,0,nil,id)
    let gc = duel::get_matching_group_count(d, Some(&|d, c| card::is_code(d, c, &[ID])), p, LOCATION_GRAVE, 0, NO_EX);
    // if gc>0 then Duel.Recover(p,500*gc,REASON_EFFECT) end
    if gc > 0 {
        duel::recover(d, p, (500 * gc) as i64, REASON_EFFECT, false, None);
    }
}
