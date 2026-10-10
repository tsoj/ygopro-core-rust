//! Roulette Spider
use super::prelude::*;

pub const ID: u32 = 36708764;

pub const INFO: ScriptInfo = ScriptInfo { roll_dice: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DICE+CATEGORY_DAMAGE+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DICE + CATEGORY_DAMAGE + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(_d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==1-tp
    a.ep == 1 - a.tp
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local at=Duel.GetAttacker()
    let at = duel::get_attacker(d).nil();
    // Duel.SetTargetCard(at)
    duel::set_target_card(d, at);
    // Duel.SetOperationInfo(0,CATEGORY_DICE,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, a.tp, 1);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local dc=Duel.TossDice(tp,1)
    let dc = duel::toss_dice(d, a.tp, 1, 0)[0];
    if dc == 1 {
        // local lp=Duel.GetLP(tp)
        let lp = duel::get_lp(d, a.tp);
        // Duel.SetLP(tp,math.ceil(lp/2))
        duel::set_lp(d, a.tp, (lp as f64 / 2.0).ceil() as i32);
        return;
    } else if dc == 2 {
        // Duel.ChangeAttackTarget(nil)
        duel::change_attack_target(d, None, false);
        return;
    } else if dc == 3 {
        // local bc=Duel.GetAttackTarget()
        let bc = duel::get_attack_target(d);
        // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,0,bc)
        let g = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, 0, bc.map_or(NO_EX, |b| b.into()));
        // if #g>0 then
        if !g.is_empty() {
            // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 0));
            // local tc=g:Select(tp,1,1,nil):GetFirst()
            let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
            let tc = group::get_first(d, &sel).nil();
            // local at=Duel.GetAttacker()
            let at = duel::get_attacker(d).nil();
            // if at:CanAttack() and not at:IsImmuneToEffect(e) and not tc:IsImmuneToEffect(e) then
            if card::can_attack(d, at) && !card::is_immune_to_effect(d, at, a.e) && !card::is_immune_to_effect(d, tc, a.e) {
                // Duel.CalculateDamage(at,tc)
                duel::calculate_damage(d, at, Some(tc), false);
            }
        }
        return;
    } else if dc == 4 {
        // local at=Duel.GetAttacker()
        let at = duel::get_attacker(d).nil();
        // local g=Duel.GetMatchingGroup(nil,tp,0,LOCATION_MZONE,at)
        let g = duel::get_matching_group(d, None, a.tp, 0, LOCATION_MZONE, at.into());
        // if #g>0 then
        if !g.is_empty() {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_APPLYTO)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_APPLYTO as u64);
            // local tc=g:Select(tp,1,1,nil):GetFirst()
            let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
            let tc = group::get_first(d, &sel).nil();
            // local at=Duel.GetAttacker()
            let at = duel::get_attacker(d).nil();
            // if at:CanAttack() and not at:IsImmuneToEffect(e) and not tc:IsImmuneToEffect(e) then
            if card::can_attack(d, at) && !card::is_immune_to_effect(d, at, a.e) && !card::is_immune_to_effect(d, tc, a.e) {
                // Duel.CalculateDamage(at,tc)
                duel::calculate_damage(d, at, Some(tc), false);
            }
        }
        return;
    } else if dc == 5 {
        // local at=Duel.GetFirstTarget()
        let at = duel::first_target(d);
        // if at and at:IsRelateToEffect(e) and Duel.NegateAttack() and at:GetAttack()>0 then
        if let Some(at) = at {
            if card::is_relate_to_effect(d, at, a.e) && duel::negate_attack(d) && card::get_attack(d, at) > 0 {
                // Duel.Damage(1-tp,at:GetAttack(),REASON_EFFECT)
                let atk = card::get_attack(d, at) as i64;
                duel::damage(d, 1 - a.tp, atk, REASON_EFFECT, false, None);
            }
        }
        return;
    } else {
        // local at=Duel.GetFirstTarget()
        let at = duel::first_target(d);
        // if at and at:IsRelateToEffect(e) and at:IsControler(1-tp) and at:IsMonster() then
        if let Some(at) = at {
            if card::is_relate_to_effect(d, at, a.e) && card::is_controler(d, at, 1 - a.tp) && card::is_monster(d, at) {
                // Duel.Destroy(at,REASON_EFFECT)
                duel::destroy(d, at, REASON_EFFECT, None, None);
            }
        }
    }
}
