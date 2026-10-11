//! No.53 Heart-eartH
use super::prelude::*;

pub const ID: u32 = 23998625;

pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(53), listed_names: &[97403510], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,nil,5,3)
    xyz::add_procedure(d, c, None, Some(5), 3, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // atkup
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e1, EVENT_BE_BATTLE_TARGET);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(cost_target(atktg)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy replace
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTarget(s.reptg)
    effect::set_target(d, e2, Some(cost_target(reptg)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // spsummon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetCard(Duel.GetAttacker())
    match duel::get_attacker(d) {
        Some(at) => duel::set_target_card(d, at),
        None => duel::set_target_card(d, CardSet::new()),
    }
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and c:IsFaceup()
    //     and tc:IsRelateToEffect(e) and tc:CanAttack() and not tc:IsStatus(STATUS_ATTACK_CANCELED) then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        let tc = tc.nil();
        if card::is_relate_to_effect(d, tc, a.e) && card::can_attack(d, tc) && !card::is_status(d, tc, STATUS_ATTACK_CANCELED) {
            // local atk=tc:GetBaseAttack()
            let atk = card::get_base_attack(d, tc);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(atk)
            effect::set_value(d, e1, atk as i64);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
    }
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:IsReason(REASON_REPLACE) and c:CheckRemoveOverlayCard(tp,1,REASON_EFFECT) end
    if a.chk == Some(0) {
        return !card::is_reason(d, c, REASON_REPLACE) && card::check_remove_overlay_card(d, c, tp, 1, REASON_EFFECT);
    }
    // if Duel.SelectEffectYesNo(tp,c,96) then
    if duel::select_effect_yes_no(d, tp, c, Some(96)) {
        // c:RemoveOverlayCard(tp,1,1,REASON_EFFECT)
        card::remove_overlay_card(d, c, tp, 1, 1, REASON_EFFECT);
        // return true
        return true;
    }
    // else return false end
    false
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsReason(REASON_EFFECT) and c:GetOverlayCount()==0
    //     and c:IsLocation(LOCATION_GRAVE)
    card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_reason(d, c, REASON_EFFECT)
        && card::get_overlay_count(d, c) == 0
        && card::is_location(d, c, LOCATION_GRAVE)
}
/// `s.spfilter(c,e,tp,rp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rp: u8) -> bool {
    // return c:IsCode(97403510) and Duel.GetLocationCountFromEx(tp,rp,nil,c)>0
    //     and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    card::is_code(d, c, &[97403510])
        && duel::get_location_count_from_ex(d, tp, Some(rp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp, rp) = (a.e, a.tp, a.rp);
    // if chk==0 then
    if a.chk == Some(0) {
        // local pg=aux.GetMustBeMaterialGroup(tp,Group.CreateGroup(),tp,nil,nil,REASON_XYZ)
        let empty = CardSet::new();
        let pg = aux::get_must_be_material_group_opt(d, tp, Some(&empty), tp, None, None, REASON_XYZ);
        // return #pg<=0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,rp)
        return pg.is_empty() && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, rp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,e:GetHandler(),1,0,0)
    let c = effect::get_handler(d, e).nil();
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp, rp) = (a.e, a.tp, a.rp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.CreateGroup(),tp,nil,nil,REASON_XYZ)
    let empty = CardSet::new();
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&empty), tp, None, None, REASON_XYZ);
    // if #pg>0 then return end
    if !pg.is_empty() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,rp):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, cc| spfilter(d, cc, e, tp, rp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if tc then
    if let Some(tc) = tc {
        // Duel.Overlay(tc,c)
        duel::overlay(d, tc, c, false);
        // Duel.SpecialSummon(tc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, SUMMON_TYPE_XYZ, tp, tp, false, false, POS_FACEUP as u8, None);
        // tc:CompleteProcedure()
        card::complete_procedure(d, tc);
    }
}
