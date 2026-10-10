//! Raider's Knight
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 28781003;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_THE_PHANTOM_KNIGHTS, SET_RAIDRAPTOR, SET_XYZ_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_DARK),4,2)
    xyz::add_procedure(d, c, Some(mat_filter(|d, x, sc, st, tp| card::is_attribute_ctx(d, x, ATTRIBUTE_DARK, sc, st, Some(tp)))), Some(4), 2, None, None, None, None, false, None);
    // Xyz Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spfilter(c,e,tp,mc,rk,pg)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId, rk: u32, pg: &CardSet) -> bool {
    // return c:IsType(TYPE_XYZ) and (c:IsSetCard(SET_RAIDRAPTOR) or c:IsSetCard(SET_XYZ_DRAGON) or c:IsSetCard(SET_THE_PHANTOM_KNIGHTS))
    if !card::is_type(d, c, TYPE_XYZ) {
        return false;
    }
    if !(card::is_set_card(d, c, &[SET_RAIDRAPTOR as u16]) || card::is_set_card(d, c, &[SET_XYZ_DRAGON as u16]) || card::is_set_card(d, c, &[SET_THE_PHANTOM_KNIGHTS as u16])) {
        return false;
    }
    // and (c:IsRank(rk-1) or c:IsRank(rk+1))
    if !(card::is_rank(d, c, &[rk.wrapping_sub(1)]) || card::is_rank(d, c, &[rk + 1])) {
        return false;
    }
    // and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0
    if !(duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0) {
        return false;
    }
    // and (#pg<=0 or pg:IsContains(mc))
    if !(pg.is_empty() || pg.contains(mc)) {
        return false;
    }
    // and mc:IsCanBeXyzMaterial(c,tp)
    if !card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), None) {
        return false;
    }
    // and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ as u32, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
        let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(c)), tp, None, None, REASON_XYZ);
        // return (#pg<=0 or (#pg==1 and pg:IsContains(c)))
        //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c,c:GetRank(),pg)
        let ok = pg.is_empty() || (pg.len() == 1 && pg.contains(c));
        return ok && {
            let rk = card::get_rank(d, c);
            duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, tp, c, rk, &pg)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
        };
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(c)), tp, None, None, REASON_XYZ);
    // if c:IsFaceup() and c:IsRelateToEffect(e) and c:IsControler(tp) and not c:IsImmuneToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) && card::is_controler(d, c, tp) && !card::is_immune_to_effect(d, c, a.e) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,c,c:GetRank(),pg)
        let rk = card::get_rank(d, c);
        let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, a.e, tp, c, rk, &pg)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        // local sc=g:GetFirst()
        let sc = group::get_first(d, &g);
        // if sc then
        if let Some(sc) = sc {
            // sc:SetMaterial(c)
            card::set_material(d, sc, CardSet::from_card(c));
            // Duel.Overlay(sc,c)
            duel::overlay(d, sc, c, false);
            // if Duel.SpecialSummon(sc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)>0 then
            if duel::special_summon(d, sc, SUMMON_TYPE_XYZ as u32, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
                // sc:CompleteProcedure()
                card::complete_procedure(d, sc);
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
                effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
                // e1:SetCode(EVENT_PHASE+PHASE_END)
                effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
                // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
                effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
                // e1:SetCountLimit(1)
                effect::set_count_limit(d, e1, 1, 0, 0, 0);
                // e1:SetLabelObject(sc)
                effect::set_label_object(d, e1, Value::Card(sc));
                // e1:SetCondition(s.descon)
                effect::set_condition(d, e1, Some(condition(descon)));
                // e1:SetOperation(s.desop)
                effect::set_operation(d, e1, Some(operation(desop)));
                // e1:SetReset(RESETS_STANDARD_PHASE_END|RESET_OPPO_TURN)
                effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END | RESET_OPPO_TURN, 1);
                // sc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END|RESET_OPPO_TURN,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,1))
                card::register_flag_effect(d, sc, ID, RESETS_STANDARD_PHASE_END | RESET_OPPO_TURN, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 1));
                // Duel.RegisterEffect(e1,tp)
                duel::register_effect(d, e1, tp);
            }
        }
    }
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return Duel.IsTurnPlayer(1-tp) and e:GetLabelObject():GetFlagEffect(id)>0
    if !duel::is_turn_player(d, 1 - tp) {
        return false;
    }
    let lo = effect::get_label_object(d, a.e).as_card().nil();
    card::get_flag_effect(d, lo, ID) > 0
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local sc=e:GetLabelObject()
    let sc = effect::get_label_object(d, a.e).as_card().nil();
    // Duel.Destroy(sc,REASON_EFFECT)
    duel::destroy(d, sc, REASON_EFFECT, None, None);
}
