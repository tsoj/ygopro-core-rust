//! Dream Mirror of Joy
use super::prelude::*;

pub const ID: u32 = 74665651;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_DREAM_MIRROR_TERROR], listed_series: &[SET_DREAM_MIRROR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate 1 "Dream Mirror of Terror" from hand or deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.tftg)
    effect::set_target(d, e2, Some(cost_target(tftg)));
    // e2:SetOperation(s.tfop)
    effect::set_operation(d, e2, Some(operation(tfop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Opponent can only target your "Dream Mirror" monsters with highest level for attacks or card effects
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e3, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e3, 0, LOCATION_MZONE);
    // e3:SetCondition(s.limcond)
    effect::set_condition(d, e3, Some(condition(limcond)));
    // e3:SetValue(s.atlimit)
    effect::set_value_fn(d, e3, effect_card_fn(atlimit));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e4, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e4:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e4, LOCATION_MZONE, 0);
    // e4:SetTarget(s.atlimit)
    effect::set_target(d, e4, Some(effect_card_fn(atlimit)));
    // e4:SetValue(aux.tgoval)
    effect::set_value_fn(d, e4, aux::tgoval());
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsCode(CARD_DREAM_MIRROR_TERROR) and c:GetActivateEffect():IsActivatable(tp,true,true)
    if !card::is_code(d, c, &[CARD_DREAM_MIRROR_TERROR]) {
        return false;
    }
    let e = card::get_activate_effect(d, c).first().copied().nil();
    effect::is_activatable(d, e, tp, true, true)
}
/// `s.tftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK|LOCATION_HAND,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, a.tp)), a.tp, LOCATION_DECK | LOCATION_HAND, 0, 1, NO_EX);
    }
    // if not Duel.CheckPhaseActivity() then Duel.RegisterFlagEffect(tp,CARD_MAGICAL_MIDBREAKER,RESET_CHAIN,0,1) end
    if !duel::check_phase_activity(d) {
        duel::register_flag_effect(d, a.tp, CARD_MAGICAL_MIDBREAKER, RESET_CHAIN, 0, 1, 0);
    }
    true
}
/// `s.tfop(e,tp,eg,ep,ev,re,r,rp)`
fn tfop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK|LOCATION_HAND,0,1,1,nil,tp):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| filter(d, x, a.tp)), a.tp, LOCATION_DECK | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // Duel.ActivateFieldSpell(tc,e,tp,eg,ep,ev,re,r,rp)
    duel::activate_field_spell(d, tc, a, None);
}
/// `s.imfilter(c)`
fn imfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_DREAM_MIRROR) and c:IsAttribute(ATTRIBUTE_LIGHT)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DREAM_MIRROR as u16]) && card::is_attribute(d, c, ATTRIBUTE_LIGHT)
}
/// `s.limcond(e,tp,eg,ep,ev,re,r,rp)`
fn limcond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.imfilter,e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&imfilter), p, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.atfilter(c)`
fn atfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_DREAM_MIRROR)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DREAM_MIRROR as u16])
}
/// `s.atlimit(e,c)`
fn atlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local g=Duel.GetMatchingGroup(s.atfilter,e:GetHandlerPlayer(),LOCATION_MZONE,0,nil)
    let p = effect::get_handler_player(d, e);
    let g = duel::get_matching_group(d, Some(&atfilter), p, LOCATION_MZONE, 0, NO_EX);
    // return c:IsFaceup() and c:IsSetCard(SET_DREAM_MIRROR) and not g:GetMaxGroup(Card.GetLevel):IsContains(c)
    if !(card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DREAM_MIRROR as u16])) {
        return false;
    }
    let max = group::get_max_group(d, &g, &|d, x| card::get_level(d, x) as i64).map(|(m, _)| m).unwrap_or_default();
    !group::is_contains(&max, c)
}
