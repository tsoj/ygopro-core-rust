//! Faisan, Hunting Scout of the Deep Forest
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 86038337;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 2 EARTH Warrior monsters
    // Fusion.AddProcMixN(c,true,true,s.matfilter,2)
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat(matfilter)), 2)]);
    // Unaffected by your opponent's activated effects, during the Main Phase only
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(main_phase_cond)));
    // e1:SetValue(function(e,te) return te:GetOwnerPlayer()~=e:GetHandlerPlayer() and te:IsActivated() end)
    effect::set_value_fn(d, e1, lua_fn(immval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 2 EARTH Warrior monsters from your GY, except "Faisan, Hunting Scout of the Deep Forest", and if you do,
    // your EARTH Warrior monsters cannot be destroyed by battle for the rest of this turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e) return Duel.IsBattlePhase() and e:GetHandler():IsFusionSummoned() end)
    effect::set_condition(d, e2, Some(condition(spcond)));
    // e2:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e2, Some(cost::self_tribute()));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // e2:SetHintTiming(TIMING_BATTLE_PHASE|TIMING_BATTLE_END,TIMING_BATTLE_START|TIMING_BATTLE_PHASE|TIMING_BATTLE_STEP_END|TIMING_BATTLE_END)
    effect::set_hint_timing(
        d,
        e2,
        TIMING_BATTLE_PHASE | TIMING_BATTLE_END,
        Some(TIMING_BATTLE_START | TIMING_BATTLE_PHASE | TIMING_BATTLE_STEP_END | TIMING_BATTLE_END),
    );
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.matfilter(c,fc,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_EARTH,fc,sumtype,tp) and c:IsRace(RACE_WARRIOR,fc,sumtype,tp)
    card::is_attribute_ctx(d, c, ATTRIBUTE_EARTH, Some(fc), sumtype, Some(tp))
        && card::is_race_ctx(d, c, RACE_WARRIOR, Some(fc), sumtype, Some(tp))
}
/// `function() return Duel.IsMainPhase() end`
fn main_phase_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_main_phase(d, None)
}
/// `s.immval(e,te)`: `EFFECT_IMMUNE_EFFECT`'s value
fn immval(d: &mut Duel, a: Args) -> bool {
    // return te:GetOwnerPlayer()~=e:GetHandlerPlayer() and te:IsActivated()
    let e = a.e(0);
    let te = a.e(1);
    effect::get_owner_player(d, te) != effect::get_handler_player(d, e) && effect::is_activated(d, te)
}
/// `function(e) return Duel.IsBattlePhase() and e:GetHandler():IsFusionSummoned() end`
fn spcond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_battle_phase(d, None) && card::is_fusion_summoned(d, effect::get_handler(d, a.e).nil())
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_EARTH) and c:IsRace(RACE_WARRIOR) and not c:IsCode(id)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_attribute(d, c, ATTRIBUTE_EARTH)
        && card::is_race(d, c, RACE_WARRIOR)
        && !card::is_code(d, c, &[ID])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetMZoneCount(tp,e:GetHandler())>=2
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,2,nil,e,tp) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::get_m_zone_count(d, a.tp, h.into(), None, None, None).0 >= 2
            && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::is_existing_matching_card(d, Some(&|d: &mut Duel, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 2, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetMZoneCount(tp)<2 or Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::get_m_zone_count(d, a.tp, NO_EX, None, None, None).0 < 2 || duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_GRAVE,0,2,2,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d: &mut Duel, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 2, 2, false, NO_EX).nil();
    // if #g==2 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)==2 then
    if g.len() == 2 && duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) == 2 {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // aux.RegisterClientHint(c,0,tp,1,0,aux.Stringid(id,1))
        aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
        // Your EARTH Warrior monsters cannot be destroyed by battle for the rest of this turn
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
        // e1:SetTarget(function(e,c) return c:IsAttribute(ATTRIBUTE_EARTH) and c:IsRace(RACE_WARRIOR) end)
        effect::set_target(d, e1, Some(effect_card_fn(|d: &mut Duel, _e, x| card::is_attribute(d, x, ATTRIBUTE_EARTH) && card::is_race(d, x, RACE_WARRIOR))));
        // e1:SetTargetRange(LOCATION_MZONE,0)
        effect::set_target_range(d, e1, LOCATION_MZONE, 0);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
