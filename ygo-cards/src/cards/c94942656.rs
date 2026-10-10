//! Stealth Kragen Spawn
use super::prelude::*;

pub const ID: u32 = 94942656;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NUMBER, SET_STEALTH_KRAGEN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz Summon
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_WATER),4,2)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_WATER, sc, st, Some(tp)))),
        Some(4),
        2,
        None,
        None,
        None,
        None,
        false,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    // e1:SetCondition(s.descon)
    effect::set_condition(d, e1, Some(condition(descon)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetOperation(s.regop)
    effect::set_operation(d, e2, Some(operation(regop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsMainPhase()
    duel::is_main_phase(d, None)
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAttribute,ATTRIBUTE_WATER),tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_attribute(d, c, ATTRIBUTE_WATER))), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,nil,1,1-tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, None, 1, 1 - a.tp, LOCATION_MZONE as i32);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.FaceupFilter(Card.IsAttribute,ATTRIBUTE_WATER),tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::faceup_filter(|d, c| card::is_attribute(d, c, ATTRIBUTE_WATER))),
        a.tp,
        0,
        LOCATION_MZONE,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
// function s.regop(e,tp,eg,ep,ev,re,r,rp)
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re);
    // if rc and re:IsMonsterEffect() and rc:IsSetCard(SET_NUMBER) and rc:IsType(TYPE_XYZ) then
    if let Some(rc) = rc {
        if effect::is_monster_effect(d, re) && card::is_set_card(d, rc, &[SET_NUMBER as u16]) && card::is_type(d, rc, TYPE_XYZ) {
            // local e3=Effect.CreateEffect(c)
            let e3 = effect::create_effect(d, c);
            // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
            effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
            // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
            // e3:SetCode(EVENT_DESTROYED)
            effect::set_code(d, e3, EVENT_DESTROYED);
            // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
            effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
            // e3:SetReset(RESET_EVENT|RESETS_STANDARD&~(RESET_TOGRAVE|RESET_LEAVE|RESET_REMOVE))
            effect::set_reset(d, e3, RESET_EVENT | (RESETS_STANDARD & !(RESET_TOGRAVE | RESET_LEAVE | RESET_REMOVE)), 1);
            // e3:SetCondition(s.spcon)
            effect::set_condition(d, e3, Some(condition(spcon)));
            // e3:SetTarget(s.sptg)
            effect::set_target(d, e3, Some(cost_target(sptg)));
            // e3:SetOperation(s.spop)
            effect::set_operation(d, e3, Some(operation(spop)));
            // c:RegisterEffect(e3)
            card::register_effect(d, c, e3, false);
            // Record of each materials before this card leaves the field
            // local e4=Effect.CreateEffect(c)
            let e4 = effect::create_effect(d, c);
            // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
            // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e4:SetCode(EVENT_LEAVE_FIELD_P)
            effect::set_code(d, e4, EVENT_LEAVE_FIELD_P);
            // e4:SetOperation(s.recordop)
            effect::set_operation(d, e4, Some(operation(recordop)));
            // e4:SetReset(RESET_EVENT|RESETS_STANDARD&~(RESET_TOGRAVE|RESET_LEAVE|RESET_REMOVE))
            effect::set_reset(d, e4, RESET_EVENT | (RESETS_STANDARD & !(RESET_TOGRAVE | RESET_LEAVE | RESET_REMOVE)), 1);
            // e4:SetLabelObject(e3)
            effect::set_label_object(d, e4, Value::Effect(e3));
            // c:RegisterEffect(e4)
            card::register_effect(d, c, e4, false);
        }
    }
}
// function s.recordop(e,tp,eg,ep,ev,re,r,rp)
fn recordop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetHandler():GetOverlayGroup()
    let h = effect::get_handler(d, a.e).nil();
    let g = card::get_overlay_group(d, h);
    // g:KeepAlive()
    // e:GetLabelObject():SetLabelObject(g)
    let e3 = effect::get_label_object(d, a.e).as_effect().nil();
    effect::set_label_object(d, e3, Value::Group(g));
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_STEALTH_KRAGEN) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_STEALTH_KRAGEN as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousLocation(LOCATION_MZONE)
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_MZONE)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    // if chk==0 then return #g>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,e:GetHandler(),e,tp) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return !g.is_empty()
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, h.into());
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local ct=math.min(Duel.GetLocationCount(tp,LOCATION_MZONE),#e:GetLabelObject())
    let lcount = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 as i64;
    let labelg = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let mut ct = lcount.min(labelg.len() as i64);
    // if ct<1 then return end
    if ct < 1 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ct=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ct = 1;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,1,ct,e:GetHandler(),e,tp)
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp))),
        a.tp,
        LOCATION_GRAVE,
        0,
        1,
        ct as u16,
        false,
        h.into(),
    )
    .nil();
    // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)>0 then
    if !g.is_empty() && duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        // local og=Duel.GetOperatedGroup()
        let og = duel::get_operated_group(d);
        // local mg=Duel.GetMatchingGroup(aux.NecroValleyFilter(Card.IsAttribute),tp,LOCATION_GRAVE,0,nil,ATTRIBUTE_WATER)
        let mg = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| card::is_attribute(d, c, ATTRIBUTE_WATER))), a.tp, LOCATION_GRAVE, 0, NO_EX);
        // if #og>0 and #mg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if !og.is_empty() && !mg.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_XMATERIAL as u64);
            // local xg=mg:Select(tp,1,#og,nil)
            let xg = group::select(d, &mg, a.tp, 1, og.len() as u16, false, NO_EX).nil();
            // for xc in aux.Next(xg) do
            for xc in group::iter(d, &xg) {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
                // local tc=og:Select(tp,1,1,nil):GetFirst()
                let sel = group::select(d, &og, a.tp, 1, 1, false, NO_EX).nil();
                let tc = group::get_first(d, &sel).nil();
                // Duel.Overlay(tc,xc)
                duel::overlay(d, tc, xc, false);
            }
        }
    }
}
