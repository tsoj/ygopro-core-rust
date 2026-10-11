//! The Fountain in the Sky
use super::prelude::*;

pub const ID: u32 = 92223641;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // adjust
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e2, CATEGORY_RECOVER);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCost(s.cost)
    effect::set_cost(d, e2, Some(cost_target(cost_fn)));
    // e2:SetTarget(s.tg)
    effect::set_target(d, e2, Some(cost_target(tg)));
    // e2:SetOperation(s.op)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter(g,tp)`: returns the LIGHT card that left the field, or nil
fn filter(d: &mut Duel, g: &Group, tp: u8) -> Option<CardId> {
    // local c=g:GetFirst()
    let mut c = group::get_first(d, g);
    // if c:IsControler(1-tp) then c=g:GetNext() end
    if card::is_controler(d, c.nil(), 1 - tp) {
        c = group::iter(d, g).get(1).copied();
    }
    // if c and c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsLocation(LOCATION_GRAVE) then return c end
    if let Some(cc) = c {
        if card::is_attribute(d, cc, ATTRIBUTE_LIGHT) && card::is_location(d, cc, LOCATION_GRAVE) {
            return Some(cc);
        }
    }
    // return nil
    None
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // local rc=s.filter(eg,tp)
        let eg = a.eg.clone().nil();
        let rc = filter(d, &eg, a.tp);
        // return rc and not Duel.IsPlayerAffectedByEffect(rc:GetControler(),CARD_SPIRIT_ELIMINATION) and rc:IsAbleToRemoveAsCost()
        return match rc {
            Some(rc) => {
                let rp = card::get_controler(d, rc);
                duel::is_player_affected_by_effect(d, rp, CARD_SPIRIT_ELIMINATION).is_none() && card::is_able_to_remove_as_cost(d, rc, None)
            }
            None => false,
        };
    }
    // local rc=s.filter(eg,tp)
    let eg = a.eg.clone().nil();
    let rc = filter(d, &eg, a.tp).nil();
    // e:SetLabel(rc:GetAttack())
    let atk = card::get_attack(d, rc);
    effect::set_label(d, a.e, &[atk as i64]);
    // Duel.Remove(rc,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, rc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    true
}
/// `s.tg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    let lbl = effect::get_label(d, a.e);
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(e:GetLabel())
    duel::set_target_param(d, lbl as u32);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,e:GetLabel())
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, lbl as i32);
    true
}
/// `s.op(e,tp,...)`
fn op(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let amount = info[1].int();
    // Duel.Recover(p,d,REASON_EFFECT)
    duel::recover(d, p, amount, REASON_EFFECT, false, None);
}
