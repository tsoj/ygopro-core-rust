//! Amorphage Pride
//! アモルファージ・ヒュペル
use super::prelude::*;

pub const ID: u32 = 6283472;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_AMORPHAGE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // flip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_FLIP)
    effect::set_code(d, e1, EVENT_FLIP);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetOperation(s.flipop)
    effect::set_operation(d, e1, Some(operation(flipop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // maintenance cost
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // special summon limit
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e3, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetTargetRange(1,1)
    effect::set_target_range(d, e3, 1, 1);
    // e3:SetTarget(s.sumlimit)
    effect::set_target(d, e3, Some(lua_fn(sumlimit)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // no effect damage
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_CHANGE_DAMAGE)
    effect::set_code(d, e4, EFFECT_CHANGE_DAMAGE);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetRange(LOCATION_PZONE)
    effect::set_range(d, e4, LOCATION_PZONE);
    // e4:SetTargetRange(1,1)
    effect::set_target_range(d, e4, 1, 1);
    // e4:SetCondition(s.damcon)
    effect::set_condition(d, e4, Some(condition(damcon)));
    // e4:SetValue(s.damval)
    effect::set_value_fn(d, e4, lua_fn(damval));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetCode(EFFECT_NO_EFFECT_DAMAGE)
    effect::set_code(d, e5, EFFECT_NO_EFFECT_DAMAGE);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // Lizard check
    // aux.addContinuousLizardCheck(c,LOCATION_MZONE,s.lizfilter,0xff,0xff)
    aux::add_continuous_lizard_check(d, c, LOCATION_MZONE, Some(lua_fn(lizfilter)), Some(0xff), Some(0xff));
}
/// `s.flipop(e,tp,eg,ep,ev,re,r,rp)`
fn flipop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.HintSelection(Group.FromCards(c),true)
    duel::hint_selection(d, CardSet::from_card(c), true);
    // if Duel.CheckReleaseGroup(tp,Card.IsReleasableByEffect,1,c) and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if duel::check_release_group(d, tp, Some(&|d, x| card::is_releasable_by_effect(d, x, None, None)), 1, duel::ReleaseArgs::default(), c.into(), false)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 0))
    {
        // local g=Duel.SelectReleaseGroup(tp,Card.IsReleasableByEffect,1,1,c)
        let g = duel::select_release_group(d, tp, Some(&|d, x| card::is_releasable_by_effect(d, x, None, None)), 1, 1, duel::ReleaseArgs::default(), c.into(), false).nil();
        // Duel.Release(g,REASON_COST)
        duel::release(d, g, REASON_COST, None);
    } else {
        // Duel.Destroy(c,REASON_COST)
        duel::destroy(d, c, REASON_COST, None, None);
    }
}
/// `s.sumlimit(e,c,sump,sumtype,sumpos,targetp,se)`
fn sumlimit(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    let e = a.e(0);
    let h = effect::get_handler(d, e).nil();
    // return c:IsLocation(LOCATION_EXTRA) and not c:IsSetCard(SET_AMORPHAGE)
    //     and (e:GetHandler():IsPendulumSummoned() or e:GetHandler():GetFlagEffect(id)~=0)
    card::is_location(d, c, LOCATION_EXTRA)
        && !card::is_set_card(d, c, &[SET_AMORPHAGE as u16])
        && (card::is_pendulum_summoned(d, h) || card::get_flag_effect(d, h, ID) != 0)
}
/// `s.damcon(e)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_AMORPHAGE),e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_AMORPHAGE as u16])), p, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.damval(e,re,val,r,rp,rc)`
fn damval(_d: &mut Duel, a: Args) -> i64 {
    // if (r&REASON_EFFECT)~=0 then return 0 end
    let r = a.int(3) as u32;
    if (r & REASON_EFFECT) != 0 {
        return 0;
    }
    // return val
    a.int(2)
}
/// `s.lizfilter(e,c)`
fn lizfilter(d: &mut Duel, a: Args) -> bool {
    // return not c:IsOriginalSetCard(SET_AMORPHAGE)
    !card::is_original_set_card(d, a.c(1), &[SET_AMORPHAGE as u16])
}
