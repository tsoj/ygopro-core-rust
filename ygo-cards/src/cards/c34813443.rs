//! Ashened to Endlessness
use super::prelude::*;

pub const ID: u32 = 34813443;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_VEIDOS_ERUPTION_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Fusion Summon 1 Pyro Fusion Monster from your Extra Deck using monsters from either field as material
    // local params={fusfilter=aux.FilterBoolFunction(Card.IsRace,RACE_PYRO),matfilter=Fusion.OnFieldMat,extrafil=s.extrafil,stage2=s.stage2}
    let params = || fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_race(d, c, RACE_PYRO))),
        matfilter: Some(fusion::on_field_mat()),
        extrafil: Some(std::sync::Arc::new(|d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet| extrafil(d, tp))),
        stage2: Some(std::sync::Arc::new(|d: &mut Duel, e: EffectId, tc: Option<CardId>, tp: u8, sg: Option<&CardSet>, chk: i64| {
            stage2(d, e, tc, tp, sg, chk)
        })),
        ..Default::default()
    };
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
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(cond_main)));
    // e1:SetTarget(Fusion.SummonEffTG(params))
    let tg = fusion::summon_eff_tg(d, params());
    effect::set_target(d, e1, Some(tg));
    // e1:SetOperation(Fusion.SummonEffOP(params))
    let op = fusion::summon_eff_op(d, params());
    effect::set_operation(d, e1, Some(op));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Make all monsters your opponent currently controls become Pyro until the end of the turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // e2:SetCost(s.rccost)
    effect::set_cost(d, e2, Some(cost_target(rccost)));
    // e2:SetTarget(s.rctg)
    effect::set_target(d, e2, Some(cost_target(rctg)));
    // e2:SetOperation(s.rcop)
    effect::set_operation(d, e2, Some(operation(rcop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function() return Duel.IsMainPhase() end`
fn cond_main(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_main_phase(d, None)
}
/// `s.fcheck(tp,sg,fc)`: `return sg:IsExists(Card.IsCode,1,nil,CARD_VEIDOS_ERUPTION_DRAGON)`
fn fcheck(d: &mut Duel, a: Args) -> bool {
    let sg = a.g(1);
    group::is_exists(d, &sg, &|d, c| card::is_code(d, c, &[CARD_VEIDOS_ERUPTION_DRAGON]), 1, NO_EX)
}
/// `s.extrafil(e,tp,mg)`: `return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsFaceup),tp,0,LOCATION_ONFIELD,nil),s.fcheck`
fn extrafil(d: &mut Duel, tp: u8) -> (Option<CardSet>, Option<LuaFn>) {
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_faceup(d, c)), tp, 0, LOCATION_ONFIELD, NO_EX);
    (Some(g), Some(lua_fn(fcheck)))
}
/// `s.stage2(e,tc,tp,sg,chk)`
fn stage2(d: &mut Duel, e: EffectId, tc: Option<CardId>, _tp: u8, sg: Option<&CardSet>, chk: i64) {
    // if chk==0 then
    if chk == 0 {
        // Gains 500 ATK for each material used
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(#sg*500)
        let n = sg.map_or(0, |s| s.len()) as i64;
        effect::set_value(d, e1, n * 500);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc.nil(), e1, false);
    }
}
/// `aux.FaceupFilter(aux.NOT(Card.IsRace),RACE_PYRO)`
fn faceup_nonpyro(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && !card::is_race(d, c, RACE_PYRO)
}
/// `s.rccostfilter(c)`
fn rccostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK) and c:IsRace(RACE_PYRO) and c:IsAbleToRemoveAsCost()
    card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_race(d, c, RACE_PYRO) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.rccost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rccost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemoveAsCost()
    //   and Duel.IsExistingMatchingCard(s.rccostfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, c, None) && duel::is_existing_matching_card(d, Some(&rccostfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.rccostfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&rccostfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g+c,POS_FACEUP,REASON_COST)
    let mut gc = g;
    group::add_card(&mut gc, &CardSet::from_card(c));
    duel::remove(d, gc, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.rctg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rctg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.FaceupFilter(aux.NOT(Card.IsRace),RACE_PYRO),tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&faceup_nonpyro), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    true
}
/// `s.rcop(e,tp,eg,ep,ev,re,r,rp)`
fn rcop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(aux.NOT(Card.IsRace),RACE_PYRO),tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&faceup_nonpyro), tp, 0, LOCATION_MZONE, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // Becomes Pyro until the end of this turn
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_CHANGE_RACE)
        effect::set_code(d, e1, EFFECT_CHANGE_RACE);
        // e1:SetValue(RACE_PYRO)
        effect::set_value(d, e1, RACE_PYRO as i64);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
