//! Glacier Aqua Madoor
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 11449436;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Battle protection
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_HANDES);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e1, EVENT_BATTLE_START);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.bpcon)
    effect::set_condition(d, e1, Some(condition(bpcon)));
    // e1:SetTarget(s.bptg)
    effect::set_target(d, e1, Some(cost_target(bptg)));
    // e1:SetOperation(s.bpop)
    effect::set_operation(d, e1, Some(operation(bpop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_HANDES+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_HANDES + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e2, EVENT_BATTLE_START);
    // e2:SetCost(s.descost)
    effect::set_cost(d, e2, Some(cost_target(descost)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.bpcon(e,tp,...)`
fn bpcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local a=Duel.GetAttacker()
    let a0 = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let d0 = duel::get_attack_target(d);
    // if not d then return false end
    let Some(d0) = d0 else { return false };
    // if a:IsControler(1-tp) then a,d=d,a end
    let (mut at, mut dt) = (a0.nil(), d0);
    if card::is_controler(d, at, 1 - tp) {
        (at, dt) = (dt, at);
    }
    // return a:IsFaceup() and a:IsType(TYPE_NORMAL) and d:IsControler(1-tp)
    card::is_faceup(d, at) && card::is_type(d, at, TYPE_NORMAL) && card::is_controler(d, dt, 1 - tp)
}
/// `s.bptg(e,tp,...,chk)`
fn bptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return not e:GetHandler():IsPublic()
    //     and Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil,REASON_EFFECT) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return !card::is_public(d, h)
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, Some(REASON_EFFECT), None)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, tp, 1);
    true
}
/// `s.bpop(e,tp,...)`
fn bpop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local a=Duel.GetAttacker()
    let a0 = duel::get_attacker(d).nil();
    // local d=Duel.GetAttackTarget()
    let d0 = duel::get_attack_target(d);
    // if a:IsControler(1-tp) then a,d=d,a end
    let a1 = if card::is_controler(d, a0, 1 - tp) { d0 } else { Some(a0) };
    // if Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_EFFECT|REASON_DISCARD,nil,REASON_EFFECT)>0
    //     and a:IsRelateToBattle() and a:IsControler(tp) then
    if duel::discard_hand(d, tp, Some(&|d, c| card::is_discardable(d, c, Some(REASON_EFFECT), None)), 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX) > 0 {
        let at = a1.nil();
        if card::is_relate_to_battle(d, at) && card::is_controler(d, at, tp) {
            // Cannot be destroyed by that battle
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
            effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
            // e1:SetValue(1)
            effect::set_value(d, e1, 1);
            // e1:SetReset(RESET_PHASE|PHASE_DAMAGE)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE, 1);
            // a:RegisterEffect(e1)
            card::register_effect(d, at, e1, false);
        }
    }
}
/// `s.costfilter(c)`
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_NORMAL) and not c:IsPublic()
    card::is_type(d, c, TYPE_NORMAL) && !card::is_public(d, c)
}
/// `s.descost(e,tp,...,chk)`
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&costfilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, g);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    true
}
/// `s.destg(e,tp,...,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e);
    // local tc=Duel.GetAttacker()
    let mut tc = duel::get_attacker(d);
    // if tc==c then tc=Duel.GetAttackTarget() end
    if tc == c {
        tc = duel::get_attack_target(d);
    }
    // if chk==0 then return tc and tc:IsControler(1-tp)
    //     and Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil,REASON_EFFECT) end
    if a.chk == Some(0) {
        let ok = match tc {
            Some(t) => card::is_controler(d, t, 1 - tp),
            None => false,
        };
        return ok && duel::is_existing_matching_card(d, Some(&|d, x| card::is_discardable(d, x, Some(REASON_EFFECT), None)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, tc.map(CardSet::from_card), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, tp, 1);
    true
}
/// `s.desop(e,tp,...)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e);
    // local tc=Duel.GetAttacker()
    let mut tc = duel::get_attacker(d);
    // if tc==c then tc=Duel.GetAttackTarget() end
    if tc == c {
        tc = duel::get_attack_target(d);
    }
    // if Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_EFFECT|REASON_DISCARD,nil,REASON_EFFECT)>0
    //     and tc:IsRelateToBattle() and tc:IsControler(1-tp) then
    if duel::discard_hand(d, tp, Some(&|d, x| card::is_discardable(d, x, Some(REASON_EFFECT), None)), 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX) > 0 {
        let t = tc.nil();
        if card::is_relate_to_battle(d, t) && card::is_controler(d, t, 1 - tp) {
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, t, REASON_EFFECT, None, None);
        }
    }
}
