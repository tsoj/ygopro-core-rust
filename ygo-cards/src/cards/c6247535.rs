//! Borreload eXcharge Dragon
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 6247535;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_BORREL],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz.AddProcedure(c,s.mfilter,4,2)
    xyz::add_procedure(d, c, Some(mat_filter(mfilter)), Some(4), 2, None, None, None, None, false, None);
    // Cannot be targeted by monsters effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.tgcon)
    effect::set_condition(d, e1, Some(condition(tgcon)));
    // e1:SetValue(s.tgval)
    effect::set_value_fn(d, e1, lua_fn(tgval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Decrease ATK/DEF and special summon 1 "Borrel" monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(cost_target(atktg)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.mfilter(c,lc,sumtype,tp)`
fn mfilter(d: &mut Duel, c: CardId, lc: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    // return c:IsRace(RACE_DRAGON,lc,sumtype,tp) and c:IsAttribute(ATTRIBUTE_DARK,lc,sumtype,tp)
    card::is_race_ctx(d, c, RACE_DRAGON, lc, sumtype, Some(tp)) && card::is_attribute_ctx(d, c, ATTRIBUTE_DARK, lc, sumtype, Some(tp))
}
/// `s.tgcon(e)`
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsXyzSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_xyz_summoned(d, h)
}
/// `s.tgval(e,re,rp)`
fn tgval(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let re = a.e(1);
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, re);
    // return re:IsMonsterEffect() and e:GetHandler()~=rc
    effect::is_monster_effect(d, re) && effect::get_handler(d, e) != rc
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and (c:GetAttack()>0 or c:GetDefense()>0)
    card::is_faceup(d, c) && (card::get_attack(d, c) > 0 || card::get_defense(d, c) > 0)
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.atkfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && atkfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.atkfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| atkfilter(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.atkfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let _ = duel::select_target(d, a.tp, Some(&|d, c| atkfilter(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_BORREL) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_BORREL as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and not tc:IsImmuneToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && !card::is_immune_to_effect(d, tc, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(-600)
        effect::set_value(d, e1, -600);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_UPDATE_DEFENSE)
        effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
        // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,nil,e,tp)
        let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp))), a.tp, LOCATION_GRAVE, 0, NO_EX);
        // if #g>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if !g.is_empty() && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
            // local sc=g:Select(tp,1,1,nil):GetFirst()
            let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
            let sc = group::get_first(d, &sel).nil();
            // if Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)~=0 then
            if duel::special_summon(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) != 0 {
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
                duel::register_effect(d, e3, a.tp);
            }
        }
    }
    // local ge1=Effect.CreateEffect(c)
    let ge1 = effect::create_effect(d, c);
    // ge1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, ge1, EFFECT_TYPE_FIELD);
    // ge1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, ge1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // ge1:SetProperty(EFFECT_FLAG_OATH+EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, ge1, EFFECT_FLAG_OATH + EFFECT_FLAG_PLAYER_TARGET, 0);
    // ge1:SetTargetRange(1,0)
    effect::set_target_range(d, ge1, 1, 0);
    // ge1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, ge1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(ge1,tp)
    duel::register_effect(d, ge1, a.tp);
    // local ge2=Effect.CreateEffect(c)
    let ge2 = effect::create_effect(d, c);
    // ge2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, ge2, EFFECT_TYPE_FIELD);
    // ge2:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
    effect::set_code(d, ge2, EFFECT_CANNOT_DIRECT_ATTACK);
    // ge2:SetProperty(EFFECT_FLAG_OATH+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, ge2, EFFECT_FLAG_OATH + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // ge2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, ge2, LOCATION_MZONE, 0);
    // ge2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, ge2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(ge2,tp)
    duel::register_effect(d, ge2, a.tp);
    // aux.RegisterClientHint(e:GetHandler(),nil,tp,1,0,aux.Stringid(id,2),nil)
    let h = effect::get_handler(d, a.e).nil();
    aux::register_client_hint(d, h, 0, a.tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
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
