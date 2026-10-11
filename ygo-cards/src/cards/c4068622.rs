//! BF－極光のアウロラ
//! Blackwing - Aurora the Northern Lights
use super::prelude::*;

pub const ID: u32 = 4068622;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // special summon limit
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e3:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e3, EFFECT_SPSUMMON_CONDITION);
    // e3:SetValue(aux.FALSE)
    effect::set_value_fn(d, e3, aux_false());
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
fn spfilter1(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsType(TYPE_TUNER)
    !card::is_type(d, c, TYPE_TUNER)
}
fn spfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_BLACKWING) and c:IsType(TYPE_TUNER)
    card::is_set_card(d, c, &[SET_BLACKWING as u16]) && card::is_type(d, c, TYPE_TUNER)
}
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, c: Option<CardId>) -> (bool, bool) {
    // return aux.ChkfMMZ(1)(sg,e,tp,mg) and sg:IsExists(s.spfilter1,1,nil,sg) and sg:IsExists(s.spfilter2,1,nil,sg)
    let chkf = aux::chkf_mmz(1);
    let (res, _) = chkf(d, sg, e, tp, mg, c);
    let r = res && group::is_exists(d, sg, &spfilter1, 1, NO_EX) && group::is_exists(d, sg, &spfilter2, 1, NO_EX);
    (r, false)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local rg=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,nil)
    let rg = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, NO_EX);
    // local g1=rg:Filter(s.spfilter1,nil)
    let g1 = group::filter(d, &rg, &spfilter1, NO_EX);
    // local g2=rg:Filter(s.spfilter2,nil)
    let g2 = group::filter(d, &rg, &spfilter2, NO_EX);
    // local g=g1:Clone()
    let mut g = g1.clone();
    // g:Merge(g2)
    group::add_card(&mut g, &g2);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>-2 and #g1>0 and #g2>0 and #g>1
    //     and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0)
    let rs: aux::Rescon = &rescon;
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -2
        && !g1.is_empty()
        && !g2.is_empty()
        && g.len() > 1
        && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(rs), 0, tp, None, None, None, false), Err(true))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local rg=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,nil)
    let rg = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, NO_EX);
    // local g1=rg:Filter(s.spfilter1,nil)
    let mut g1 = group::filter(d, &rg, &spfilter1, NO_EX);
    // local g2=rg:Filter(s.spfilter2,nil)
    let g2 = group::filter(d, &rg, &spfilter2, NO_EX);
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // local sg=aux.SelectUnselectGroup(g1,e,tp,2,2,s.rescon,1,tp,HINTMSG_REMOVE,nil,nil,true)
    let rs: aux::Rescon = &rescon;
    let sg = aux::select_unselect_group(d, &g1, Some(e), tp, Some(2), Some(2), Some(rs), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true).unwrap_or_default();
    // if #sg>0 then
    if !sg.is_empty() {
        // sg:KeepAlive()
        // e:SetLabelObject(sg)
        effect::set_label_object(d, e, Value::Group(sg));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    if let Value::Group(g) = effect::get_label_object(d, e) {
        duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    }
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_BLACKWING) and c:IsAbleToRemove()
    card::is_set_card(d, c, &[SET_BLACKWING as u16]) && card::is_able_to_remove(d, c, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,0,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 0, a.tp, LOCATION_EXTRA as i32);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_EXTRA,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&filter), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if tc and c:IsFaceup() and c:IsRelateToEffect(e) and Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)>0 then
    if let Some(tc) = tc {
        if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) && duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 {
            // local code=tc:GetOriginalCode()
            let code = card::get_original_code(d, tc);
            // local ba=tc:GetBaseAttack()
            let ba = card::get_base_attack(d, tc);
            // local reset_flag=RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_END
            let reset_flag = RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_END;
            // c:CopyEffect(code, reset_flag, 1)
            card::copy_effect(d, c, code, reset_flag, 1);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetReset(reset_flag)
            effect::set_reset(d, e1, reset_flag, 1);
            // e1:SetCode(EFFECT_CHANGE_CODE)
            effect::set_code(d, e1, EFFECT_CHANGE_CODE);
            // e1:SetValue(code)
            effect::set_value(d, e1, code as i64);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e2:SetReset(reset_flag)
            effect::set_reset(d, e2, reset_flag, 1);
            // e2:SetCode(EFFECT_SET_BASE_ATTACK)
            effect::set_code(d, e2, EFFECT_SET_BASE_ATTACK);
            // e2:SetValue(ba)
            effect::set_value(d, e2, ba as i64);
            // c:RegisterEffect(e2)
            card::register_effect(d, c, e2, false);
        }
    }
}
