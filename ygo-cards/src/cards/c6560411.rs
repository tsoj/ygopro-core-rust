//! Bramble Rose Dragon
use super::prelude::*;

pub const ID: u32 = 6560411;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_BLACK_ROSE_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --Synchro Summon procedure: 1 Plant Tuner + 1+ non-Tuner monsters
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_PLANT),1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_PLANT, sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner(None)),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // --Each time your opponent activates a monster effect, inflict 600 to your opponent when that effect resolves, and if you do, all monsters they control lose 600 ATK
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1a, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1a:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1a, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1a:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1a, EVENT_CHAINING);
    // e1a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1a, LOCATION_MZONE);
    // e1a:SetOperation(aux.chainreg)
    effect::set_operation(d, e1a, Some(aux::chainreg()));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=Effect.CreateEffect(c)
    let e1b = effect::create_effect(d, c);
    // e1b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1b, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1b:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e1b, EVENT_CHAIN_SOLVED);
    // e1b:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1b, LOCATION_MZONE);
    // e1b:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return ep==1-tp and re:IsMonsterEffect() and e:GetHandler():HasFlagEffect(1) end)
    effect::set_condition(d, e1b, Some(condition(e1b_cond)));
    // e1b:SetOperation(s.damop)
    effect::set_operation(d, e1b, Some(operation(damop)));
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // --Special Summon 1 "Black Rose Dragon" from your Extra Deck (this is treated as a Synchro Summon)
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
    // e2:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // e2:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e2, Some(lua_fn(e2_cond)));
    // e2:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e2, Some(cost::self_tribute()));
    // e2:SetTarget(s.exsptg)
    effect::set_target(d, e2, Some(cost_target(exsptg)));
    // e2:SetOperation(s.exspop)
    effect::set_operation(d, e2, Some(operation(exspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --Special Summon 1 Level 4 or lower Plant Tuner from your GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e3, Some(cost::self_banish()));
    // e3:SetTarget(s.gysptg)
    effect::set_target(d, e3, Some(cost_target(gysptg)));
    // e3:SetOperation(s.gyspop)
    effect::set_operation(d, e3, Some(operation(gyspop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function() return Duel.IsMainPhase() end
fn e2_cond(d: &mut Duel, _a: Args) -> bool {
    duel::is_main_phase(d, None)
}
// function(e,tp,eg,ep,ev,re,r,rp) return ep==1-tp and re:IsMonsterEffect() and e:GetHandler():HasFlagEffect(1) end
fn e1b_cond(d: &mut Duel, a: &EffArgs) -> bool {
    if a.ep != 1 - a.tp {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    let c = effect::get_handler(d, a.e).nil();
    card::has_flag_effect(d, c, 1, None)
}
// function s.damop(e,tp,eg,ep,ev,re,r,rp)
fn damop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // if Duel.Damage(1-tp,600,REASON_EFFECT)>0 then
    if duel::damage(d, 1 - a.tp, 600, REASON_EFFECT, false, None) > 0 {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
        let g = duel::get_matching_group(d, Some(&|d, cc| card::is_faceup(d, cc)), a.tp, 0, LOCATION_MZONE, NO_EX);
        // g:ForEach(function(tc) tc:UpdateAttack(-600,nil,c) end)
        for tc in group::iter(d, &g) {
            card::update_attack(d, tc, -600, None, Some(c), None);
        }
    }
}
// function s.exspfilter(c,e,tp,mc)
fn exspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: Option<CardId>) -> bool {
    // return c:IsCode(CARD_BLACK_ROSE_DRAGON) and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0
    //   and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_SYNCHRO,tp,false,false)
    let ex = match mc {
        Some(m) => m.into(),
        None => NO_EX,
    };
    card::is_code(d, c, &[CARD_BLACK_ROSE_DRAGON])
        && duel::get_location_count_from_ex(d, tp, Some(tp), ex, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_SYNCHRO, tp, false, false, None, None, None)
}
// function s.exsptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn exsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.exspfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,e:GetHandler()) end
    if a.chk == Some(0) {
        let handler = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, c| exspfilter(d, c, a.e, a.tp, Some(handler))), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
// function s.exspop(e,tp,eg,ep,ev,re,r,rp)
fn exspop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.exspfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&|d, c| exspfilter(d, c, a.e, a.tp, None)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if not sc then return end
    let Some(sc) = sc else {
        return;
    };
    // sc:SetMaterial(nil)
    card::set_material(d, sc, CardSet::new());
    // if Duel.SpecialSummon(sc,SUMMON_TYPE_SYNCHRO,tp,tp,false,false,POS_FACEUP)>0 then
    if duel::special_summon(d, sc, SUMMON_TYPE_SYNCHRO as u32, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        // sc:CompleteProcedure()
        card::complete_procedure(d, sc);
    }
}
// function s.gyspfilter(c,e,tp)
fn gyspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(4) and c:IsRace(RACE_PLANT) and c:IsType(TYPE_TUNER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_below(d, c, 4)
        && card::is_race(d, c, RACE_PLANT)
        && card::is_type(d, c, TYPE_TUNER)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.gysptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn gysptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.gyspfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| gyspfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
// function s.gyspop(e,tp,eg,ep,ev,re,r,rp)
fn gyspop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.gyspfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| gyspfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
