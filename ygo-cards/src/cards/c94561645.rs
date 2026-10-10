//! Counter Gate
use super::prelude::*;

pub const ID: u32 = 94561645;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DRAW+CATEGORY_SUMMON)
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_SUMMON);
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
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttacker():IsControler(1-tp) and Duel.GetAttackTarget()==nil
    let attacker = duel::get_attacker(d).nil();
    card::is_controler(d, attacker, 1 - a.tp) && duel::get_attack_target(d).is_none()
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateAttack() and Duel.Draw(tp,1,REASON_EFFECT)~=0 then
    if duel::negate_attack(d) && duel::draw(d, a.tp, 1, REASON_EFFECT) != 0 {
        // local tc=Duel.GetOperatedGroup():GetFirst()
        let operated = duel::get_operated_group(d);
        let tc = group::get_first(d, &operated);
        // if tc:IsMonster() and tc:IsSummonable(true,nil)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
        if card::is_monster(d, tc.nil())
            && card::is_summonable(d, tc.nil(), true, None, 0, None)
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0))
        {
            // Duel.Summon(tp,tc,true,nil)
            duel::summon(d, a.tp, tc.nil(), true, None, 0, None);
        }
    }
}
