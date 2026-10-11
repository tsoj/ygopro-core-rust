//! Changing Destiny
use super::prelude::*;

pub const ID: u32 = 24673894;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
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
    // return Duel.IsTurnPlayer(1-tp)
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=Duel.GetAttacker()
    let tc = duel::get_attacker(d);
    // if chkc then return chkc==tc end
    if let Some(chkc) = a.chkc() {
        return Some(chkc) == tc;
    }
    // if chk==0 then return tc:IsLocation(LOCATION_MZONE) and tc:IsAttackPos()
    //     and tc:IsCanChangePosition() and tc:IsCanBeEffectTarget(e) end
    if a.chk == Some(0) {
        let tc = tc.nil();
        return card::is_location(d, tc, LOCATION_MZONE)
            && card::is_attack_pos(d, tc)
            && card::is_can_change_position(d, tc)
            && card::is_can_be_effect_target(d, tc, Some(a.e));
    }
    // Duel.SetTargetCard(tc)
    if let Some(tc) = tc {
        duel::set_target_card(d, tc);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetAttacker()
    let Some(tc) = duel::get_attacker(d) else {
        return;
    };
    // if tc and tc:IsRelateToEffect(e) and Duel.NegateAttack() and Duel.ChangePosition(tc,POS_FACEUP_DEFENSE)~=0 then
    if card::is_relate_to_effect(d, tc, e) && duel::negate_attack(d) && duel::change_position(d, tc, POS_FACEUP_DEFENSE as u8, None, None, None, false) != 0 {
        // Cannot change its battle position
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(3313)
        effect::set_description(d, e1, 3313);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CANNOT_CHANGE_POSITION)
        effect::set_code(d, e1, EFFECT_CANNOT_CHANGE_POSITION);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local val=tc:GetAttack()/2
        let val = lua_int(card::get_attack(d, tc) as f64 / 2.0);
        // local op=Duel.SelectOption(1-tp,aux.Stringid(id,0),aux.Stringid(id,1))
        let op = duel::select_option(d, 1 - tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)]);
        // if op==0 then Duel.Recover(1-tp,val,REASON_EFFECT)
        if op == 0 {
            duel::recover(d, 1 - tp, val, REASON_EFFECT, false, None);
        // else Duel.Damage(tp,val,REASON_EFFECT) end
        } else {
            duel::damage(d, tp, val, REASON_EFFECT, false, None);
        }
    }
}
