//! Spiritual Fire Art - Kurenai
use super::prelude::*;

pub const ID: u32 = 42945701;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn release_filter(d: &mut Duel, c: CardId) -> bool {
    // Card.IsAttribute, called with ATTRIBUTE_FIRE as its extra argument
    card::is_attribute(d, c, ATTRIBUTE_FIRE)
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // e:SetLabel(1)
    effect::set_label(d, e, &[1]);
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,Card.IsAttribute,1,false,nil,nil,ATTRIBUTE_FIRE) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&release_filter), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,Card.IsAttribute,1,1,false,nil,nil,ATTRIBUTE_FIRE)
    let g = duel::select_release_group_cost(d, tp, Some(&release_filter), 1, 1, false, None, NO_EX);
    // local atk=g:GetFirst():GetTextAttack()
    let first = group::get_first(d, &g).nil();
    let mut atk = card::get_text_attack(d, first) as i64;
    // if atk<0 then atk=0 end
    if atk < 0 {
        atk = 0;
    }
    // e:SetLabel(atk)
    effect::set_label(d, e, &[atk]);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    // if chk==0 then
    if a.chk == Some(0) {
        // local res=e:GetLabel()~=0
        let res = effect::get_label(d, e) != 0;
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // return res
        return res;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(e:GetLabel())
    duel::set_target_param(d, effect::get_label(d, e) as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,e:GetLabel())
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, effect::get_label(d, e) as i32);
    // e:SetLabel(0)
    effect::set_label(d, e, &[0]);
    true
}
fn activate(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
