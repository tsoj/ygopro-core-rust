//! Darktellarknight Batlamyus
use super::prelude::*;

pub const ID: u32 = 64414267;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TELLARKNIGHT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 4 "tellarknight" monsters
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_TELLARKNIGHT),4,2)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_TELLARKNIGHT as u16], sc, st, Some(tp)))),
        Some(4),
        2,
        None,
        None,
        None,
        None,
        false,
        None,
    );
    // All face-up monsters on the field become DARK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CHANGE_ATTRIBUTE)
    effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetValue(ATTRIBUTE_DARK)
    effect::set_value(d, e1, ATTRIBUTE_DARK as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 LIGHT "tellarknight" Xyz Monster from your Extra Deck by using this face-up card you control as the Xyz Material
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,0,EFFECT_COUNT_CODE_CHAIN)
    effect::set_count_limit(d, e2, 1, 0, 0, EFFECT_COUNT_CODE_CHAIN as u8);
    // e2:SetCondition(aux.NOT(s.quickcon))
    effect::set_condition(d, e2, Some(condition(|d: &mut Duel, a: &EffArgs| !quickcon(d, a))));
    // e2:SetCost(Cost.AND(Cost.DetachFromSelf(1),Cost.Discard()))
    effect::set_cost(
        d,
        e2,
        Some(cost::and(vec![cost::detach_from_self(cost::Count::N(1), None, None), cost::discard(None, false, None, None, None)])),
    );
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e3:SetCondition(s.quickcon)
    effect::set_condition(d, e3, Some(condition(quickcon)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.quickconfilter(c)`
fn quickconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_TELLARKNIGHT) and c:IsMonster()
    card::is_set_card(d, c, &[SET_TELLARKNIGHT as u16]) && card::is_monster(d, c)
}
/// `s.quickcon(e,tp,eg,ep,ev,re,r,rp)`
fn quickcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetMatchingGroup(s.quickconfilter,tp,LOCATION_GRAVE,0,nil):GetClassCount(Card.GetCode)>=7
    let g = duel::get_matching_group(d, Some(&quickconfilter), a.tp, LOCATION_GRAVE, 0, NO_EX);
    group::get_class_count(d, &g, &|d, c| card::get_code(d, c).first().copied().unwrap_or(0) as i64) >= 7
}
/// `s.spfilter(c,e,tp,mc)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsSetCard(SET_TELLARKNIGHT) and c:IsType(TYPE_XYZ)
    //     and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0 and mc:IsCanBeXyzMaterial(c,tp)
    //     and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    card::is_attribute(d, c, ATTRIBUTE_LIGHT)
        && card::is_set_card(d, c, &[SET_TELLARKNIGHT as u16])
        && card::is_type(d, c, TYPE_XYZ)
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), None)
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then
    if a.chk == Some(0) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, e).nil();
        // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
        let fc = CardSet::from_card(c);
        let pg = aux::get_must_be_material_group_opt(d, tp, Some(&fc), tp, None, None, REASON_XYZ);
        // return (#pg<=0 or (#pg==1 and pg:IsContains(c))) and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c)
        return (pg.is_empty() || (pg.len() == 1 && group::is_contains(&pg, c)))
            && duel::is_existing_matching_card(d, Some(&|d, cc| spfilter(d, cc, e, tp, c)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
    let fc = CardSet::from_card(c);
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&fc), tp, None, None, REASON_XYZ);
    // if c:IsFaceup() and c:IsRelateToEffect(e) and c:IsControler(tp) and not c:IsImmuneToEffect(e)
    //     and (#pg<=0 or (#pg==1 and pg:IsContains(c))) then
    if card::is_faceup(d, c)
        && card::is_relate_to_effect(d, c, e)
        && card::is_controler(d, c, tp)
        && !card::is_immune_to_effect(d, c, e)
        && (pg.is_empty() || (pg.len() == 1 && group::is_contains(&pg, c)))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local xyzc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,c):GetFirst()
        let sel = duel::select_matching_card(d, tp, Some(&|d, cc| spfilter(d, cc, e, tp, c)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        let xyzc = group::get_first(d, &sel);
        // if xyzc then
        if let Some(xyzc) = xyzc {
            // xyzc:SetMaterial(c)
            card::set_material(d, xyzc, CardSet::from_card(c));
            // Duel.Overlay(xyzc,c)
            duel::overlay(d, xyzc, c, false);
            // if Duel.SpecialSummon(xyzc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)>0 then
            if duel::special_summon(d, xyzc, SUMMON_TYPE_XYZ, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
                // xyzc:CompleteProcedure()
                card::complete_procedure(d, xyzc);
            }
        }
    }
    // You cannot Xyz Summon other monsters for the rest of this turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c,sump,sumtype) return (sumtype&SUMMON_TYPE_XYZ)==SUMMON_TYPE_XYZ end)
    effect::set_target(d, e1, Some(lua_fn(xyz_sumtype_limit)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `function(e,c,sump,sumtype) return (sumtype&SUMMON_TYPE_XYZ)==SUMMON_TYPE_XYZ end`
fn xyz_sumtype_limit(_d: &mut Duel, a: Args) -> bool {
    let sumtype = a.int(3);
    (sumtype & SUMMON_TYPE_XYZ as i64) == SUMMON_TYPE_XYZ as i64
}
