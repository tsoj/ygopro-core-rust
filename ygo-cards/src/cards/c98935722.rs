//! World Legacy's Secret
use super::prelude::*;

pub const ID: u32 = 98935722;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MEKK_KNIGHT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EVENT_LEAVE_FIELD_P)
    effect::set_code(d, e2, EVENT_LEAVE_FIELD_P);
    // e2:SetOperation(s.checkop)
    effect::set_operation(d, e2, Some(operation(checkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e3:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // disable
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e4, EVENT_CHAIN_SOLVING);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCondition(s.discon)
    effect::set_condition(d, e4, Some(condition(discon)));
    // e4:SetOperation(s.disop)
    effect::set_operation(d, e4, Some(operation(disop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelAbove(5) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_above(d, c, 5) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, e, tp);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // if Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::select_yes_no(d, tp, aux::stringid(ID, 0))
    {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, e, EFFECT_FLAG_CARD_TARGET, 0);
        // e:SetOperation(s.activate)
        effect::set_operation(d, e, Some(operation(activate)));
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
        let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    } else {
        // e:SetCategory(0)
        effect::set_category(d, e, 0);
        // e:SetProperty(0)
        effect::set_property(d, e, 0, 0);
        // e:SetOperation(nil)
        effect::set_operation(d, e, None);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc:IsRelateToEffect(e)
    //     and Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP) then
    if card::is_relate_to_effect(d, c, a.e) {
        let tc = tc.nil();
        if card::is_relate_to_effect(d, tc, a.e) && duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) {
            // c:SetCardTarget(tc)
            card::set_card_target(d, c, tc);
            // Duel.SpecialSummonComplete()
            duel::special_summon_complete(d);
        }
    }
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsDisabled() then e:SetLabel(1)
    let h = effect::get_handler(d, a.e).nil();
    if card::is_disabled(d, h) {
        effect::set_label(d, a.e, &[1]);
    } else {
        // else e:SetLabel(0) end
        effect::set_label(d, a.e, &[0]);
    }
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // if e:GetLabelObject():GetLabel()~=0 then return end
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    if effect::get_label(d, lo) != 0 {
        return;
    }
    // local tc=e:GetHandler():GetFirstCardTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_first_card_target(d, h);
    // if tc and tc:IsLocation(LOCATION_MZONE) then Duel.Destroy(tc,REASON_EFFECT) end
    if let Some(tc) = tc {
        if card::is_location(d, tc, LOCATION_MZONE) {
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
/// `s.cfilter(c,seq,p)`
fn cfilter(d: &mut Duel, c: CardId, seq: u32, p: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_MEKK_KNIGHT) and c:IsColumn(seq,p,LOCATION_MZONE)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MEKK_KNIGHT as u16]) && card::is_column(d, c, seq, Some(p), Some(LOCATION_MZONE))
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if rp==tp or not re:IsMonsterEffect() then return false end
    if a.rp == tp {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    // local rc=re:GetHandler()
    let _rc = effect::get_handler(d, re);
    // local p,loc,seq=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_CONTROLER,CHAININFO_TRIGGERING_LOCATION,CHAININFO_TRIGGERING_SEQUENCE)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_CONTROLER, CHAININFO_TRIGGERING_LOCATION, CHAININFO_TRIGGERING_SEQUENCE]);
    let p = info[0].int() as u8;
    let loc = info[1].int();
    let seq = info[2].int() as u32;
    // return loc==LOCATION_MZONE and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil,seq,p)
    loc == LOCATION_MZONE as i64 && duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, seq, p)), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // Duel.NegateEffect(ev)
    duel::negate_effect(d, a.ev as u8);
}
