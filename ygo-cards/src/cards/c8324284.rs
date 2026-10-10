//! Spirit Shield
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 8324284;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // Negate an opponent's attack
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_condition(d, e1, Some(condition(atk_cond)));
    effect::set_cost(d, e1, Some(cost_target(negatkcost)));
    effect::set_target(d, e1, Some(cost_target(negatktg)));
    effect::set_operation(d, e1, Some(operation(atk_op)));
    card::register_effect(d, c, e1, false);
    // Negate the activation of an opponent's effect that would destroy a card(s)
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_NEGATE);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(eff_cond)));
    effect::set_cost(d, e2, Some(cost_target(negeffcost)));
    effect::set_target(d, e2, Some(cost_target(negefftg)));
    effect::set_operation(d, e2, Some(operation(eff_op)));
    card::register_effect(d, c, e2, false);
    // Send this card to the GY
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_TOGRAVE);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(tg_cond)));
    effect::set_target(d, e3, Some(cost_target(tgtg)));
    effect::set_operation(d, e3, Some(operation(tg_op)));
    card::register_effect(d, c, e3, false);
}
// function s.negcostfilter(c)
fn negcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_FIEND|RACE_ZOMBIE) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_race(d, c, RACE_FIEND | RACE_ZOMBIE) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
// function(e,tp) return Duel.GetAttacker():IsControler(1-tp) end
fn atk_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let atk = duel::get_attacker(d).nil();
    card::is_controler(d, atk, 1 - a.tp)
}
// function() Duel.NegateAttack() end
fn atk_op(d: &mut Duel, _a: &EffArgs) {
    duel::negate_attack(d);
}
// function s.negatkcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn negatkcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        // return not c:HasFlagEffect(id) and Duel.IsExistingMatchingCard(s.negcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil)
        return !card::has_flag_effect(d, c, ID, None)
            && duel::is_existing_matching_card(d, Some(&negcostfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE,0,1)
    card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 0, 1, 0, 0);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.negcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&negcostfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
// function s.negatktg(e,tp,eg,ep,ev,re,r,rp,chk)
fn negatktg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // local bc=Duel.GetAttacker()
        // return bc:IsOnField() and bc:IsRelateToBattle()
        let bc = duel::get_attacker(d).nil();
        return card::is_on_field(d, bc) && card::is_relate_to_battle(d, bc);
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    true
}
// function s.negeffcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn negeffcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // return Duel.IsExistingMatchingCard(s.negcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil)
        return duel::is_existing_matching_card(d, Some(&negcostfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.negcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&negcostfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
// function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp and Duel.GetOperationInfo(ev,CATEGORY_DESTROY) and Duel.IsChainNegatable(ev) end
fn eff_cond(d: &mut Duel, a: &EffArgs) -> bool {
    a.rp == 1 - a.tp && duel::get_operation_info(d, a.ev, CATEGORY_DESTROY).is_some() && duel::is_chain_negatable(d, a.ev as u8)
}
// function s.negefftg(e,tp,eg,ep,ev,re,r,rp,chk)
fn negefftg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, a.tp, 0);
    true
}
// function(e,tp,eg,ep,ev,re,r,rp) Duel.NegateActivation(ev) end
fn eff_op(d: &mut Duel, a: &EffArgs) {
    duel::negate_activation(d, a.ev as u8);
}
// function(e,tp) return not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_FIEND|RACE_ZOMBIE),tp,LOCATION_MZONE,0,1,nil) end
fn tg_cond(d: &mut Duel, a: &EffArgs) -> bool {
    !duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_FIEND | RACE_ZOMBIE)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
// function s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,e:GetHandler(),1,tp,0)
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
// function(e) Duel.SendtoGrave(e:GetHandler(),REASON_EFFECT) end
fn tg_op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    duel::send_to_grave(d, c, REASON_EFFECT, None, None);
}
