//! Laevatein, Generaider Boss of Shadows
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 74615388;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_GENERAIDER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // Xyz Summon procedure: 2+ Level 9 monsters
    // Xyz.AddProcedure(c,nil,9,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(9), 2, None, None, Some(xyz::INFINITE_MATS), None, false, None);
    // Decrease ATK/DEF
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetValue(-1000)
    effect::set_value(d, e1, -1000);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 1 non-Fairy "Generaider" Xyz Monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e3, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    // e3:SetCondition(function(e) return e:GetHandler():IsXyzSummoned() end)
    effect::set_condition(d, e3, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_xyz_summoned(d, h)
    })));
    // e3:SetCost(s.spcost)
    effect::set_cost(d, e3, Some(cost_target(spcost)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsReleasable() end
    if a.chk == Some(0) {
        return card::is_releasable(d, c, None, None);
    }
    // Duel.SetTargetParam(c:GetOverlayCount())
    let ct = card::get_overlay_count(d, c) as u32;
    duel::set_target_param(d, ct);
    // Duel.Release(c,REASON_COST)
    duel::release(d, CardSet::from_card(c), REASON_COST, None);
    true
}
/// `s.spfilter(c,e,tp,ec)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ec: Option<CardId>) -> bool {
    // return c:IsType(TYPE_XYZ) and c:IsSetCard(SET_GENERAIDER) and not c:IsRace(RACE_FAIRY)
    //     and Duel.GetLocationCountFromEx(tp,tp,ec,c)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    let ex = ec.map_or(NO_EX, |x| x.into());
    card::is_type(d, c, TYPE_XYZ)
        && card::is_set_card(d, c, &[SET_GENERAIDER as u16])
        && !card::is_race(d, c, RACE_FAIRY)
        && duel::get_location_count_from_ex(d, tp, Some(tp), ex, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, Some(h))), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.ovfilter(c,xc,tp,e)`
fn ovfilter(d: &mut Duel, c: CardId, xc: CardId, tp: u8, e: EffectId) -> bool {
    // return c:IsCanBeXyzMaterial(xc,tp,REASON_EFFECT) and not c:IsImmuneToEffect(e)
    card::is_can_be_xyz_material(d, c, Some(xc), Some(tp), Some(REASON_EFFECT)) && !card::is_immune_to_effect(d, c, e)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    // local c=e:GetHandler() (unused)
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp):GetFirst()
    let sg = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp, None)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sg);
    // if not tc or Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)==0 then return end
    let Some(tc) = tc else { return };
    if duel::special_summon(d, CardSet::from_card(tc), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) == 0 {
        return;
    }
    // local ct=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let ct = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // local og=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.ovfilter),tp,LOCATION_ONFIELD|LOCATION_GRAVE,LOCATION_ONFIELD|LOCATION_GRAVE,tc,tc,tp,e)
    let og = duel::get_matching_group(
        d,
        Some(&aux::necro_valley_filter(|d, x| ovfilter(d, x, tc, a.tp, a.e))),
        a.tp,
        LOCATION_ONFIELD | LOCATION_GRAVE,
        LOCATION_ONFIELD | LOCATION_GRAVE,
        tc.into(),
    );
    // if #og==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,1)) then return end
    if og.is_empty() || !duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_XMATERIAL as u64);
    // local g=og:Select(tp,1,ct,nil)
    let g = group::select(d, &og, a.tp, 1, ct as u16, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // for tc in g:Iter() do
        for x in group::iter(d, &g) {
            // if tc:IsSpellTrap() and tc:IsLocation(LOCATION_SZONE) then
            if card::is_spell_trap(d, x) && card::is_location(d, x, LOCATION_SZONE) {
                // tc:CancelToGrave()
                card::cancel_to_grave(d, x, true);
            }
        }
        // Duel.HintSelection(g,true)
        duel::hint_selection(d, g.clone(), true);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Overlay(tc,g,true)
        duel::overlay(d, tc, g, true);
    }
}
