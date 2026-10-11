//! Clear Vice Dragon
use super::prelude::*;

pub const ID: u32 = 97811903;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_CLEAR_WORLD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You are unaffected by the effects of "Clear World"
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CLEAR_WORLD_IMMUNE)
    effect::set_code(d, e1, EFFECT_CLEAR_WORLD_IMMUNE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card attacks an opponent's monster, the ATK of this card becomes twice the ATK of the attack target, during damage calculation only
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_SET_ATTACK_FINAL)
    effect::set_code(d, e2, EFFECT_SET_ATTACK_FINAL);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetValue(function(e,c) return Duel.GetAttackTarget():GetAttack()*2 end)
    effect::set_value_fn(d, e2, effect_card_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // If this card would be destroyed by an opponent's card effect, you can discard 1 card instead
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e3, EFFECT_DESTROY_REPLACE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTarget(s.reptg)
    effect::set_target(d, e3, Some(cost_target(reptg)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=Duel.GetAttackTarget()
    let bc = duel::get_attack_target(d);
    // return Duel.IsPhase(PHASE_DAMAGE_CAL) and e:GetHandler()==Duel.GetAttacker() and bc and bc:IsControler(1-e:GetHandlerPlayer())
    if !duel::is_phase(d, PHASE_DAMAGE_CAL) || effect::get_handler(d, a.e) != duel::get_attacker(d) {
        return false;
    }
    match bc {
        Some(bc) => {
            let p = 1 - effect::get_handler_player(d, a.e);
            card::is_controler(d, bc, p)
        }
        None => false,
    }
}
/// `function(e,c) return Duel.GetAttackTarget():GetAttack()*2 end`
fn atkval(d: &mut Duel, _e: EffectId, _c: CardId) -> i64 {
    let at = duel::get_attack_target(d).nil();
    card::get_attack(d, at) as i64 * 2
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:IsReason(REASON_REPLACE) and c:IsReason(REASON_EFFECT) and c:IsReasonPlayer(1-tp)
    //     and Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil,REASON_EFFECT) end
    if a.chk == Some(0) {
        return !card::is_reason(d, c, REASON_REPLACE)
            && card::is_reason(d, c, REASON_EFFECT)
            && card::is_reason_player(d, c, 1 - tp)
            && duel::is_existing_matching_card(d, Some(&|d, x| card::is_discardable(d, x, Some(REASON_EFFECT), None)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // return Duel.SelectEffectYesNo(tp,c,96) and Duel.DiscardHand(tp,nil,1,1,REASON_EFFECT|REASON_DISCARD)>0
    duel::select_effect_yes_no(d, tp, c, Some(96))
        && duel::discard_hand(d, tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX) > 0
}
