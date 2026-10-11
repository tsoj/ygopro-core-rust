//! Counterforce
use super::prelude::*;

pub const ID: u32 = 30488793;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetCode(EVENT_ATTACK_DISABLED)
    effect::set_code(d, e1, EVENT_ATTACK_DISABLED);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    let a = duel::get_attacker(d);
    // local t=Duel.GetAttackTarget()
    let t = duel::get_attack_target(d);
    // return a:IsLocation(LOCATION_MZONE) and t and t:IsLocation(LOCATION_MZONE) and t:IsPosition(POS_FACEUP_ATTACK)
    card::is_location(d, a.nil(), LOCATION_MZONE)
        && match t {
            Some(t) => card::is_location(d, t, LOCATION_MZONE) && card::is_position(d, t, POS_FACEUP_ATTACK),
            None => false,
        }
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local a=Duel.GetAttacker()
    let at = duel::get_attacker(d);
    // local t=Duel.GetAttackTarget()
    let t = duel::get_attack_target(d);
    // local g=Group.FromCards(a,t)
    let mut g = CardSet::new();
    group::add_card(&mut g, &CardSet::from_card(at.nil()));
    group::add_card(&mut g, &CardSet::from_card(t.nil()));
    // local dam=math.abs(a:GetAttack()-t:GetAttack())
    let dam = (card::get_attack(d, at.nil()) - card::get_attack(d, t.nil())).abs() as i64;
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(dam)
    duel::set_target_param(d, dam as u32);
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,0,0,1-tp,dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, dam as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g<2 then return end
    if g.len() < 2 {
        return;
    }
    // local c1=g:GetFirst()
    // local c2=g:GetNext()
    let order = group::iter(d, &g);
    let (c1, c2) = (order[0], order[1]);
    // if c1:IsFaceup() and c2:IsFaceup() then
    if card::is_faceup(d, c1) && card::is_faceup(d, c2) {
        // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
        let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
        // local dam=math.abs(c1:GetAttack()-c2:GetAttack())
        let dam = (card::get_attack(d, c1) - card::get_attack(d, c2)).abs() as i64;
        // Duel.Damage(p,dam,REASON_EFFECT)
        duel::damage(d, p, dam, REASON_EFFECT, false, None);
    }
}
