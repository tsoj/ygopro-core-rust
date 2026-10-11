//! Resurgam Xyz
use super::prelude::*;

pub const ID: u32 = 49082032;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RANK_UP_MAGIC, SET_NUMBER_C, SET_CXYZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // e0:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e0, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e0:SetHintTiming(TIMING_DAMAGE_STEP,TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e0, TIMING_DAMAGE_STEP, Some(TIMING_DAMAGE_STEP));
    // e0:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e0, Some(condition(stat_change_damage_step_cond)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // All Xyz Monsters you control gain 800 ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsType,TYPE_XYZ))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_type(d, c, TYPE_XYZ))));
    // e1:SetValue(800)
    effect::set_value(d, e1, 800);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Once per turn: You can discard 1 Spell, then target 1 Xyz Monster you control; ...
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
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCost(s.cost)
    effect::set_cost(d, e2, Some(cost_target(cost_fn)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `aux.StatChangeDamageStepCondition`
fn stat_change_damage_step_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    aux::stat_change_damage_step_condition(d)
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpell() and c:IsDiscardable()
    card::is_spell(d, c) && card::is_discardable(d, c, Some(REASON_COST), None)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DISCARD as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&cfilter), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST|REASON_DISCARD)
    duel::send_to_grave(d, g.clone(), REASON_COST | REASON_DISCARD, None, None);
    // if g:GetFirst():IsSetCard(SET_RANK_UP_MAGIC) then e:SetLabel(0) else e:SetLabel(1) end
    let first = group::get_first(d, &g).nil();
    if card::is_set_card(d, first, &[SET_RANK_UP_MAGIC as u16]) {
        effect::set_label(d, a.e, &[0]);
    } else {
        effect::set_label(d, a.e, &[1]);
    }
    true
}
/// `s.filter1(c,e,tp)`
fn filter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local rk=c:GetRank()
    let rk = card::get_rank(d, c);
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(c)), tp, None, None, REASON_XYZ);
    // return (#pg<=0 or (#pg==1 and pg:IsContains(c))) and c:IsFaceup() and (rk>0 or c:IsStatus(STATUS_NO_LEVEL))
    //     and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_EXTRA,0,1,nil,e,tp,c,rk+1,c:GetRace(),pg)
    (pg.is_empty() || (pg.len() == 1 && group::is_contains(&pg, c)))
        && card::is_faceup(d, c)
        && (rk > 0 || card::is_status(d, c, STATUS_NO_LEVEL))
        && {
            let race = card::get_race(d, c);
            duel::is_existing_matching_card(d, Some(&|d, x| filter2(d, x, e, tp, c, rk + 1, race, &pg)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
        }
}
/// `s.filter2(c,e,tp,mc,rk,rc,pg)`
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId, rk: u32, rc: u64, _pg: &CardSet) -> bool {
    // if c.rum_limit and not c.rum_limit(mc,e) then return false end
    if let Some(rum_limit) = crate::card_info(d, c).rum_limit {
        if !rum_limit(d, mc, e) {
            return false;
        }
    }
    // return mc:IsType(TYPE_XYZ,c,SUMMON_TYPE_XYZ,tp) and c:IsRank(rk) and c:IsRace(rc)
    //     and (c:IsSetCard(SET_NUMBER_C) or c:IsSetCard(SET_CXYZ)) and mc:IsCanBeXyzMaterial(c,tp)
    //     and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0 and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    card::is_type_ctx(d, mc, TYPE_XYZ, Some(c), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_rank(d, c, &[rk])
        && card::is_race(d, c, rc)
        && (card::is_set_card(d, c, &[SET_NUMBER_C as u16]) || card::is_set_card(d, c, &[SET_CXYZ as u16]))
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.filter1(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && filter1(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter1,tp,LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter1(d, c, a.e, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.filter1,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
    let _ = duel::select_target(d, a.tp, Some(&|d, c| filter1(d, c, a.e, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(tc),tp,nil,nil,REASON_XYZ)
    let tcg = match tc {
        Some(t) => CardSet::from_card(t),
        None => CardSet::new(),
    };
    let pg = aux::get_must_be_material_group_opt(d, a.tp, Some(&tcg), a.tp, None, None, REASON_XYZ);
    // if not tc or tc:IsFacedown() or not tc:IsRelateToEffect(e) or tc:IsControler(1-tp) or tc:IsImmuneToEffect(e)
    //     or #pg>1 or (#pg==1 and not pg:IsContains(tc)) then return end
    let tc = match tc {
        Some(t) => t,
        None => return,
    };
    if card::is_facedown(d, tc)
        || !card::is_relate_to_effect(d, tc, a.e)
        || card::is_controler(d, tc, 1 - a.tp)
        || card::is_immune_to_effect(d, tc, a.e)
        || pg.len() > 1
        || (pg.len() == 1 && !group::is_contains(&pg, tc))
    {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.filter2,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,tc,tc:GetRank()+1,tc:GetRace(),pg):GetFirst()
    let rank = card::get_rank(d, tc) + 1;
    let race = card::get_race(d, tc);
    let sg = duel::select_matching_card(d, a.tp, Some(&|d, x| filter2(d, x, a.e, a.tp, tc, rank, race, &pg)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sg);
    // if sc then
    if let Some(sc) = sc {
        // sc:SetMaterial(tc)
        card::set_material(d, sc, CardSet::from_card(tc));
        // Duel.Overlay(sc,tc)
        duel::overlay(d, sc, CardSet::from_card(tc), false);
        // Duel.SpecialSummon(sc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(sc), SUMMON_TYPE_XYZ, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        // sc:CompleteProcedure()
        card::complete_procedure(d, sc);
        // if e:GetLabel()==1 then
        if effect::get_label(d, a.e) == 1 {
            // local fid=c:GetFieldID()
            let fid = card::get_field_id(d, c);
            // sc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,fid)
            card::register_flag_effect(d, sc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, fid as i64, 0);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
            effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
            // e1:SetCode(EVENT_PHASE+PHASE_END)
            effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
            // e1:SetCountLimit(1)
            effect::set_count_limit(d, e1, 1, 0, 0, 0);
            // e1:SetLabel(fid)
            effect::set_label(d, e1, &[fid as i64]);
            // e1:SetLabelObject(sc)
            effect::set_label_object(d, e1, Value::Card(sc));
            // e1:SetCondition(s.thcon2)
            effect::set_condition(d, e1, Some(condition(thcon2)));
            // e1:SetOperation(s.thop2)
            effect::set_operation(d, e1, Some(operation(thop2)));
            // Duel.RegisterEffect(e1,tp)
            duel::register_effect(d, e1, a.tp);
        }
    }
}
/// `s.thcon2(e,tp,eg,ep,ev,re,r,rp)`
fn thcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // if tc:GetFlagEffectLabel(id)~=e:GetLabel() then
    let fl = card::get_flag_effect_label(d, tc, ID).first().copied();
    if fl != Some(effect::get_label(d, a.e)) {
        // e:Reset()
        effect::reset(d, a.e);
        // return false
        return false;
    }
    // else return true end
    true
}
/// `s.thop2(e,tp,eg,ep,ev,re,r,rp)`
fn thop2(d: &mut Duel, a: &EffArgs) {
    // Duel.SendtoDeck(e:GetLabelObject(),nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    duel::send_to_deck(d, CardSet::from_card(tc), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
}
