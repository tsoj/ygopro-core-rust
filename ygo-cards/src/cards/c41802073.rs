//! Vaylantz Voltage Viscount
use super::prelude::*;

pub const ID: u32 = 41802073;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_VAYLANTZ],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
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
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e) return e:GetHandler():IsSpecialSummoned() end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetTarget(s.sztg)
    effect::set_target(d, e2, Some(cost_target(sztg)));
    // e2:SetOperation(s.szop)
    effect::set_operation(d, e2, Some(operation(szop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_MOVE)
    effect::set_code(d, e3, EVENT_MOVE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(s.pzcon)
    effect::set_condition(d, e3, Some(condition(pzcon)));
    // e3:SetTarget(s.pztg)
    effect::set_target(d, e3, Some(cost_target(pztg)));
    // e3:SetOperation(s.pzop)
    effect::set_operation(d, e3, Some(operation(pzop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e) return e:GetHandler():IsSpecialSummoned() end`
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_special_summoned(d, h)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then
    if a.chk == Some(0) {
        // local zone=(1<<c:GetSequence())&ZONES_MMZ
        let zone = (1u32 << card::get_sequence(d, c)) & ZONES_MMZ;
        // return zone~=0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,tp,zone)
        return zone != 0 && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, Some(POS_FACEUP as u8), Some(a.tp), Some(zone));
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(c.into()), 1, 0, 0);
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
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e1, aux::stringid(ID, 3));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(effect_card_fn(splimit)));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.splimit(e,c)`
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsSetCard(SET_VAYLANTZ) and not c:IsLocation(LOCATION_EXTRA)
    !card::is_set_card(d, c, &[SET_VAYLANTZ as u16]) && !card::is_location(d, c, LOCATION_EXTRA)
}
/// `s.penfilter(c)`
fn penfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_VAYLANTZ) and not c:IsForbidden()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_VAYLANTZ as u16]) && !card::is_forbidden(d, c)
}
/// `s.sztg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sztg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //   and Duel.IsExistingMatchingCard(s.penfilter,tp,LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&penfilter), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    true
}
/// `s.szop(e,tp,eg,ep,ev,re,r,rp)`
fn szop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.penfilter,tp,LOCATION_EXTRA,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&penfilter), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if tc and Duel.MoveToField(tc,tp,tp,LOCATION_SZONE,POS_FACEUP,true) then
    if let Some(tc) = tc {
        if duel::move_to_field(d, tc, a.tp, a.tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetCode(EFFECT_CHANGE_TYPE)
            effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD&~RESET_TURN_SET)
            effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
            // e1:SetValue(TYPE_SPELL+TYPE_CONTINUOUS)
            effect::set_value(d, e1, (TYPE_SPELL + TYPE_CONTINUOUS) as i64);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
/// `s.pzcon(e,tp,eg,ep,ev,re,r,rp)`
fn pzcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsLocation(LOCATION_MZONE) and c:IsPreviousLocation(LOCATION_MZONE)
    card::is_location(d, c, LOCATION_MZONE) && card::is_previous_location(d, c, LOCATION_MZONE)
}
/// `s.pztg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pztg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.penfilter,tp,LOCATION_EXTRA,0,1,nil)
    //   and Duel.CheckPendulumZones(tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&penfilter), a.tp, LOCATION_EXTRA, 0, 1, NO_EX) && duel::check_pendulum_zones(d, a.tp);
    }
    true
}
/// `s.pzop(e,tp,eg,ep,ev,re,r,rp)`
fn pzop(d: &mut Duel, a: &EffArgs) {
    // if not Duel.CheckPendulumZones(tp) then return end
    if !duel::check_pendulum_zones(d, a.tp) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.penfilter,tp,LOCATION_EXTRA,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&penfilter), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if tc then
    if let Some(tc) = group::get_first(d, &sel) {
        // Duel.MoveToField(tc,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
        duel::move_to_field(d, tc, a.tp, a.tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
