//! Magnet Warrior Sigma Minus
use super::prelude::*;

pub const ID: u32 = 87814728;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MAGNA_WARRIOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate an attack involving 2 EARTH monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.atknegcon)
    effect::set_condition(d, e1, Some(condition(atknegcon)));
    // e1:SetOperation(function() Duel.NegateAttack() end)
    effect::set_operation(d, e1, Some(operation(|d, _a| {
        duel::negate_attack(d);
    })));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Fusion Summon 1 Fusion Monster from your Extra Deck, using Rock monsters from your hand or field, including this card in your hand
    // local fusion_params={handler=c, matfilter=function(c) return c:IsRace(RACE_ROCK) end, gc=Fusion.ForcedHandler}
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.SelfReveal)
    effect::set_cost(d, e2, Some(cost::self_reveal()));
    // e2:SetTarget(Fusion.SummonEffTG(fusion_params))
    let tg = fusion::summon_eff_tg(d, fusion_params());
    effect::set_target(d, e2, Some(tg));
    // e2:SetOperation(Fusion.SummonEffOP(fusion_params))
    let op = fusion::summon_eff_op(d, fusion_params());
    effect::set_operation(d, e2, Some(op));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Send 1 Level 8 "Magna Warrior" monster from your Deck to your GY
    // local e3a=Effect.CreateEffect(c)
    let e3a = effect::create_effect(d, c);
    // e3a:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3a, aux::stringid(ID, 2));
    // e3a:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e3a, CATEGORY_TOGRAVE);
    // e3a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3a:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3a, EFFECT_FLAG_DELAY, 0);
    // e3a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3a, EVENT_SUMMON_SUCCESS);
    // e3a:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3a, 1, ID, 1, 0);
    // e3a:SetTarget(s.tgtg)
    effect::set_target(d, e3a, Some(cost_target(tgtg)));
    // e3a:SetOperation(s.tgop)
    effect::set_operation(d, e3a, Some(operation(tgop)));
    // c:RegisterEffect(e3a)
    card::register_effect(d, c, e3a, false);
    // local e3b=e3a:Clone()
    let e3b = effect::clone(d, e3a);
    // e3b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3b)
    card::register_effect(d, c, e3b, false);
}
/// `local fusion_params={handler=c, matfilter=function(c) return c:IsRace(RACE_ROCK) end, gc=Fusion.ForcedHandler}`
fn fusion_params() -> fusion::SummonEff {
    fusion::SummonEff {
        matfilter: Some(fusion::mat_filter_fn(|d, c, _e, _tp, _chk| card::is_race(d, c, RACE_ROCK))),
        gc: Some(fusion::forced_handler()),
        ..Default::default()
    }
}
/// `s.atknegcon(e,tp,eg,ep,ev,re,r,rp)`
fn atknegcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // local bc1=Duel.GetAttacker()
    let bc1 = duel::get_attacker(d).nil();
    // local bc2=Duel.GetAttackTarget()
    let bc2 = duel::get_attack_target(d);
    // return bc1:IsAttribute(ATTRIBUTE_EARTH) and bc2 and bc2:IsAttribute(ATTRIBUTE_EARTH) and bc2:IsFaceup()
    if !card::is_attribute(d, bc1, ATTRIBUTE_EARTH) {
        return false;
    }
    match bc2 {
        Some(bc2) => card::is_attribute(d, bc2, ATTRIBUTE_EARTH) && card::is_faceup(d, bc2),
        None => false,
    }
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevel(8) and c:IsSetCard(SET_MAGNA_WARRIOR) and c:IsAbleToGrave()
    card::get_level(d, c) == 8 && card::is_set_card(d, c, &[SET_MAGNA_WARRIOR as u16]) && card::is_able_to_grave(d, c, None)
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
