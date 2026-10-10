//! Neo-Spacian Air Hummingbird
use super::prelude::*;

pub const ID: u32 = 54959865;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // recover
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e1, CATEGORY_RECOVER);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, tp, 0, LOCATION_HAND) > 0;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, tp, 0);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    // local rt=Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)*500
    let rt = duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) as i64 * 500;
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER]);
    let p = info[0].int() as u8;
    // Duel.Recover(p,rt,REASON_EFFECT)
    duel::recover(d, p, rt, REASON_EFFECT, false, None);
}
