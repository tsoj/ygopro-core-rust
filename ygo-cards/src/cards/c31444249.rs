//! Void Imagination
//! 煉獄の虚夢
use super::prelude::*;

pub const ID: u32 = 31444249;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_INFERNOID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Change level
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CHANGE_LEVEL)
    effect::set_code(d, e2, EFFECT_CHANGE_LEVEL);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // e2:SetTarget(s.lvtg)
    effect::set_target(d, e2, Some(effect_card_fn(lvtg)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --Reduce battle damage
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e3, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e3:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTarget(s.rdtg)
    effect::set_target(d, e3, Some(effect_card_fn(rdtg)));
    // e3:SetValue(aux.ChangeBattleDamage(1,HALF_DAMAGE))
    effect::set_value_fn(d, e3, aux::change_battle_damage(1, HALF_DAMAGE as i64));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // --Fusion Summon
    // local params = {fusfilter=aux.FilterBoolFunction(Card.IsSetCard,SET_INFERNOID),extrafil=s.fextra,extratg=s.extratg}
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCost(Cost.SelfToGrave)
    effect::set_cost(d, e4, Some(cost::self_to_grave()));
    // e4:SetTarget(Fusion.SummonEffTG(params))
    let tg = fusion::summon_eff_tg(d, fus_params());
    effect::set_target(d, e4, Some(tg));
    // e4:SetOperation(Fusion.SummonEffOP(params))
    let op = fusion::summon_eff_op(d, fus_params());
    effect::set_operation(d, e4, Some(op));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `params` of the Fusion Summon effect (`fusfilter`, `extrafil`, `extratg`)
fn fus_params() -> fusion::SummonEff {
    fusion::SummonEff {
        // fusfilter=aux.FilterBoolFunction(Card.IsSetCard,SET_INFERNOID)
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_INFERNOID as u16]))),
        // extrafil=s.fextra
        extrafil: Some(std::sync::Arc::new(fextra)),
        // extratg=s.extratg
        extratg: Some(cost_target(extratg)),
        ..Default::default()
    }
}
/// `s.lvtg(e,c)`
fn lvtg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsSetCard(SET_INFERNOID) and c:GetOriginalLevel()>=2
    card::is_set_card(d, c, &[SET_INFERNOID as u16]) && card::get_original_level(d, c) >= 2
}
/// `s.rdtg(e,c)`
fn rdtg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsSetCard(SET_INFERNOID) and c:GetOriginalLevel()>=2
    card::is_set_card(d, c, &[SET_INFERNOID as u16]) && card::get_original_level(d, c) >= 2
}
/// `s.fcheck(tp,sg,fc)`
fn fcheck_body(d: &mut Duel, a: Args) -> bool {
    // return sg:FilterCount(Card.IsLocation,nil,LOCATION_DECK)<=6
    let sg = a.g(1);
    group::filter_count(d, &sg, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX) <= 6
}
/// `s.dmcon(tp)`
fn dmcon(d: &mut Duel, tp: u8) -> bool {
    // return not Duel.IsExistingMatchingCard(Card.IsSummonLocation,tp,LOCATION_MZONE,0,1,nil,LOCATION_EXTRA)
    //   and Duel.IsExistingMatchingCard(Card.IsSummonLocation,tp,0,LOCATION_MZONE,1,nil,LOCATION_EXTRA)
    !duel::is_existing_matching_card(d, Some(&|d, c| card::is_summon_location(d, c, LOCATION_EXTRA)), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_summon_location(d, c, LOCATION_EXTRA)), tp, 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // if s.dmcon(tp) then
    if dmcon(d, tp) {
        // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToGrave),tp,LOCATION_DECK,0,nil),s.fcheck
        let g = duel::get_matching_group(
            d,
            Some(&|d, c| card::is_monster(d, c) && card::is_able_to_grave(d, c, None)),
            tp,
            LOCATION_DECK,
            0,
            NO_EX,
        );
        return (Some(g), Some(lua_fn(fcheck_body)));
    }
    // return nil
    (None, None)
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
