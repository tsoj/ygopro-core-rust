//! Depth Amulet
use super::prelude::*;

pub const ID: u32 = 8279188;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target1)
    effect::set_target(d, e1, Some(cost_target(target1)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // negate
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetTarget(s.target2)
    effect::set_target(d, e2, Some(cost_target(target2)));
    // e2:SetOperation(s.activate)
    effect::set_operation(d, e2, Some(operation(activate)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp)
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.target1(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target1(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local tg=Duel.GetAttacker()
    let tg = duel::get_attacker(d);
    // if chkc then return chkc==tg end
    if let Some(chkc) = a.chkc() {
        return Some(chkc) == tg;
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // e:GetHandler():SetTurnCounter(0)
    let h = effect::get_handler(d, e).nil();
    card::set_turn_counter(d, h, 0);
    // destroy
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCondition(s.descon)
    effect::set_condition(d, e1, Some(condition(descon)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // e1:SetReset(RESETS_STANDARD_PHASE_END|RESET_OPPO_TURN,3)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END | RESET_OPPO_TURN, 3);
    // e:GetHandler():RegisterEffect(e1)
    card::register_effect(d, h, e1, false);
    // if Duel.CheckEvent(EVENT_ATTACK_ANNOUNCE) and Duel.IsTurnPlayer(1-tp) and tg:IsOnField() and tg:IsCanBeEffectTarget(e)
    //     and Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)~=0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    let selected = duel::check_event(d, EVENT_ATTACK_ANNOUNCE).is_some()
        && duel::is_turn_player(d, 1 - tp)
        && card::is_on_field(d, tg.nil())
        && card::is_can_be_effect_target(d, tg.nil(), Some(e))
        && duel::get_field_group_count(d, tp, LOCATION_HAND, 0) != 0
        && duel::select_yes_no(d, tp, aux::stringid(ID, 0));
    if selected {
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, e, EFFECT_FLAG_CARD_TARGET, 0);
        // Duel.SetTargetCard(tg)
        duel::set_target_card(d, CardSet::from_card(tg.nil()));
    } else {
        // e:SetProperty(0)
        effect::set_property(d, e, 0, 0);
    }
    true
}
/// `s.target2(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local tg=Duel.GetAttacker()
    let tg = duel::get_attacker(d);
    // if chkc then return chkc==tg end
    if let Some(chkc) = a.chkc() {
        return Some(chkc) == tg;
    }
    // if chk==0 then return not e:GetHandler():IsStatus(STATUS_CHAINING)
    //     and Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)~=0 and tg:IsOnField() and tg:IsCanBeEffectTarget(e) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return !card::is_status(d, h, STATUS_CHAINING)
            && duel::get_field_group_count(d, tp, LOCATION_HAND, 0) != 0
            && card::is_on_field(d, tg.nil())
            && card::is_can_be_effect_target(d, tg.nil(), Some(a.e));
    }
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, CardSet::from_card(tg.nil()));
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, h, e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and tc:IsFaceup() and Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)~=0 then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc) && duel::get_field_group_count(d, tp, LOCATION_HAND, 0) != 0 {
            // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_EFFECT|REASON_DISCARD)
            duel::discard_hand(d, tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
            // Duel.NegateAttack()
            duel::negate_attack(d);
        }
    }
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp)
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=c:GetTurnCounter()
    let mut ct = card::get_turn_counter(d, c);
    // ct=ct+1
    ct += 1;
    // c:SetTurnCounter(ct)
    card::set_turn_counter(d, c, ct as u16);
    // if ct==3 then
    if ct == 3 {
        // Duel.Destroy(c,REASON_EFFECT)
        duel::destroy(d, CardSet::from_card(c), REASON_EFFECT, None, None);
    }
}
