//! Dinomorphia Kentregina
//! ダイノルフィア・ケントレギナ
use super::prelude::*;

pub const ID: u32 = 48832775;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DINOMORPHIA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // 2 "Dinoruffia" monsters with different names
    // Fusion.AddProcMixN(c,true,true,s.ffilter,2)
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat_full(|d, c, a| ffilter(d, c, a))), 2)]);
    // Lose ATK equal to LP
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(function(_,c)return -Duel.GetLP(c:GetControler())end)
    effect::set_value_fn(d, e1, effect_card_value(|d, _e, c| -(duel::get_lp(d, card::get_controler(d, c)) as i64)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Apply Normal Trap effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some((TIMING_MAIN_END | TIMINGS_CHECK_MONSTER) as u32));
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function()return Duel.IsMainPhase()end)
    effect::set_condition(d, e2, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e2:SetCost(s.efcost)
    effect::set_cost(d, e2, Some(cost_target(efcost)));
    // e2:SetTarget(s.eftg)
    effect::set_target(d, e2, Some(cost_target(eftg)));
    // e2:SetOperation(s.efop)
    effect::set_operation(d, e2, Some(operation(efop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.ffilter(c,fc,sumtype,tp,sub,mg,sg)`
fn ffilter(d: &mut Duel, c: CardId, a: &fusion::MatArgs) -> bool {
    let (fc, sumtype, tp) = (a.fc, a.sumtype, a.tp);
    // return c:IsSetCard(SET_DINOMORPHIA,fc,sumtype,tp) and (not sg or not sg:IsExists(s.fusfilter,1,c,c:GetCode(fc,sumtype,tp),fc,sumtype,tp))
    if !card::is_set_card_ctx(d, c, &[SET_DINOMORPHIA as u16], Some(fc), sumtype, Some(tp)) {
        return false;
    }
    let Some(sg) = a.sg else { return true };
    let codes = card::get_code_ctx(d, c, Some(fc), sumtype, Some(tp));
    let code = codes.first().copied().nil();
    !group::is_exists(d, sg, &|d, x| fusfilter(d, x, code, fc, sumtype, tp), 1, c.into())
}
/// `s.fusfilter(c,code,fc,sumtype,tp)`
fn fusfilter(d: &mut Duel, c: CardId, code: u32, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsSummonCode(fc,sumtype,tp,code) and not c:IsHasEffect(511002961)
    card::is_summon_code(d, c, Some(fc), sumtype, tp, &[code]) && card::is_has_effect(d, c, 511002961, None).is_empty()
}
/// `s.effilter(c)`
fn effilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToRemoveAsCost() and c:IsSetCard(SET_DINOMORPHIA) and c:IsNormalTrap()
    //     and c:CheckActivateEffect(false,true,false)~=nil
    card::is_able_to_remove_as_cost(d, c, None)
        && card::is_set_card(d, c, &[SET_DINOMORPHIA as u16])
        && card::is_normal_trap(d, c)
        && card::check_activate_effect(d, c, false, true, false).is_some()
}
/// `s.efcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.effilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&effilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.PayLPCost(tp,Duel.GetLP(tp)//2)
    let half = duel::get_lp(d, tp).div_euclid(2) as u32;
    duel::pay_lp_cost(d, tp, half);
    true
}
/// The arguments `(e,tp,eg,ep,ev,re,r,rp)` of a copied effect, followed by `tail`
fn copy_args(a: &EffArgs, tail: &[Value]) -> Vec<Value> {
    let mut v = vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        a.re.map_or(Value::Nil, Value::Effect),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ];
    v.extend_from_slice(tail);
    v
}
/// `s.eftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eftg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then
    if let Some(chkc) = a.chkc() {
        // local te=e:GetLabelObject()
        // if not te then return end
        let Value::Effect(te) = effect::get_label_object(d, e) else { return false };
        // local tg=te:GetTarget()
        // return tg and tg(e,tp,eg,ep,ev,re,r,rp,0,chkc)
        return match effect::get_target(d, te) {
            Some(tg) => tg.call(d, &copy_args(a, &[Value::Int(0), Value::Card(chkc)])).truthy(),
            None => false,
        };
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.effilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&effilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if Duel.Remove(g,POS_FACEUP,REASON_COST)==0 then return end
    if duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_COST, None, None) == 0 {
        return false;
    }
    // local te=g:GetFirst():CheckActivateEffect(false,true,false)
    let gc = group::get_first(d, &g).nil();
    let te = card::check_activate_effect(d, gc, false, true, false).map(|x| x.0).nil();
    // e:SetLabel(te:GetLabel())
    let te_label = effect::get_labels(d, te);
    effect::set_label(d, e, &te_label);
    // e:SetLabelObject(te:GetLabelObject())
    let te_lo = effect::get_label_object(d, te);
    effect::set_label_object(d, e, te_lo);
    // local tg=te:GetTarget()
    // if tg then tg(e,tp,eg,ep,ev,re,r,rp,1) end
    if let Some(tg) = effect::get_target(d, te) {
        tg.call(d, &copy_args(a, &[Value::Int(1)]));
    }
    // te:SetLabel(e:GetLabel())
    let e_label = effect::get_labels(d, e);
    effect::set_label(d, te, &e_label);
    // te:SetLabelObject(e:GetLabelObject())
    let e_lo = effect::get_label_object(d, e);
    effect::set_label_object(d, te, e_lo);
    // e:SetLabelObject(te)
    effect::set_label_object(d, e, Value::Effect(te));
    // Duel.ClearOperationInfo(0)
    duel::clear_operation_info(d, 0);
    true
}
/// `s.efop(e,tp,eg,ep,ev,re,r,rp)`
fn efop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local te=e:GetLabelObject()
    // if not te then return end
    let Value::Effect(te) = effect::get_label_object(d, e) else { return };
    // e:SetLabel(te:GetLabel())
    let te_label = effect::get_labels(d, te);
    effect::set_label(d, e, &te_label);
    // e:SetLabelObject(te:GetLabelObject())
    let te_lo = effect::get_label_object(d, te);
    effect::set_label_object(d, e, te_lo);
    // local op=te:GetOperation()
    // if op then op(e,tp,eg,ep,ev,re,r,rp) end
    if let Some(op) = effect::get_operation(d, te) {
        op.call(d, &copy_args(a, &[]));
    }
    // te:SetLabel(e:GetLabel())
    let e_label = effect::get_labels(d, e);
    effect::set_label(d, te, &e_label);
    // te:SetLabelObject(e:GetLabelObject())
    let e_lo = effect::get_label_object(d, e);
    effect::set_label_object(d, te, e_lo);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return (r&REASON_EFFECT+REASON_BATTLE)~=0
    (a.r as u32 & (REASON_EFFECT + REASON_BATTLE)) != 0
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_DINOMORPHIA) and c:IsLevelBelow(4) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_DINOMORPHIA as u16])
        && card::is_level_below(d, c, 4)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
