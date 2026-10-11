//! Dragunity Legion
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 28927782;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DRAGUNITY], ..ScriptInfo::DEFAULT };

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
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(65)
    effect::set_description(d, e2, 65);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_END_PHASE));
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.eftg2)
    effect::set_target(d, e2, Some(cost_target(eftg2)));
    // e2:SetOperation(s.efop)
    effect::set_operation(d, e2, Some(operation(efop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.select(e,tp,b1,b2)`
fn select(d: &mut Duel, e: EffectId, tp: u8, b1: bool, b2: bool) {
    // local op=0 (every branch below sets it)
    // if b1 and b2 then
    let op: i64 = if b1 && b2 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))+1
        duel::select_option(d, tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)]) as i64 + 1
    // elseif b1 then
    } else if b1 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,1))+1
        duel::select_option(d, tp, true, &[aux::stringid(ID, 1)]) as i64 + 1
    // else op=Duel.SelectOption(tp,aux.Stringid(id,2))+2 end
    } else {
        duel::select_option(d, tp, true, &[aux::stringid(ID, 2)]) as i64 + 2
    };
    // e:SetLabel(op)
    effect::set_label(d, e, &[op]);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_SZONE,0,1,1,nil,e,tp)
        let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_SZONE, 0, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    // else
    } else {
        // e:SetCategory(CATEGORY_EQUIP)
        effect::set_category(d, e, CATEGORY_EQUIP);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
        // local g=Duel.SelectTarget(tp,s.eqfilter1,tp,LOCATION_MZONE,0,1,1,nil,tp)
        let g = duel::select_target(d, tp, Some(&|d, c| eqfilter1(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, g, 1, tp, 0);
    }
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_DRAGUNITY) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_DRAGUNITY as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.eqfilter1(c,tp)`
fn eqfilter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_DRAGUNITY) and Duel.IsExistingMatchingCard(s.eqfilter2,tp,LOCATION_GRAVE,0,1,nil,c,tp)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_DRAGUNITY as u16])
        && duel::is_existing_matching_card(d, Some(&|d, x| eqfilter2(d, x, c, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.eqfilter2(c,tc,tp)`
fn eqfilter2(d: &mut Duel, c: CardId, _tc: CardId, _tp: u8) -> bool {
    // return c:IsSetCard(SET_DRAGUNITY) and c:IsMonster() and not c:IsForbidden()
    card::is_set_card(d, c, &[SET_DRAGUNITY as u16]) && card::is_monster(d, c) && !card::is_forbidden(d, c)
}
/// `s.eftg2(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eftg2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then
    if let Some(chkc) = a.chkc() {
        // if e:GetLabel()==0 then
        if effect::get_label(d, a.e) == 0 {
            // return false
            return false;
        // elseif e:GetLabel()==1 then
        } else if effect::get_label(d, a.e) == 1 {
            // return chkc:IsLocation(LOCATION_SZONE) and s.spfilter(chkc,e,tp)
            return card::is_location(d, chkc, LOCATION_SZONE) && spfilter(d, chkc, a.e, tp);
        // else
        } else {
            // return chkc:IsLocation(LOCATION_MZONE) and s.eqfilter1(chkc,tp) end
            return card::is_location(d, chkc, LOCATION_MZONE) && eqfilter1(d, chkc, tp);
        }
    }
    // local b1=Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_SZONE,0,1,nil,e,tp)
    let b1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_SZONE, 0, 1, NO_EX);
    // local b2=Duel.GetLocationCount(tp,LOCATION_SZONE)>0 and Duel.IsExistingTarget(s.eqfilter1,tp,LOCATION_MZONE,0,1,nil,tp)
    let b2 = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
        && duel::is_existing_target(d, Some(&|d, c| eqfilter1(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    // if chk==0 then
    if a.chk == Some(0) {
        // return b1 or b2
        return b1 || b2;
    }
    // s.select(e,tp,b1,b2)
    select(d, a.e, tp, b1, b2);
    true
}
/// `s.efop(e,tp,eg,ep,ev,re,r,rp)`
fn efop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if e:GetLabel()==0 then
    if effect::get_label(d, a.e) == 0 {
        // return
        return;
    // elseif e:GetLabel()==1 then
    } else if effect::get_label(d, a.e) == 1 {
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d);
        // if tc and tc:IsRelateToEffect(e) then
        if let Some(tc) = tc {
            if card::is_relate_to_effect(d, tc, a.e) {
                // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
                duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
            }
        }
    // else
    } else {
        // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
        if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        // local ec=Duel.GetFirstTarget()
        let ec = duel::first_target(d);
        // if ec and ec:IsRelateToEffect(e) and ec:IsFaceup() then
        if let Some(ec) = ec {
            if card::is_relate_to_effect(d, ec, a.e) && card::is_faceup(d, ec) {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
                // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.eqfilter2),tp,LOCATION_GRAVE,0,1,1,nil,ec,tp)
                let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(|d, x| eqfilter2(d, x, ec, tp))), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
                // local tc=g:GetFirst()
                let tc = group::get_first(d, &g);
                // if not tc or not Duel.Equip(tp,tc,ec,true) then return end
                let Some(tc) = tc else {
                    return;
                };
                if !duel::equip(d, tp, tc, ec, true, false) {
                    return;
                }
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetCode(EFFECT_EQUIP_LIMIT)
                effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // e1:SetValue(s.eqlimit2)
                effect::set_value_fn(d, e1, effect_card_fn(eqlimit2));
                // e1:SetLabelObject(ec)
                effect::set_label_object(d, e1, Value::Card(ec));
                // tc:RegisterEffect(e1)
                card::register_effect(d, tc, e1, false);
            }
        }
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetLabelObject(e)
    effect::set_label_object(d, e1, Value::Effect(a.e));
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(effect_card_fn(splimit)));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,3),nil)
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 3)), 0, None);
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp,se)`
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsSetCard(SET_DRAGUNITY)
    !card::is_set_card(d, c, &[SET_DRAGUNITY as u16])
}
/// `s.eqlimit2(e,c)`
fn eqlimit2(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c==e:GetLabelObject()
    effect::get_label_object(d, e).as_card() == Some(c)
}
