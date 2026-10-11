//! Solfachord Musica
use super::prelude::*;

pub const ID: u32 = 56058749;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SOLFACHORD], ..ScriptInfo::DEFAULT };

/// A scale test passed as `Card.IsOddScale` / `Card.IsEvenScale` (the C function `f(c)`).
type ScaleFn = fn(&mut Duel, CardId) -> bool;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c,f)`
fn cfilter(d: &mut Duel, c: CardId, f: ScaleFn) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_SOLFACHORD) and c:IsOriginalType(TYPE_PENDULUM) and c:IsOriginalType(TYPE_MONSTER) and f(c)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_SOLFACHORD as u16])
        && card::is_original_type(d, c, TYPE_PENDULUM)
        && card::is_original_type(d, c, TYPE_MONSTER)
        && f(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc) end
    if let Some(chkc) = a.chkc() {
        return destg_f(d, tp, a.chk == Some(0), Some(chkc));
    }
    // local bo=Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_ONFIELD,0,1,nil,Card.IsOddScale)
    let bo = duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, card::is_odd_scale)), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    // local be=Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_ONFIELD,0,1,nil,Card.IsEvenScale)
    let be = duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, card::is_even_scale)), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    // local b1=bo and s.sptg(Card.IsOddScale)(e,tp,eg,ep,ev,re,r,rp,0)
    let b1 = bo && sptg_f(d, e, tp, card::is_odd_scale, true);
    // local b2=be and s.sptg(Card.IsEvenScale)(e,tp,eg,ep,ev,re,r,rp,0)
    let b2 = be && sptg_f(d, e, tp, card::is_even_scale, true);
    // local b3=bo and be and s.destg(e,tp,eg,ep,ev,re,r,rp,0)
    let b3 = bo && be && destg_f(d, tp, true, None);
    // if chk==0 then return b1 or b2 or b3 end
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    // local ops={}
    // local opval={}
    // local off=1
    let mut ops: Vec<u64> = Vec::new();
    let mut opval: Vec<i64> = Vec::new();
    // if b1 then
    if b1 {
        // ops[off]=aux.Stringid(id,0)
        ops.push(aux::stringid(ID, 0));
        // opval[off-1]=1
        opval.push(1);
        // off=off+1
    }
    // if b2 then
    if b2 {
        // ops[off]=aux.Stringid(id,1)
        ops.push(aux::stringid(ID, 1));
        // opval[off-1]=2
        opval.push(2);
    }
    // if b3 then
    if b3 {
        // ops[off]=aux.Stringid(id,2)
        ops.push(aux::stringid(ID, 2));
        // opval[off-1]=3
        opval.push(3);
    }
    // local op=Duel.SelectOption(tp,table.unpack(ops))
    let op = duel::select_option(d, tp, true, &ops);
    // local sel=opval[op]
    let sel = opval.get(op as usize).copied();
    match sel {
        // if sel==1 then
        Some(1) => {
            // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
            effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON);
            // e:SetProperty(0)
            effect::set_property(d, e, 0, 0);
            // e:SetOperation(s.spop(Card.IsOddScale))
            effect::set_operation(d, e, Some(operation(|d, a| spop_f(d, a, card::is_odd_scale))));
            // s.sptg(Card.IsOddScale)(e,tp,eg,ep,ev,re,r,rp,1)
            sptg_f(d, e, tp, card::is_odd_scale, false);
        }
        // elseif sel==2 then
        Some(2) => {
            // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
            effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON);
            // e:SetProperty(0)
            effect::set_property(d, e, 0, 0);
            // e:SetOperation(s.spop(Card.IsEvenScale))
            effect::set_operation(d, e, Some(operation(|d, a| spop_f(d, a, card::is_even_scale))));
            // s.sptg(Card.IsEvenScale)(e,tp,eg,ep,ev,re,r,rp,1)
            sptg_f(d, e, tp, card::is_even_scale, false);
        }
        // elseif sel==3 then
        Some(3) => {
            // e:SetCategory(CATEGORY_DESTROY)
            effect::set_category(d, e, CATEGORY_DESTROY);
            // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
            effect::set_property(d, e, EFFECT_FLAG_CARD_TARGET, 0);
            // e:SetOperation(s.desop)
            effect::set_operation(d, e, Some(operation(desop)));
            // s.destg(e,tp,eg,ep,ev,re,r,rp,1)
            destg_f(d, tp, false, None);
        }
        // else
        _ => {
            // e:SetCategory(0)
            effect::set_category(d, e, 0);
            // e:SetProperty(0)
            effect::set_property(d, e, 0, 0);
            // e:SetOperation(nil)
            effect::set_operation(d, e, None);
        }
    }
    true
}
/// `s.spfilter(c,e,tp,f)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, f: ScaleFn) -> bool {
    // return s.cfilter(c,f) and c:IsCanBeSpecialSummoned(e,0,tp,false,true)
    cfilter(d, c, f) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, true, None, None, None)
}
/// `s.sptg(f)(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg_f(d: &mut Duel, e: EffectId, tp: u8, f: ScaleFn, is_check: bool) -> bool {
    // if chk==0 then return Duel.GetLocationCountFromEx(tp)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,f) end
    if is_check {
        return duel::get_location_count_from_ex(d, tp, None, NO_EX, duel::ExtraCard::None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, f)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop(f)(e,tp,eg,ep,ev,re,r,rp)`
fn spop_f(d: &mut Duel, a: &EffArgs, f: ScaleFn) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCountFromEx(tp)<=0 then return end
    if duel::get_location_count_from_ex(d, tp, None, NO_EX, duel::ExtraCard::None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,f)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, f)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then Duel.SpecialSummon(g,0,tp,tp,false,true,POS_FACEUP) end
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, false, true, POS_FACEUP as u8, None);
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg_f(d: &mut Duel, tp: u8, is_check: bool, chkc: Option<CardId>) -> bool {
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsOnField() end
    if let Some(chkc) = chkc {
        return card::is_controler(d, chkc, 1 - tp) && card::is_on_field(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.TRUE,tp,0,LOCATION_ONFIELD,1,nil) end
    if is_check {
        return duel::is_existing_target(d, None, tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.TRUE,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, None, tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
