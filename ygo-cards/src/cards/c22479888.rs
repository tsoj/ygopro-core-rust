//! Clay Charge
use super::prelude::*;

pub const ID: u32 = 22479888;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[84327329], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e1, EVENT_BE_BATTLE_TARGET);
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
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local at=Duel.GetAttackTarget()
    // return at:IsControler(tp) and at:IsCode(84327329)
    let at = duel::get_attack_target(d).nil();
    card::is_controler(d, at, tp) && card::is_code(d, at, &[84327329])
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local a=Duel.GetAttacker()
    let atk = duel::get_attacker(d);
    // local at=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return a:IsOnField() and at:IsOnField() end
    if a.chk == Some(0) {
        return card::is_on_field(d, atk.nil()) && card::is_on_field(d, at.nil());
    }
    // if at:IsFacedown() then Duel.ConfirmCards(1-tp,at) end
    let at = at.nil();
    if card::is_facedown(d, at) {
        duel::confirm_cards(d, 1 - tp, at);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,Group.FromCards(a,at),2,0,0)
    let mut g = CardSet::new();
    group::add_card(&mut g, &CardSet::from_card(atk.nil()));
    group::add_card(&mut g, &CardSet::from_card(at));
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 2, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,800)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 800);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local a=Duel.GetAttacker()
    let atk = duel::get_attacker(d);
    // local at=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    // if a and a:IsRelateToBattle() and at and at:IsRelateToBattle() then
    if let Some(atk) = atk {
        if card::is_relate_to_battle(d, atk) {
            if let Some(at) = at {
                if card::is_relate_to_battle(d, at) {
                    // Duel.Destroy(Group.FromCards(a,at),REASON_EFFECT)
                    let mut g = CardSet::new();
                    group::add_card(&mut g, &CardSet::from_card(atk));
                    group::add_card(&mut g, &CardSet::from_card(at));
                    duel::destroy(d, g, REASON_EFFECT, None, None);
                    // Duel.Damage(1-tp,800,REASON_EFFECT)
                    duel::damage(d, 1 - tp, 800, REASON_EFFECT, false, None);
                }
            }
        }
    }
}
