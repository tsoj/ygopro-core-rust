//! Satellarknight Alsahm
use super::prelude::*;

pub const ID: u32 = 65056481;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Inflict 1000 damage to your opponent
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    // e1a:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1a, CATEGORY_DAMAGE);
    // e1a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1a:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY, 0);
    // e1a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1a, EVENT_SUMMON_SUCCESS);
    // e1a:SetCountLimit(1,id)
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    // e1a:SetTarget(s.damtg)
    effect::set_target(d, e1a, Some(cost_target(damtg)));
    // e1a:SetOperation(s.damop)
    effect::set_operation(d, e1a, Some(operation(damop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    // e1b:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    // c:RegisterEffect(e1b)
    let e1b = effect::clone(d, e1a);
    effect::set_code(d, e1b, EVENT_FLIP_SUMMON_SUCCESS);
    card::register_effect(d, c, e1b, false);
    // local e1c=e1a:Clone()
    // e1c:SetCode(EVENT_SPSUMMON_SUCCESS)
    // c:RegisterEffect(e1c)
    let e1c = effect::clone(d, e1a);
    effect::set_code(d, e1c, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e1c, false);
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(1000)
    duel::set_target_param(d, 1000);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,1000)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 1000);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let dmg = info[1].int();
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, p, dmg, REASON_EFFECT, false, None);
}
