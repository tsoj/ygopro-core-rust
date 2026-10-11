//! Mind Mirror Force
use super::prelude::*;

pub const ID: u32 = 66247039;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If your opponent controls 5 or more face-up cards, this card's activation and effect cannot be negated, also you can activate this card the turn it was Set
    // c:RegisterFlagEffect(id,0,0,1)
    card::register_flag_effect(d, c, ID, 0, 0, 1, 0, 0);
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, move |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD);
        // ge1:SetCode(EFFECT_CANNOT_INACTIVATE)
        effect::set_code(d, ge1, EFFECT_CANNOT_INACTIVATE);
        // ge1:SetValue(function(e,ch)
        //     local trig_eff=Chain.GetTriggeringEffect(ch)
        //     local trig_player=Chain.GetTriggeringPlayer(ch)
        //     return trig_eff:GetHandler():HasFlagEffect(id) and Duel.GetMatchingGroupCount(Card.IsFaceup,trig_player,0,LOCATION_ONFIELD,nil)>=5
        // end)
        effect::set_value_fn(d, ge1, lua_fn(value_inactivate));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
        // local ge2=ge1:Clone()
        let ge2 = effect::clone(d, ge1);
        // ge2:SetCode(EFFECT_CANNOT_DISEFFECT)
        effect::set_code(d, ge2, EFFECT_CANNOT_DISEFFECT);
        // Duel.RegisterEffect(ge2,0)
        duel::register_effect(d, ge2, 0);
    });
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e0, aux::stringid(ID, 0));
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e0, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e0:SetCode(EFFECT_TRAP_ACT_IN_SET_TURN)
    effect::set_code(d, e0, EFFECT_TRAP_ACT_IN_SET_TURN);
    // e0:SetCondition(function(e)
    //     return Duel.GetMatchingGroupCount(Card.IsFaceup,e:GetHandlerPlayer(),0,LOCATION_ONFIELD,nil)>=5
    // end)
    effect::set_condition(d, e0, Some(condition(e0_cond)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // --● When the monster your opponent controls with the highest ATK (even if tied) declares an attack
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1a, aux::stringid(ID, 1));
    // e1a:SetCategory(CATEGORY_DISABLE+CATEGORY_DESTROY)
    effect::set_category(d, e1a, CATEGORY_DISABLE + CATEGORY_DESTROY);
    // e1a:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1a, EFFECT_TYPE_ACTIVATE);
    // e1a:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1a, EVENT_ATTACK_ANNOUNCE);
    // e1a:SetCondition(s.condition1)
    effect::set_condition(d, e1a, Some(condition(condition1)));
    // e1a:SetTarget(s.target)
    effect::set_target(d, e1a, Some(cost_target(target)));
    // e1a:SetOperation(s.activate)
    effect::set_operation(d, e1a, Some(operation(activate)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // --● When your opponent activates a monster effect that would destroy a card(s) on the field
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1b, EVENT_CHAINING);
    // e1b:SetCondition(s.condition2)
    effect::set_condition(d, e1b, Some(condition(condition2)));
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // --● When your opponent activates a monster effect in the hand or field during your turn
    // local e1c=e1b:Clone()
    let e1c = effect::clone(d, e1b);
    // e1c:SetCondition(s.condition3)
    effect::set_condition(d, e1c, Some(condition(condition3)));
    // c:RegisterEffect(e1c)
    card::register_effect(d, c, e1c, false);
}
/// `ge1:SetValue(function(e,ch) ... end)`
fn value_inactivate(d: &mut Duel, a: Args) -> bool {
    // local trig_eff=Chain.GetTriggeringEffect(ch)
    let ch = a.int(1) as u8;
    let trig_eff = duel::get_chain_info(d, ch, &[CHAININFO_TRIGGERING_EFFECT])[0].as_effect().nil();
    // local trig_player=Chain.GetTriggeringPlayer(ch)
    let trig_player = duel::get_chain_info(d, ch, &[CHAININFO_TRIGGERING_PLAYER])[0].int() as u8;
    // return trig_eff:GetHandler():HasFlagEffect(id) and Duel.GetMatchingGroupCount(Card.IsFaceup,trig_player,0,LOCATION_ONFIELD,nil)>=5
    let h = effect::get_handler(d, trig_eff).nil();
    card::has_flag_effect(d, h, ID, None)
        && duel::get_matching_group_count(d, Some(&|d, c| card::is_faceup(d, c)), trig_player, 0, LOCATION_ONFIELD, NO_EX) >= 5
}
/// `e0:SetCondition(function(e) ... end)`
fn e0_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    duel::get_matching_group_count(d, Some(&|d, c| card::is_faceup(d, c)), p, 0, LOCATION_ONFIELD, NO_EX) >= 5
}
/// `s.condition1(e,tp,eg,ep,ev,re,r,rp)`
fn cond1_inner(d: &mut Duel, tp: u8) -> bool {
    // local bc=Duel.GetAttacker()
    let bc = duel::get_attacker(d).nil();
    // return bc:IsControler(1-tp) and Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil):GetMaxGroup(Card.GetAttack):IsContains(bc)
    if !card::is_controler(d, bc, 1 - tp) {
        return false;
    }
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
    match group::get_max_group(d, &g, &|d, c| card::get_attack(d, c) as i64) {
        Some((max, _)) => max.contains(bc),
        None => false,
    }
}
fn condition1(d: &mut Duel, a: &EffArgs) -> bool {
    cond1_inner(d, a.tp)
}
/// `s.condition2(e,tp,eg,ep,ev,re,r,rp)`
fn condition2(d: &mut Duel, a: &EffArgs) -> bool {
    // Avoid prompting which effect to activate if both 'condition1' and 'condition2' are true
    // local event_chk,event_g,event_p,event_v,event_reff,event_r,event_rp=Duel.CheckEvent(EVENT_ATTACK_ANNOUNCE,true)
    // if event_chk and s.condition1(e,tp,event_g,event_p,event_v,event_reff,event_r,event_rp) then return false end
    if duel::check_event(d, EVENT_ATTACK_ANNOUNCE).is_some() && cond1_inner(d, a.tp) {
        return false;
    }
    // if not (rp==1-tp and re:IsMonsterEffect()) then return false end
    if !(a.rp == 1 - a.tp && effect::is_monster_effect(d, a.re.nil())) {
        return false;
    }
    // local opinfo_chk,opinfo_g=Duel.GetOperationInfo(ev,CATEGORY_DESTROY)
    // return opinfo_chk and opinfo_g and opinfo_g:IsExists(Card.IsOnField,1,nil)
    match duel::get_operation_info(d, a.ev as u32, CATEGORY_DESTROY) {
        Some(op) => match op.op_cards {
            Some(g) => group::is_exists(d, &g, &|d, c| card::is_on_field(d, c), 1, NO_EX),
            None => false,
        },
        None => false,
    }
}
/// `s.condition3(e,tp,eg,ep,ev,re,r,rp)`
fn condition3(d: &mut Duel, a: &EffArgs) -> bool {
    // Avoid prompting which effect to activate if both 'condition2' and 'condition3' are true
    // if s.condition2(e,tp,eg,ep,ev,re,r,rp) then return false end
    if condition2(d, a) {
        return false;
    }
    // return Duel.IsTurnPlayer(tp) and rp==1-tp and re:IsMonsterEffect() and Chain.IsTriggeringLocation(ev,LOCATION_HAND|LOCATION_MZONE)
    duel::is_turn_player(d, a.tp)
        && a.rp == 1 - a.tp
        && effect::is_monster_effect(d, a.re.nil())
        && chain::is_triggering(d, a.ev as u8, chain::Prop::Location, &[(LOCATION_HAND | LOCATION_MZONE) as i64]).truthy()
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(Card.IsNegatable,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_negatable(d, c)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    let n = g.len() as u8;
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,#g,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(g.clone()), n, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, a.tp, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(Card.IsNegatable,tp,0,LOCATION_ONFIELD,nil):Match(Card.IsCanBeDisabledByEffect,nil,e)
    let e = a.e;
    let mut g = duel::get_matching_group(d, Some(&|d, x| card::is_negatable(d, x)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    group::match_(d, &mut g, &|d, x| card::is_can_be_disabled_by_effect(d, x, e, true), NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // for nc in g:Iter() do
        //     --Negate the effects of as many face-up cards your opponent controls as possible, and if you do, destroy them
        //     nc:NegateEffects(c)
        // end
        for nc in group::iter(d, &g) {
            card::negate_effects(d, nc, c, None, false, None);
        }
        // Duel.AdjustInstantly()
        duel::adjust_instantly(d, None);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
    // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,2),nil,2)
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 2)), 0, Some(2));
    // Your monsters cannot attack directly until the end of the next turn after this card resolves
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END,2)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 2);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
