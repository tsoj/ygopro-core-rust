//! Mecha-Dog Marron
use super::prelude::*;

pub const ID: u32 = 94667532;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetCondition(s.damcon)
    effect::set_condition(d, e1, Some(condition(damcon)));
    // e1:SetTarget(s.damtg)
    effect::set_target(d, e1, Some(cost_target(damtg)));
    // e1:SetOperation(s.damop)
    effect::set_operation(d, e1, Some(operation(damop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD) and e:GetHandler():IsReason(REASON_DESTROY)
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_ONFIELD) && card::is_reason(d, h, REASON_DESTROY)
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local p=PLAYER_ALL
    let mut p = PLAYER_ALL as u8;
    // if not e:GetHandler():IsReason(REASON_BATTLE) then
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_reason(d, h, REASON_BATTLE) {
        // p=1-tp
        p = 1 - a.tp;
    }
    // Duel.SetTargetPlayer(p)
    duel::set_target_player(d, p);
    // Duel.SetTargetParam(1000)
    duel::set_target_param(d, 1000);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,p,1000)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, p, 1000);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let (p, dmg) = (info[0].int() as u8, info[1].int());
    // if p==PLAYER_ALL then
    if p as u32 == PLAYER_ALL {
        // Duel.Damage(0,d,REASON_EFFECT,true)
        duel::damage(d, 0, dmg, REASON_EFFECT, true, None);
        // Duel.Damage(1,d,REASON_EFFECT,true)
        duel::damage(d, 1, dmg, REASON_EFFECT, true, None);
        // Duel.RDComplete()
        duel::rd_complete(d);
    } else {
        // Duel.Damage(p,d,REASON_EFFECT)
        duel::damage(d, p, dmg, REASON_EFFECT, false, None);
    }
}
