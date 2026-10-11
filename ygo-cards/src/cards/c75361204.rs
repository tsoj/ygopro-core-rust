//! Graydle Split
use super::prelude::*;

pub const ID: u32 = 75361204;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GRAYDLE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e1, Some(condition(stat_change_cond)));
    // e1:SetCost(aux.RemainFieldCost)
    effect::set_cost(d, e1, Some(aux::remain_field_cost()));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `aux.StatChangeDamageStepCondition`
fn stat_change_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    aux::stat_change_damage_step_condition(d)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return e:IsHasType(EFFECT_TYPE_ACTIVATE) and Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) && duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_MZONE,0,1,1,nil)
    let _ = duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsLocation(LOCATION_SZONE) or not c:IsRelateToEffect(e) or c:IsStatus(STATUS_LEAVE_CONFIRMED) then return end
    if !card::is_location(d, c, LOCATION_SZONE) || !card::is_relate_to_effect(d, c, e) || card::is_status(d, c, STATUS_LEAVE_CONFIRMED) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:IsControler(tp) then
    if card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc) && card::is_controler(d, tc, tp) {
        // Duel.Equip(tp,c,tc)
        duel::equip(d, tp, c, tc, true, false);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_EQUIP)
        effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(500)
        effect::set_value(d, e1, 500);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_EQUIP_LIMIT)
        effect::set_code(d, e2, EFFECT_EQUIP_LIMIT);
        // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e2:SetValue(1)
        effect::set_value(d, e2, 1);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e3, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
        // e3:SetType(EFFECT_TYPE_QUICK_O)
        effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
        // e3:SetRange(LOCATION_SZONE)
        effect::set_range(d, e3, LOCATION_SZONE);
        // e3:SetCode(EVENT_FREE_CHAIN)
        effect::set_code(d, e3, EVENT_FREE_CHAIN);
        // e3:SetCountLimit(1,id)
        effect::set_count_limit(d, e3, 1, ID, 0, 0);
        // e3:SetCondition(s.spcon)
        effect::set_condition(d, e3, Some(condition(spcon)));
        // e3:SetCost(s.spcost)
        effect::set_cost(d, e3, Some(cost_target(spcost)));
        // e3:SetTarget(s.sptg)
        effect::set_target(d, e3, Some(cost_target(sptg)));
        // e3:SetOperation(s.spop)
        effect::set_operation(d, e3, Some(operation(spop)));
        // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e3)
        card::register_effect(d, c, e3, false);
    } else {
        // c:CancelToGrave(false)
        card::cancel_to_grave(d, c, false);
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetEquipTarget() and Duel.IsTurnPlayer(tp) and (Duel.IsMainPhase())
    let h = effect::get_handler(d, a.e).nil();
    card::get_equip_target(d, h).is_some() && duel::is_turn_player(d, a.tp) && duel::is_main_phase(d, None)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    // local h=e:GetHandler()
    let h = effect::get_handler(d, e).nil();
    // if chk==0 then return e:GetHandler():IsAbleToGraveAsCost() end
    if a.chk == Some(0) {
        return card::is_able_to_grave_as_cost(d, h);
    }
    // e:SetLabelObject(e:GetHandler():GetEquipTarget())
    let et = card::get_equip_target(d, h);
    effect::set_label_object(d, e, Value::card(et));
    // Duel.SendtoGrave(e:GetHandler(),REASON_COST)
    duel::send_to_grave(d, h, REASON_COST, None, None);
    true
}
/// `s.spfilter1(c,e,tp)`
fn spfilter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GRAYDLE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingMatchingCard(s.spfilter2,tp,LOCATION_DECK,0,1,nil,e,tp,c:GetCode())
    let code = card::get_code(d, c)[0];
    card::is_set_card(d, c, &[SET_GRAYDLE as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter2(d, x, e, tp, code)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.spfilter2(c,e,tp,code)`
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32) -> bool {
    // return c:IsSetCard(SET_GRAYDLE) and not c:IsCode(code) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_GRAYDLE as u16]) && !card::is_code(d, c, &[code]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter1,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter1(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // local ec=e:GetLabelObject()
    let ec = effect::get_label_object(d, e).as_card().nil();
    // ec:CreateEffectRelation(e)
    card::create_effect_relation(d, ec, e);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,ec,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(ec)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local ec=e:GetLabelObject()
    let ec = effect::get_label_object(d, e).as_card().nil();
    // if ec:IsRelateToEffect(e) and ec:IsFaceup() and Duel.Destroy(ec,REASON_EFFECT)~=0
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>1 then
    if card::is_relate_to_effect(d, ec, e)
        && card::is_faceup(d, ec)
        && duel::destroy(d, ec, REASON_EFFECT, None, None) != 0
        && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 1
    {
        // local fid=e:GetHandler():GetFieldID()
        let h = effect::get_handler(d, e).nil();
        let fid = card::get_field_id(d, h);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g1=Duel.SelectMatchingCard(tp,s.spfilter1,tp,LOCATION_DECK,0,1,1,nil,e,tp)
        let mut g1 = duel::select_matching_card(d, tp, Some(&|d, c| spfilter1(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // local tc1=g1:GetFirst()
        // if not tc1 then return end
        let Some(tc1) = group::get_first(d, &g1) else {
            return;
        };
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g2=Duel.SelectMatchingCard(tp,s.spfilter2,tp,LOCATION_DECK,0,1,1,nil,e,tp,tc1:GetCode())
        let code1 = card::get_code(d, tc1)[0];
        let g2 = duel::select_matching_card(d, tp, Some(&|d, c| spfilter2(d, c, e, tp, code1)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // local tc2=g2:GetFirst()
        let tc2 = group::get_first(d, &g2).nil();
        // Duel.SpecialSummonStep(tc1,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, tc1, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        // Duel.SpecialSummonStep(tc2,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, tc2, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        // tc1:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,fid)
        card::register_flag_effect(d, tc1, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, fid as i64, 0);
        // tc2:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,fid)
        card::register_flag_effect(d, tc2, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, fid as i64, 0);
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
        // g1:Merge(g2)
        group::add_card(&mut g1, &g2);
        // g1:KeepAlive()
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
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
        // e1:SetLabelObject(g1)
        effect::set_label_object(d, e1, Value::Group(g1));
        // e1:SetCondition(s.descon)
        effect::set_condition(d, e1, Some(condition(descon)));
        // e1:SetOperation(s.desop)
        effect::set_operation(d, e1, Some(operation(desop)));
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
}
/// `s.desfilter(c,fid)`
fn desfilter(d: &mut Duel, c: CardId, fid: i64) -> bool {
    // return c:GetFlagEffectLabel(id)==fid
    card::get_flag_effect_label(d, c, ID).first().copied() == Some(fid)
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, e).as_group().cloned().nil();
    let fid = effect::get_label(d, e);
    // if not g:IsExists(s.desfilter,1,nil,e:GetLabel()) then
    if !group::is_exists(d, &g, &|d, c| desfilter(d, c, fid), 1, NO_EX) {
        // g:DeleteGroup()
        // e:Reset()
        effect::reset(d, e);
        // return false
        return false;
    }
    // else return true end
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, e).as_group().cloned().nil();
    // local tg=g:Filter(s.desfilter,nil,e:GetLabel())
    let fid = effect::get_label(d, e);
    let tg = group::filter(d, &g, &|d, c| desfilter(d, c, fid), NO_EX);
    // Duel.Destroy(tg,REASON_EFFECT)
    duel::destroy(d, tg, REASON_EFFECT, None, None);
}
