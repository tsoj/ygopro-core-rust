//! Dracotail Mululu
//! scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 7375867;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DRACOTAIL], ..ScriptInfo::DEFAULT };

/// `Fusion.SummonEffTG(aux.FilterBoolFunction(Card.IsSetCard,SET_DRACOTAIL))` / `Fusion.SummonEffOP(...)`
fn fus_params() -> fusion::SummonEff {
    fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_DRACOTAIL as u16]))),
        ..Default::default()
    }
}

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion Summon 1 "Dragontail" Fusion Monster from your Extra Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(e1_cond)));
    // e1:SetTarget(Fusion.SummonEffTG(aux.FilterBoolFunction(Card.IsSetCard,SET_DRACOTAIL)))
    let tg = fusion::summon_eff_tg(d, fus_params());
    effect::set_target(d, e1, Some(tg));
    // e1:SetOperation(s.fusop)
    effect::set_operation(d, e1, Some(operation(fusop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Set 1 "Dragontail" Spell/Trap from your Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DISABLE+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_DISABLE + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return e:GetHandler():IsLocation(LOCATION_GRAVE) and (r&REASON_FUSION)==REASON_FUSION end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetTarget(s.settg)
    effect::set_target(d, e2, Some(cost_target(settg)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.fusop(e,tp,eg,ep,ev,re,r,rp)
fn fusop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Fusion.SummonEffOP(aux.FilterBoolFunction(Card.IsSetCard,SET_DRACOTAIL))(e,tp,eg,ep,ev,re,r,rp)
    fusion::summon_eff_op_call(d, &fus_params(), a);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // --You cannot Special Summon from the Extra Deck for the rest of this turn, except Fusion Monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsType(TYPE_FUSION) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !card::is_type(d, c, TYPE_FUSION))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // --"Clock Lizard" check
    // aux.addTempLizardCheck(c,tp,function(c) return not c:IsOriginalType(TYPE_FUSION) end)
    aux::add_temp_lizard_check(d, c, tp, Some(lua_fn(lizard_filter)), None, None, None, None);
}
// function(c) return not c:IsOriginalType(TYPE_FUSION) end
fn lizard_filter(d: &mut Duel, a: Args) -> bool {
    !card::is_original_type(d, a.c(0), TYPE_FUSION)
}
// function(e,tp,eg,ep,ev,re,r,rp) return e:GetHandler():IsLocation(LOCATION_GRAVE) and (r&REASON_FUSION)==REASON_FUSION end
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_location(d, c, LOCATION_GRAVE) && (a.r as u32 & REASON_FUSION) == REASON_FUSION
}
// function() return Duel.IsMainPhase() end
fn e1_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_main_phase(d, None)
}
// function s.setfilter(c)
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_DRACOTAIL) and c:IsSpellTrap() and c:IsSSetable()
    card::is_set_card(d, c, &[SET_DRACOTAIL as u16]) && card::is_spell_trap(d, c) && card::is_s_setable(d, c, false, None)
}
// function s.settg(e,tp,eg,ep,ev,re,r,rp,chk)
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DISABLE,nil,1,1-tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_DISABLE as u64, None, 1, 1 - a.tp, LOCATION_MZONE as i32);
    true
}
// function s.setop(e,tp,eg,ep,ev,re,r,rp)
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local g=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&setfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g==0 or Duel.SSet(tp,g)==0 then return end
    if g.is_empty() || duel::s_set(d, tp, g, None, true) == 0 {
        return;
    }
    // local ng=Duel.GetMatchingGroup(Card.IsNegatableMonster,tp,0,LOCATION_MZONE,nil)
    let ng = duel::get_matching_group(d, Some(&|d, c| card::is_negatable_monster(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
    // if #ng==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,3)) then return end
    if ng.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 3)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
    // local sc=ng:Select(tp,1,1,nil):GetFirst()
    let sel = group::select(d, &ng, tp, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if sc then
    if let Some(sc) = sc {
        // Duel.HintSelection(sc)
        duel::hint_selection(d, CardSet::from_card(sc), true);
        // sc:NegateEffects(e:GetHandler())
        let c = effect::get_handler(d, a.e).nil();
        card::negate_effects(d, sc, c, None, false, None);
    }
}
