//! Megalith Phaleg
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 63233638;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_MEGALITH],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Ritual Summon 1 "Megalith" Ritual Monster from your hand, by Tributing monsters from your hand or field whose total Levels equal or exceed its Level
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_RELEASE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_RELEASE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e1, Some(cost::self_discard()));
    // e1:SetTarget(Ritual.Target(ritual_target_params))
    let target = ritual::target(d, target_params(c));
    effect::set_target(d, e1, Some(target));
    // e1:SetOperation(Ritual.Operation(ritual_operation_params))
    let operation_fn = ritual::operation(d, operation_params());
    effect::set_operation(d, e1, Some(operation_fn));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Monsters you control gain 300 ATK/DEF for each Ritual Monster in your GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetValue(function(e,c) return 300*Duel.GetMatchingGroupCount(Card.IsRitualMonster,e:GetHandlerPlayer(),LOCATION_GRAVE,0,nil) end)
    effect::set_value_fn(d, e2, effect_value(atk_value));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function(e,c) return 300*Duel.GetMatchingGroupCount(Card.IsRitualMonster,e:GetHandlerPlayer(),LOCATION_GRAVE,0,nil) end
fn atk_value(d: &mut Duel, e: EffectId) -> i64 {
    let p = effect::get_handler_player(d, e);
    300 * duel::get_matching_group_count(d, Some(&card::is_ritual_monster), p, LOCATION_GRAVE, 0, NO_EX) as i64
}
// local ritual_target_params={handler=c,lvtype=RITPROC_GREATER,filter=function(ritual_c) return ritual_c:IsSetCard(SET_MEGALITH) and ritual_c~=c end,forcedselection=s.forcedselection}
fn target_params(c: CardId) -> ritual::RitualProc {
    ritual::RitualProc {
        lvtype: RITPROC_GREATER,
        filter: Some(Arc::new(move |d: &mut Duel, x: CardId| card::is_set_card(d, x, &[SET_MEGALITH as u16]) && x != c)),
        forcedselection: Some(Arc::new(|d: &mut Duel, e: EffectId, _tp: u8, g: &CardSet, _sc: CardId| forcedselection(d, e, g))),
        ..Default::default()
    }
}
// local ritual_operation_params={handler=c,lvtype=RITPROC_GREATER,filter=function(ritual_c) return ritual_c:IsSetCard(SET_MEGALITH) end}
fn operation_params() -> ritual::RitualProc {
    ritual::RitualProc {
        lvtype: RITPROC_GREATER,
        filter: Some(Arc::new(|d: &mut Duel, x: CardId| card::is_set_card(d, x, &[SET_MEGALITH as u16]))),
        ..Default::default()
    }
}
// function s.forcedselection(e,tp,g,sc)
fn forcedselection(d: &mut Duel, e: EffectId, g: &CardSet) -> (bool, bool) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // return not g:IsContains(c),g:IsContains(c)
    (!g.contains(c), g.contains(c))
}
