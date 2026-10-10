//! D/D/D Deviser King Deus Machinex
use super::prelude::*;

pub const ID: u32 = 46593546;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_DDD,SET_DARK_CONTRACT}
    listed_series: &[SET_DDD, SET_DARK_CONTRACT],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Pendulum.AddProcedure(c,false)
    pendulum::add_procedure(d, c, Some(false), None);
    // Can only control 1 in your Monster Zone
    // c:SetUniqueOnField(1,0,id,LOCATION_MZONE)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), Some(LOCATION_MZONE));
    // Xyz Summoning procedure
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_FIEND),10,2,s.ovfilter,aux.Stringid(id,0))
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, x, sc, st, tp| card::is_race_ctx(d, x, RACE_FIEND, sc, st, Some(tp)))),
        Some(10),
        2,
        Some(xyz::alter_filter(ovfilter)),
        Some(aux::stringid(ID, 0)),
        None,
        None,
        false,
        None,
    );
    // Special Summon a card from the Pendulum Zone
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Attach an opponent's Monster Card to this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.attcon)
    effect::set_condition(d, e2, Some(condition(attcon)));
    // e2:SetTarget(s.atttg)
    effect::set_target(d, e2, Some(cost_target(atttg)));
    // e2:SetOperation(s.attop)
    effect::set_operation(d, e2, Some(operation(attop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Place this card in your Pendulum Zone
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e3, aux::stringid(ID, 3));
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e3, EVENT_PHASE | PHASE_STANDBY);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(function(_,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e3, Some(condition(|d, a| duel::is_turn_player(d, a.tp))));
    // e3:SetTarget(s.pentg)
    effect::set_target(d, e3, Some(cost_target(pentg)));
    // e3:SetOperation(s.penop)
    effect::set_operation(d, e3, Some(operation(penop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.ovfilter(c,tp,sc)`
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, sc: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_DDD,sc,SUMMON_TYPE_XYZ,tp)
    card::is_faceup(d, c) && card::is_set_card_ctx(d, c, &[SET_DDD as u16], Some(sc), SUMMON_TYPE_XYZ as u64, Some(tp))
}
/// `s.penfilter(c)`
fn penfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_PENDULUM) and c:IsFaceup() and not c:IsForbidden()
    card::is_type(d, c, TYPE_PENDULUM) && card::is_faceup(d, c) && !card::is_forbidden(d, c)
}
/// The other Pendulum Zone card of `tp` (`(Duel.GetFieldGroup(tp,LOCATION_PZONE,0)-c):GetFirst()`)
fn other_pendulum_card(d: &mut Duel, c: CardId, tp: u8) -> Option<CardId> {
    let mut pg = duel::get_field_group(d, tp, LOCATION_PZONE, 0);
    group::remove_card(&mut pg, &c.into());
    group::get_first(d, &pg)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and s.penfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && penfilter(d, chkc);
    }
    // local pc=(pg-c):GetFirst()  where pg=Duel.GetFieldGroup(tp,LOCATION_PZONE,0)
    let pc = other_pendulum_card(d, c, tp);
    // if chk==0 then return pc and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and pc:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingTarget(s.penfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return pc.is_some()
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, pc.nil(), e, 0, tp, false, false, None, None, None)
            && duel::is_existing_target(d, Some(&penfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOZONE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOZONE as u64);
    // local g=Duel.SelectTarget(tp,s.penfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&penfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,pc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, pc.map(|x| x.into()), 1, tp, 0);
    // if g:GetFirst():IsLocation(LOCATION_GRAVE) then
    let first = group::get_first(d, &g).nil();
    if card::is_location(d, first, LOCATION_GRAVE) {
        // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(g), 1, 0, 0);
    }
    true
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local pc=(pg-c):GetFirst()
    // if not pc or Duel.SpecialSummon(pc,0,tp,tp,false,false,POS_FACEUP)==0 then return end
    let Some(pc) = other_pendulum_card(d, c, tp) else {
        return;
    };
    if duel::special_summon(d, pc, 0, tp, tp, false, false, POS_FACEUP as u8, None) == 0 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.CheckPendulumZones(tp) then
    if card::is_relate_to_effect(d, tc, e) && duel::check_pendulum_zones(d, tp) {
        // Duel.MoveToField(tc,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
        duel::move_to_field(d, tc, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
/// `s.attcon(e,tp,eg,ep,ev,re,r,rp)`
fn attcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // return e:GetHandler():GetFlagEffect(id)==0 and rp==1-tp and rc:IsOriginalType(TYPE_MONSTER) and rc:IsOnField()
    //     and rc:IsRelateToEffect(re)
    let c = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, c, ID) == 0
        && a.rp == 1 - a.tp
        && card::is_original_type(d, rc, TYPE_MONSTER)
        && card::is_on_field(d, rc)
        && card::is_relate_to_effect(d, rc, re)
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_DARK_CONTRACT)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DARK_CONTRACT as u16])
}
/// `s.atttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atttg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:GetOverlayCount()>=2 or Duel.IsExistingMatchingCard(s.desfilter,tp,LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return card::get_overlay_count(d, c) >= 2 || duel::is_existing_matching_card(d, Some(&desfilter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // c:RegisterFlagEffect(id,RESET_CHAIN,0,1)
    card::register_flag_effect(d, c, ID, RESET_CHAIN, 0, 1, 0, 0);
    true
}
/// `s.attop(e,tp,eg,ep,ev,re,r,rp)`
fn attop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local b1=c:IsRelateToEffect(e) and c:GetOverlayCount()>=2
    let b1 = card::is_relate_to_effect(d, c, e) && card::get_overlay_count(d, c) >= 2;
    // local b2=Duel.IsExistingMatchingCard(s.desfilter,tp,LOCATION_ONFIELD,0,1,nil)
    let b2 = duel::is_existing_matching_card(d, Some(&desfilter), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    // if not (b1 or b2) then return end
    if !(b1 || b2) {
        return;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,4)}, {b2,aux.Stringid(id,5)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 4)), (b2, aux::stringid(ID, 5))]);
    // local success=false
    let success = if op == Some(1) {
        // success=c:RemoveOverlayCard(tp,2,2,REASON_EFFECT)==2
        card::remove_overlay_card(d, c, tp, 2, 2, REASON_EFFECT) == 2
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local dg=Duel.SelectMatchingCard(tp,s.desfilter,tp,LOCATION_ONFIELD,0,1,1,nil)
        let dg = duel::select_matching_card(d, tp, Some(&desfilter), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
        // success=#dg>0 and Duel.Destroy(dg,REASON_EFFECT)>0
        !dg.is_empty() && duel::destroy(d, dg, REASON_EFFECT, None, None) > 0
    };
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // if success and rc:IsRelateToEffect(re) and rc:IsControler(1-tp) and c:IsRelateToEffect(e) and c:IsType(TYPE_XYZ)
    //     and not rc:IsImmuneToEffect(e) then
    if success
        && card::is_relate_to_effect(d, rc, re)
        && card::is_controler(d, rc, 1 - tp)
        && card::is_relate_to_effect(d, c, e)
        && card::is_type(d, c, TYPE_XYZ)
        && !card::is_immune_to_effect(d, rc, e)
    {
        // Duel.Overlay(c,rc,true)
        duel::overlay(d, c, rc, true);
    }
}
/// `s.pentg(e,tp,...,chk)`
fn pentg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsType(TYPE_PENDULUM) and Duel.CheckPendulumZones(tp) end
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return card::is_type(d, c, TYPE_PENDULUM) && duel::check_pendulum_zones(d, a.tp);
    }
    true
}
/// `s.penop(e,tp,...)`
fn penop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if not Duel.CheckPendulumZones(tp) then return end
    if !duel::check_pendulum_zones(d, tp) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, e) {
        // Duel.MoveToField(c,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
        duel::move_to_field(d, c, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
