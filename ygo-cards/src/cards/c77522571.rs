//! Frightfur Meister
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 77522571;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FRIGHTFUR, SET_FLUFFAL, SET_EDGE_IMP], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Pendulum procedure
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // --Special Summon 1 Fiend monster from your Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.pendsptg)
    effect::set_target(d, e1, Some(cost_target(pendsptg)));
    // e1:SetOperation(s.pendspop)
    effect::set_operation(d, e1, Some(operation(pendspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Special Summon 1 "Frightfur", "Fluffal", or "Edge Imp" montser from your Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.mzonesptg)
    effect::set_target(d, e2, Some(cost_target(mzonesptg)));
    // e2:SetOperation(s.mzonespop)
    effect::set_operation(d, e2, Some(operation(mzonespop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --Special Summon 1 "Frightfur" Fusion Monster from your Extra Deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCost(s.cost)
    effect::set_cost(d, e3, Some(cost_target(cost_fn)));
    // e3:SetTarget(s.exsptg)
    effect::set_target(d, e3, Some(cost_target(exsptg)));
    // e3:SetOperation(s.exspop)
    effect::set_operation(d, e3, Some(operation(exspop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    // return true
    true
}
// s.pendcfilter(c,e,tp)
fn pendcfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard({SET_FRIGHTFUR,SET_FLUFFAL,SET_EDGE_IMP}) and c:IsLevelBelow(4) and Duel.GetMZoneCount(tp,c)>0
    if !(card::is_set_card(d, c, &[SET_FRIGHTFUR as u16, SET_FLUFFAL as u16, SET_EDGE_IMP as u16])
        && card::is_level_below(d, c, 4)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0)
    {
        return false;
    }
    // and Duel.IsExistingMatchingCard(s.pendspfilter,tp,LOCATION_DECK,0,1,nil,e,tp,c:GetLevel(),c:GetCode())
    let lv = card::get_level(d, c);
    let code = card::get_code(d, c)[0];
    duel::is_existing_matching_card(d, Some(&|d, x| pendspfilter(d, x, e, tp, lv, code)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
// s.pendspfilter(c,e,tp,lv,code)
fn pendspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: u32, code: u32) -> bool {
    // return c:IsRace(RACE_FIEND) and c:IsLevel(lv) and not c:IsCode(code) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_FIEND)
        && card::is_level(d, c, &[lv])
        && !card::is_code(d, c, &[code])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// s.pendsptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn pendsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=100 then return false end
        if effect::get_label(d, e) != 100 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // return Duel.CheckReleaseGroupCost(tp,s.pendcfilter,1,false,nil,nil,e,tp)
        return duel::check_release_group_cost(d, tp, Some(&|d, c| pendcfilter(d, c, e, tp)), 1, 1, false, None, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
    // local rc=Duel.SelectReleaseGroupCost(tp,s.pendcfilter,1,1,false,nil,nil,e,tp):GetFirst()
    let g = duel::select_release_group_cost(d, tp, Some(&|d, c| pendcfilter(d, c, e, tp)), 1, 1, false, None, NO_EX);
    let rc = group::get_first(d, &g).nil();
    // e:SetLabel(rc:GetLevel(),rc:GetCode())
    let lv = card::get_level(d, rc) as i64;
    let code = card::get_code(d, rc)[0] as i64;
    effect::set_label(d, e, &[lv, code]);
    // Duel.Release(rc,REASON_COST)
    duel::release(d, rc, REASON_COST, None);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
// s.pendspop(e,tp,eg,ep,ev,re,r,rp)
fn pendspop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<1 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    // local lv,code=e:GetLabel()
    let labels = effect::get_labels(d, e);
    let lv = labels.first().copied().unwrap_or(0) as u32;
    let code = labels.get(1).copied().unwrap_or(0) as u32;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.pendspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,lv,code)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| pendspfilter(d, c, e, tp, lv, code)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
// s.mzonespfilter(c,e,tp)
fn mzonespfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard({SET_FRIGHTFUR,SET_FLUFFAL,SET_EDGE_IMP}) and c:IsLevelBelow(4) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_FRIGHTFUR as u16, SET_FLUFFAL as u16, SET_EDGE_IMP as u16])
        && card::is_level_below(d, c, 4)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// s.mzonesptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn mzonesptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.mzonespfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let e = a.e;
        let tp = a.tp;
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| mzonespfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// s.mzonespop(e,tp,eg,ep,ev,re,r,rp)
fn mzonespop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // --Cannot Special Summon from the Extra Deck, except Fusion Monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e1, aux::stringid(ID, 3));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(_e,_c) return not _c:IsType(TYPE_FUSION) and _c:IsLocation(LOCATION_EXTRA) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_type(d, c, TYPE_FUSION) && card::is_location(d, c, LOCATION_EXTRA))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // aux.addTempLizardCheck(c,tp,function(_e,_c) return not _c:IsOriginalType(TYPE_FUSION) end)
    aux::add_temp_lizard_check(d, c, tp, Some(effect_card_fn(|d, _e, c| !card::is_original_type(d, c, TYPE_FUSION))), None, None, None, None);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<1 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.mzonespfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| mzonespfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
// s.excfilter(c)
fn excfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_FIEND) and c:HasLevel()
    card::is_race(d, c, RACE_FIEND) && card::has_level(d, c)
}
// s.exspfilter(c,e,tp,lv,sg)
fn exspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: i64, sg: Option<&CardSet>) -> bool {
    // return c:IsSetCard(SET_FRIGHTFUR) and c:IsType(TYPE_FUSION) and c:IsLevel(lv) and Duel.GetLocationCountFromEx(tp,tp,sg,c)>0
    //     and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_FUSION,tp,false,false)
    if !(card::is_set_card(d, c, &[SET_FRIGHTFUR as u16]) && card::is_type(d, c, TYPE_FUSION) && card::is_level(d, c, &[lv as u32])) {
        return false;
    }
    let ex = match sg {
        Some(g) => g.into(),
        None => NO_EX,
    };
    duel::get_location_count_from_ex(d, tp, Some(tp), ex, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_FUSION, tp, false, false, None, None, None)
}
// s.excheck(sg,tp,exg,e)
fn excheck(d: &mut Duel, sg: &CardSet, tp: u8, _exg: &CardSet, e: EffectId) -> bool {
    // return Duel.IsExistingMatchingCard(s.exspfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,sg:GetSum(Card.GetOriginalLevel),sg)
    let lv = group::get_sum(d, sg, &|d, c| card::get_original_level(d, c) as i64);
    duel::is_existing_matching_card(d, Some(&|d, c| exspfilter(d, c, e, tp, lv, Some(sg))), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
// s.exsptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn exsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    let check: aux::ReleaseCheck = &|d, sg, tp2, exg| (excheck(d, sg, tp2, exg, e), false);
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=100 then return false end
        if effect::get_label(d, e) != 100 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // return Duel.CheckReleaseGroupCost(tp,s.excfilter,2,99,false,s.excheck,nil,e)
        return duel::check_release_group_cost(d, tp, Some(&excfilter), 2, 99, false, Some(check), NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
    // local g=Duel.SelectReleaseGroupCost(tp,s.excfilter,2,99,false,s.excheck,nil,e)
    let g = duel::select_release_group_cost(d, tp, Some(&excfilter), 2, 99, false, Some(check), NO_EX);
    // e:SetLabel(g:GetSum(Card.GetOriginalLevel))
    let sum = group::get_sum(d, &g, &|d, c| card::get_original_level(d, c) as i64);
    effect::set_label(d, e, &[sum]);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
// s.exspop(e,tp,eg,ep,ev,re,r,rp)
fn exspop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.exspfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,e:GetLabel()):GetFirst()
    let lv = effect::get_label(d, e);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| exspfilter(d, c, e, tp, lv, None)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if sc then
    if let Some(sc) = group::get_first(d, &g) {
        // Duel.SpecialSummon(sc,SUMMON_TYPE_FUSION,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sc, SUMMON_TYPE_FUSION, tp, tp, false, false, POS_FACEUP as u8, None);
        // sc:CompleteProcedure()
        card::complete_procedure(d, sc);
    }
}
