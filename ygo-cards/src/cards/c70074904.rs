//! D.D. Assailant
use super::prelude::*;

pub const ID: u32 = 70074904;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // remove
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_BATTLED);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsStatus(STATUS_BATTLE_DESTROYED) and c:IsStatus(STATUS_OPPO_BATTLE)
    card::is_status(d, c, STATUS_BATTLE_DESTROYED) && card::is_status(d, c, STATUS_OPPO_BATTLE)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local a=Duel.GetAttacker()
    let at = duel::get_attacker(d);
    // local t=Duel.GetAttackTarget()
    let t = duel::get_attack_target(d);
    // if chk==0 then
    if a.chk == Some(0) {
        // return (t==c and a:IsAbleToRemove()) or (a==c and t~=nil and t:IsAbleToRemove())
        return (t == Some(c) && card::is_able_to_remove(d, at.nil(), None, None, None))
            || (at == Some(c) && t.is_some() && card::is_able_to_remove(d, t.nil(), None, None, None));
    }
    // local g=Group.CreateGroup()
    let mut g = CardSet::new();
    // if a:IsRelateToBattle() then g:AddCard(a) end
    if card::is_relate_to_battle(d, at.nil()) {
        group::add_card(&mut g, &CardSet::from_card(at.nil()));
    }
    // if t~=nil and t:IsRelateToBattle() then g:AddCard(t) end
    if let Some(t) = t {
        if card::is_relate_to_battle(d, t) {
            group::add_card(&mut g, &CardSet::from_card(t));
        }
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g.clone()), g.len() as u8, 0, 0);
    true
}
fn op(d: &mut Duel, _a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let at = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let dt = duel::get_attack_target(d);
    // local g=Group.FromCards(a,d)
    let cards: Vec<CardId> = [at, dt].into_iter().flatten().collect();
    let g = group::create_group(&cards);
    // local rg=g:Filter(Card.IsRelateToBattle,nil)
    let rg = group::filter(d, &g, &|d, c| card::is_relate_to_battle(d, c), NO_EX);
    // Duel.Remove(rg,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, rg, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
