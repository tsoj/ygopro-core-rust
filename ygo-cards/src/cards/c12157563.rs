//! Forbidden Ritual Art
use super::prelude::*;

pub const ID: u32 = 12157563;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual Summon 1 Ritual Monster from your GY, by banishing monsters from your GY whose total Levels equal its Level,
    // but destroy it during the End Phase
    // local e1=Ritual.AddProcEqual({handler=c,location=LOCATION_GRAVE,matfilter=function(c) return c:IsLocation(LOCATION_GRAVE) end,
    //   extrafil=s.rextra,extratg=s.extratg,stage2=s.stage2})
    let e1 = ritual::add_proc_equal(
        d,
        c,
        ritual::RitualProc {
            location: Some(LOCATION_GRAVE),
            matfilter: Some(ritual::MatFilt::F(std::sync::Arc::new(|d: &mut Duel, c: CardId, _e: EffectId, _tp: u8| {
                card::is_location(d, c, LOCATION_GRAVE)
            }))),
            extrafil: Some(std::sync::Arc::new(rextra)),
            extratg: Some(cost_target(extratg)),
            stage2: Some(std::sync::Arc::new(stage2)),
            ..Default::default()
        },
    );
    // e1:SetCategory(e1:GetCategory()+CATEGORY_REMOVE)
    let cat = effect::get_category(d, e1);
    effect::set_category(d, e1, cat + CATEGORY_REMOVE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Apply an "once this turn, when you Ritual Summon with a card effect that requires use of monster Tributes,
    // you can also banish Ritual Monsters from your GY as monsters used for the Ritual Summon" effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.rextra(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rextra(d: &mut Duel, a: &EffArgs) -> Option<CardSet> {
    // if not Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) then
    if duel::is_player_affected_by_effect(d, a.tp, CARD_SPIRIT_ELIMINATION).is_none() {
        // return Duel.GetMatchingGroup(aux.AND(Card.HasLevel,Card.IsAbleToRemove),tp,LOCATION_GRAVE,0,nil)
        return Some(duel::get_matching_group(d, Some(&|d, c| card::has_level(d, c) && card::is_able_to_remove(d, c, None, None, None)), a.tp, LOCATION_GRAVE, 0, NO_EX));
    }
    None
}
/// `s.extratg(e,tp,...,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.stage2(mat,e,tp,eg,ep,ev,re,r,rp,rc)`
fn stage2(d: &mut Duel, _mat: &CardSet, a: &EffArgs, rc: CardId) {
    // Destroy it during the End Phase
    // aux.DelayedOperation(rc,PHASE_END,id,e,tp,function(ag) Duel.Destroy(ag,REASON_EFFECT) end,nil,0,0,aux.Stringid(id,2))
    aux::delayed_operation(d, rc, PHASE_END, ID, a.e, a.tp, Some(std::sync::Arc::new(destroy_oper)), None, Some(0), Some(0), Some(aux::stringid(ID, 2)), None);
}
/// `function(ag) Duel.Destroy(ag,REASON_EFFECT) end`
fn destroy_oper(d: &mut Duel, ag: &CardSet, _a: &EffArgs) -> bool {
    duel::destroy(d, ag.clone(), REASON_EFFECT, None, None);
    true
}
/// `s.effop(e,tp,...)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // aux.RegisterClientHint(c,0,tp,1,0,aux.Stringid(id,3))
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 3)), 0, None);
    // Once this turn, when you Ritual Summon with a card effect that requires use of monster Tributes, you can also
    // banish Ritual Monsters from your GY as monsters used for the Ritual Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_EXTRA_RITUAL_MATERIAL)
    effect::set_code(d, e1, EFFECT_EXTRA_RITUAL_MATERIAL);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTargetRange(LOCATION_GRAVE,0)
    effect::set_target_range(d, e1, LOCATION_GRAVE, 0);
    // e1:SetCondition(function(e) return not Duel.HasFlagEffect(tp,id) end)
    let tp = a.tp;
    effect::set_condition(d, e1, Some(lua_fn(move |d: &mut Duel, _a: Args| duel::get_flag_effect(d, tp, ID) == 0)));
    // e1:SetTarget(function(e,c) return c:IsRitualMonster() and c:IsAbleToRemove() end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_ritual_monster(d, c) && card::is_able_to_remove(d, c, None, None, None))));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
