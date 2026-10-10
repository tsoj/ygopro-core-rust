//! Dracotail Faimena
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 1498449;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DRACOTAIL as u32], ..ScriptInfo::DEFAULT };

/// `Fusion.SummonEffTG/OP({fusfilter=aux.FilterBoolFunction(Card.IsRace,RACE_DRAGON|RACE_SPELLCASTER)})` (e1)
fn e1_params() -> fusion::SummonEff {
    fusion::SummonEff { fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_race(d, c, RACE_DRAGON | RACE_SPELLCASTER))), ..Default::default() }
}
/// `Fusion.SummonEffTG(params)` of the activation with `extrafil=s.fextra(e:GetHandler())`
fn fustg_params(exc: CardId) -> fusion::SummonEff {
    fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_race(d, c, RACE_DRAGON | RACE_SPELLCASTER))),
        extrafil: Some(std::sync::Arc::new(move |_d: &mut Duel, _e: EffectId, _tp: u8, _mg: &CardSet| (None, Some(fcheck(exc))))),
        ..Default::default()
    }
}
/// `s.fcheck(exc)`: `function(tp,sg,fc) return not (exc and sg:IsContains(exc)) end`
fn fcheck(exc: CardId) -> LuaFn {
    lua_fn(move |_d, a| {
        let sg = a.g(1);
        !group::is_contains(&sg, exc)
    })
}

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion Summon 1 Dragon or Spellcaster Fusion Monster from your Extra Deck
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
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER as u32));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e1:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e1, Some(cost::self_discard()));
    // e1:SetTarget(s.fustg)
    effect::set_target(d, e1, Some(cost_target(fustg)));
    // e1:SetOperation(Fusion.SummonEffOP({fusfilter=...}))
    let op = fusion::summon_eff_op(d, e1_params());
    effect::set_operation(d, e1, Some(op));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Set 1 "Dragontail" Spell/Trap from your Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return e:GetHandler():IsLocation(LOCATION_GRAVE) and (r&REASON_FUSION)==REASON_FUSION end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_location(d, h, LOCATION_GRAVE) && (a.r & REASON_FUSION) == REASON_FUSION
    })));
    // e2:SetTarget(s.settg)
    effect::set_target(d, e2, Some(cost_target(settg)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.fustg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn fustg(d: &mut Duel, a: &EffArgs) -> bool {
    // local h = e:GetHandler(); params={extrafil=s.fextra(h), fusfilter=...}
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then ... return Fusion.SummonEffTG(params)(e,tp,eg,ep,ev,re,r,rp,0) end
    let mut call = a.clone();
    if a.chk == Some(0) {
        call.chk = Some(0);
        return fusion::summon_eff_tg_call(d, &fustg_params(h), &call);
    }
    // Fusion.SummonEffTG({fusfilter=...})(e,tp,eg,ep,ev,re,r,rp,1)
    call.chk = Some(1);
    fusion::summon_eff_tg_call(d, &e1_params(), &call);
    true
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_DRACOTAIL) and c:IsSpellTrap() and c:IsSSetable()
    card::is_set_card(d, c, &[SET_DRACOTAIL as u16]) && card::is_spell_trap(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local g=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&setfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then Duel.SSet(tp,g) end
    if !g.is_empty() {
        duel::s_set(d, tp, g, None, true);
    }
}
