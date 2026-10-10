//! Neo-Spacian Grand Mole
use super::prelude::*;

pub const ID: u32 = 80344569;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // return to hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_BATTLE_START);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetAttackTarget()==c or (Duel.GetAttacker()==c and Duel.GetAttackTarget()~=nil) end
    if a.chk == Some(0) {
        return duel::get_attack_target(d) == Some(c) || (duel::get_attacker(d) == Some(c) && duel::get_attack_target(d).is_some());
    }
    // local g=Group.FromCards(Duel.GetAttacker(),Duel.GetAttackTarget())
    // (a nil argument is skipped, as in Group.CreateGroup)
    let mut g = CardSet::new();
    if let Some(x) = duel::get_attacker(d) {
        g.insert(x);
    }
    if let Some(x) = duel::get_attack_target(d) {
        g.insert(x);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, _a: &EffArgs) {
    // local g=Group.CreateGroup()
    let mut g = CardSet::new();
    // local c=Duel.GetAttacker()
    let c = duel::get_attacker(d).nil();
    // if c:IsRelateToBattle() then g:AddCard(c) end
    if card::is_relate_to_battle(d, c) {
        g.insert(c);
    }
    // c=Duel.GetAttackTarget()
    let c = duel::get_attack_target(d);
    // if c~=nil and c:IsRelateToBattle() then g:AddCard(c) end
    if let Some(c) = c {
        if card::is_relate_to_battle(d, c) {
            g.insert(c);
        }
    }
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g, nil, REASON_EFFECT)
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
