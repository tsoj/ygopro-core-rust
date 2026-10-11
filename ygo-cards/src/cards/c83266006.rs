//! Ghost Meets Girl - A Mayakashi's Manuscript
use super::prelude::*;

pub const ID: u32 = 83266006;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetHintTiming(0,TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.GetCurrentPhase() & PHASE_MAIN1 + PHASE_MAIN2 ~= 0
    (duel::get_current_phase(d) & (PHASE_MAIN1 + PHASE_MAIN2)) != 0
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_ZOMBIE) and c:IsType(TYPE_SYNCHRO)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_REMOVED|LOCATION_GRAVE,0,1,nil,e,tp,c:GetAttribute())
    if !(card::is_faceup(d, c) && card::is_race(d, c, RACE_ZOMBIE) && card::is_type(d, c, TYPE_SYNCHRO)) {
        return false;
    }
    let att = card::get_attribute(d, c);
    duel::is_existing_matching_card(
        d,
        Some(&|d, x| spfilter(d, x, e, tp, att)),
        tp,
        LOCATION_REMOVED | LOCATION_GRAVE,
        0,
        1,
        NO_EX,
    )
}
/// `s.spfilter(c,e,tp,att)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, att: u32) -> bool {
    // return c:IsRace(RACE_ZOMBIE) and (c:IsFaceup() or not c:IsLocation(LOCATION_REMOVED))
    //     and c:IsAttribute(att) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_ZOMBIE)
        && (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_REMOVED))
        && card::is_attribute(d, c, att)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, tp)
            && filter(d, chkc, e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
    duel::select_target(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_REMOVED|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_REMOVED | LOCATION_GRAVE) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if card::is_faceup(d, tc.nil())
        && card::is_relate_to_effect(d, tc.nil(), e)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_REMOVED|LOCATION_GRAVE,0,1,1,nil,e,tp,tc:GetAttribute()):GetFirst()
        let att = card::get_attribute(d, tc.nil());
        let g = duel::select_matching_card(
            d,
            tp,
            Some(&|d, x| spfilter(d, x, e, tp, att)),
            tp,
            LOCATION_REMOVED | LOCATION_GRAVE,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        let sc = group::get_first(d, &g);
        // if sc and Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)~=0 then
        if let Some(sc) = sc {
            if duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) != 0 {
                // sc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
                card::register_flag_effect(d, sc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
                // local e3=Effect.CreateEffect(c)
                let e3 = effect::create_effect(d, c);
                // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
                effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
                // e3:SetCode(EVENT_PHASE+PHASE_END)
                effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
                // e3:SetCountLimit(1)
                effect::set_count_limit(d, e3, 1, 0, 0, 0);
                // e3:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
                effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
                // e3:SetLabelObject(sc)
                effect::set_label_object(d, e3, Value::Card(sc));
                // e3:SetCondition(s.rmcon)
                effect::set_condition(d, e3, Some(condition(rmcon)));
                // e3:SetOperation(s.rmop)
                effect::set_operation(d, e3, Some(operation(rmop)));
                // Duel.RegisterEffect(e3,tp)
                duel::register_effect(d, e3, tp);
            }
        }
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(effect_card_fn(splimit)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.rmcon(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // if tc:GetFlagEffect(id)~=0 then
    if card::get_flag_effect(d, tc, ID) != 0 {
        // return true
        true
    } else {
        // e:Reset()
        effect::reset(d, a.e);
        // return false
        false
    }
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp,se)`
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsRace(RACE_ZOMBIE)
    !card::is_race(d, c, RACE_ZOMBIE)
}
