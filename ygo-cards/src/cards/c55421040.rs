//! Hunting Horn
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 55421040;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion Summon 1 EARTH Warrior Fusion Monster from your Extra Deck, using Warrior monsters from your hand or field,
    // then if you activated this card during the Battle Phase, you can choose monsters your opponent controls, up to the number
    // of materials used from the hand for this Fusion Summon, and their ATK become halved until the end of the Battle Phase
    // local e1=Fusion.CreateSummonEff({handler=c,fusfilter=...,matfilter=...,stage2=s.stage2})
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d: &mut Duel, x, _tp| card::is_attribute(d, x, ATTRIBUTE_EARTH) && card::is_race(d, x, RACE_WARRIOR))),
            matfilter: Some(fusion::mat_filter_fn(|d: &mut Duel, x, _e, _tp, _chk| card::is_race(d, x, RACE_WARRIOR))),
            stage2: Some(std::sync::Arc::new(stage2)),
            ..Default::default()
        },
    );
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetHintTiming(TIMING_BATTLE_PHASE,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMING_BATTLE_START|TIMING_BATTLE_PHASE|TIMING_BATTLE_STEP_END|TIMING_BATTLE_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(
        d,
        e1,
        TIMING_BATTLE_PHASE,
        Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMING_BATTLE_START | TIMING_BATTLE_PHASE | TIMING_BATTLE_STEP_END | TIMING_BATTLE_END | TIMINGS_CHECK_MONSTER_E),
    );
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_ATTACK,function(c) return c:IsAttribute(ATTRIBUTE_EARTH) and c:IsRace(RACE_WARRIOR) end)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_ATTACK, lua_fn(activity_filter));
}
/// `function(c) return c:IsAttribute(ATTRIBUTE_EARTH) and c:IsRace(RACE_WARRIOR) end` (the activity counter)
fn activity_filter(d: &mut Duel, a: Args) -> bool {
    let c = a.c(0);
    card::is_attribute(d, c, ATTRIBUTE_EARTH) && card::is_race(d, c, RACE_WARRIOR)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetCustomActivityCount(id,tp,ACTIVITY_ATTACK)==0 end
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_ATTACK) == 0;
    }
    // e:SetLabel(Duel.IsBattlePhase() and 1 or 0)
    let label: i64 = if duel::is_battle_phase(d, None) { 1 } else { 0 };
    effect::set_label(d, a.e, &[label]);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // aux.RegisterClientHint(c,EFFECT_FLAG_OATH,tp,1,0,aux.Stringid(id,1))
    aux::register_client_hint(d, c, EFFECT_FLAG_OATH, a.tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
    // You cannot declare attacks the turn you activate this card, except with EARTH Warrior monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_OATH, 0);
    // e1:SetCode(EFFECT_CANNOT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(function(e,c) return not (c:IsAttribute(ATTRIBUTE_EARTH) and c:IsRace(RACE_WARRIOR)) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d: &mut Duel, _e, x| !(card::is_attribute(d, x, ATTRIBUTE_EARTH) && card::is_race(d, x, RACE_WARRIOR)))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    true
}
/// `s.stage2(e,fc,tp,sg,chk)`
fn stage2(d: &mut Duel, e: EffectId, _fc: Option<CardId>, tp: u8, sg: Option<&CardSet>, chk: i64) {
    // if chk==1 and e:IsHasType(EFFECT_TYPE_ACTIVATE) and e:GetLabel()==1 then
    if chk == 1 && effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) && effect::get_label(d, e) == 1 {
        let sg = sg.cloned().nil();
        // local ct=sg:FilterCount(Card.IsPreviousLocation,nil,LOCATION_HAND)
        let ct = group::filter_count(d, &sg, &|d: &mut Duel, x| card::is_previous_location(d, x, LOCATION_HAND), NO_EX);
        // if ct>0 and Duel.IsExistingMatchingCard(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if ct > 0
            && duel::is_existing_matching_card(d, Some(&|d: &mut Duel, x| card::is_faceup(d, x)), tp, 0, LOCATION_MZONE, 1, NO_EX)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATKDEF as u64);
            // local g=Duel.SelectMatchingCard(tp,Card.IsFaceup,tp,0,LOCATION_MZONE,1,ct,nil)
            let g = duel::select_matching_card(d, tp, Some(&|d: &mut Duel, x| card::is_faceup(d, x)), tp, 0, LOCATION_MZONE, 1, ct as u16, false, NO_EX).nil();
            // if #g==0 then return end
            if g.is_empty() {
                return;
            }
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // local c=e:GetHandler()
            let c = effect::get_handler(d, e).nil();
            // Duel.BreakEffect()
            duel::break_effect(d);
            // for sc in g:Iter() do
            for sc in group::iter(d, &g) {
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
                // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
                effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
                // e1:SetValue(sc:GetAttack()//2)
                let atk = card::get_attack(d, sc) as i64;
                effect::set_value(d, e1, atk.div_euclid(2));
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 1);
                // sc:RegisterEffect(e1)
                card::register_effect(d, sc, e1, false);
            }
        }
    }
}
