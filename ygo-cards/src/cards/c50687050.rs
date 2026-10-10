//! Arktos XII - Chronochasm Vaylantz
//! Scripted by Eerie Code
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 50687050;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VAYLANTZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Pendulum.AddProcedure(c,false)
    pendulum::add_procedure(d, c, Some(false), None);
    // Fusion.AddProcMixN(c,true,true,aux.FilterBoolFunctionEx(s.ffilter),2)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| ffilter(d, c, fc, st, tp))), 2)],
    );
    // Fusion.AddContactProc(c,s.contactfil,s.contactop,s.splimit)
    fusion::add_contact_proc(
        d,
        c,
        fusion::Contact {
            material_filter: Some(Arc::new(contactfil)),
            material_operation: Some(Arc::new(contactop)),
            sumcon: Some(Some(lua_fn(splimit))),
            ..Default::default()
        },
    );
    // Special Summon self or move 1 "Valiants" monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.spmvtg)
    effect::set_target(d, e1, Some(cost_target(spmvtg)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Switch the locations of 2 monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.chtg)
    effect::set_target(d, e2, Some(cost_target(chtg)));
    // e2:SetOperation(s.chop)
    effect::set_operation(d, e2, Some(operation(chop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy 1 card on the field
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_MOVE)
    effect::set_code(d, e3, EVENT_MOVE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(s.descon)
    effect::set_condition(d, e3, Some(condition(descon)));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.ffilter(c,fc,sumtype,tp)`
fn ffilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsSetCard(SET_VAYLANTZ,fc,sumtype,tp) and c:IsLevelAbove(5)
    card::is_set_card_ctx(d, c, &[SET_VAYLANTZ as u16], Some(fc), sumtype, Some(tp)) && card::is_level_above(d, c, 5)
}
/// `s.contactfil(tp)`
fn contactfil(d: &mut Duel, tp: u8) -> CardSet {
    // return Duel.GetReleaseGroup(tp)
    duel::get_release_group(d, tp, false, false, None)
}
/// `s.contactop(g)`
fn contactop(d: &mut Duel, g: &CardSet, _tp: u8, _c: CardId) {
    // Duel.Release(g,REASON_COST|REASON_MATERIAL)
    duel::release(d, g.clone(), REASON_COST | REASON_MATERIAL, None);
}
/// `s.splimit(e,se,sp,st)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // local c=e:GetHandler()
    let h = effect::get_handler(d, a.e(0)).nil();
    // return not (c:IsLocation(LOCATION_EXTRA) and c:IsFacedown())
    !(card::is_location(d, h, LOCATION_EXTRA) && card::is_facedown(d, h))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`, called with `chk` 0 or 1
fn sptg(d: &mut Duel, e: EffectId, tp: u8, check: bool) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    if check {
        // local zone=(1<<c:GetSequence())&ZONES_MMZ
        let zone = (1u32 << card::get_sequence(d, c)) & ZONES_MMZ;
        // return zone~=0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,tp,zone)
        return zone != 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(zone));
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.mvtg(e,tp,eg,ep,ev,re,r,rp,chk)`, called with `chk` 0
fn mvtg(d: &mut Duel, tp: u8) -> bool {
    // return Duel.IsExistingTarget(Card.CheckAdjacent,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_target(d, Some(&|d, c| card::check_adjacent(d, c)), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.spmvtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spmvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local sp=s.sptg(e,tp,eg,ep,ev,re,r,rp,0)
    let sp = sptg(d, a.e, a.tp, true);
    // local mv=s.mvtg(e,tp,eg,ep,ev,re,r,rp,0)
    let mv = mvtg(d, a.tp);
    // if chk==0 then return sp or mv end
    if a.chk == Some(0) {
        return sp || mv;
    }
    // local op=Duel.SelectEffect(tp,
    //     {sp,aux.Stringid(id,3)},
    //     {mv,aux.Stringid(id,4)})
    let op = duel::select_effect(d, a.tp, &[(sp, aux::stringid(ID, 3)), (mv, aux::stringid(ID, 4))]);
    match op {
        // if op==1 then
        Some(1) => {
            // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
            effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
            // e:SetOperation(s.spop)
            effect::set_operation(d, a.e, Some(operation(spop)));
            // s.sptg(e,tp,eg,ep,ev,re,r,rp,1)
            sptg(d, a.e, a.tp, false);
        }
        // elseif op==2 then
        Some(2) => {
            // e:SetCategory(0)
            effect::set_category(d, a.e, 0);
            // e:SetOperation(s.mvop)
            effect::set_operation(d, a.e, Some(operation(mvop)));
        }
        // else
        _ => {
            // e:SetCategory(0)
            effect::set_category(d, a.e, 0);
            // e:SetOperation(nil)
            effect::set_operation(d, a.e, None);
        }
    }
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local zone=(1<<c:GetSequence())&ZONES_MMZ
    let zone = (1u32 << card::get_sequence(d, c)) & ZONES_MMZ;
    // if zone~=0 then
    if zone != 0 {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP,zone)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, Some(zone));
    }
}
/// `s.mvop(e,tp,eg,ep,ev,re,r,rp)`
fn mvop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SELECT)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SELECT as u64);
    // local tc=Duel.SelectMatchingCard(tp,Card.CheckAdjacent,tp,LOCATION_MZONE,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| card::check_adjacent(d, c)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // tc:MoveAdjacent()
        card::move_adjacent(d, tc, None);
    }
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetControler)==1
    (group::get_class_count(d, sg, &|d, c| card::get_controler(d, c) as i64) == 1, false)
}
/// `s.chtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn chtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetFieldGroup(tp,LOCATION_MMZONE,LOCATION_MMZONE)
    let g = duel::get_field_group(d, a.tp, LOCATION_MMZONE, LOCATION_MMZONE);
    // if chk==0 then return aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 0, a.tp, None, None, None, false).err().unwrap_or(false);
    }
    true
}
/// `s.chop(e,tp,eg,ep,ev,re,r,rp)`
fn chop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,LOCATION_MMZONE,LOCATION_MMZONE)
    let g = duel::get_field_group(d, a.tp, LOCATION_MMZONE, LOCATION_MMZONE);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local swap_g=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,aux.Stringid(id,5))
    let swap_g = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 1, a.tp, Some(aux::stringid(ID, 5)), None, None, false).unwrap_or_default();
    // if #swap_g~=2 then return end
    if swap_g.len() != 2 {
        return;
    }
    // Duel.HintSelection(swap_g,true)
    duel::hint_selection(d, swap_g.clone(), true);
    // Duel.SwapSequence(swap_g:GetFirst(),swap_g:GetNext())
    let order = group::iter(d, &swap_g);
    duel::swap_sequence(d, order[0], order[1]);
}
/// `s.descfilter(c)`
fn descfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_MZONE) and c:IsPreviousLocation(LOCATION_MZONE)
    card::is_location(d, c, LOCATION_MZONE) && card::is_previous_location(d, c, LOCATION_MZONE)
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.descfilter,1,nil)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &descfilter, 1, NO_EX)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,PLAYER_EITHER,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, PLAYER_EITHER as u8, LOCATION_ONFIELD as i32);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g,true)
        duel::hint_selection(d, g.clone(), true);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
