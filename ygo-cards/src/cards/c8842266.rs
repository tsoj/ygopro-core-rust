//! Poison of the Old Man
use super::prelude::*;

pub const ID: u32 = 8842266;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // rec or dam
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EFFECT)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EFFECT as u64);
    // local op=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1))
    let op = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)]);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    if op == 0 {
        // e:SetCategory(CATEGORY_RECOVER)
        effect::set_category(d, a.e, CATEGORY_RECOVER);
        // Duel.SetTargetPlayer(tp)
        duel::set_target_player(d, a.tp);
        // Duel.SetTargetParam(1200)
        duel::set_target_param(d, 1200);
        // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,1200)
        duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, 1200);
    } else {
        // e:SetCategory(CATEGORY_DAMAGE)
        effect::set_category(d, a.e, CATEGORY_DAMAGE);
        // Duel.SetTargetPlayer(1-tp)
        duel::set_target_player(d, 1 - a.tp);
        // Duel.SetTargetParam(800)
        duel::set_target_param(d, 800);
        // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,800)
        duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 800);
    }
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // if e:GetLabel()==0 then Duel.Recover(p,d,REASON_EFFECT)
    if effect::get_label(d, a.e) == 0 {
        duel::recover(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
    } else {
        // else Duel.Damage(p,d,REASON_EFFECT) end
        duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
    }
}
