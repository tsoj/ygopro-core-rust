//! Metaphys Executor
use super::prelude::*;

pub const ID: u32 = 45148985;

// s.listed_series={SET_METAPHYS}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_METAPHYS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // spsummon limit
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCondition(s.sprcon)
    effect::set_condition(d, e2, Some(lua_fn(sprcon)));
    // e2:SetTarget(s.sprtg)
    effect::set_target(d, e2, Some(lua_fn(sprtg)));
    // e2:SetOperation(s.sprop)
    effect::set_operation(d, e2, Some(operation(sprop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // indes
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // cannot be banished
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetCode(EFFECT_CANNOT_REMOVE)
    effect::set_code(d, e4, EFFECT_CANNOT_REMOVE);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetTargetRange(1,1)
    effect::set_target_range(d, e4, 1, 1);
    // e4:SetTarget(s.rmlimit)
    effect::set_target(d, e4, Some(lua_fn(rmlimit)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // spsummon
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e5, aux::stringid(ID, 0));
    // e5:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e5, CATEGORY_SPECIAL_SUMMON);
    // e5:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    // e5:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e5, EFFECT_FLAG_CARD_TARGET, 0);
    // e5:SetCountLimit(1)
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCondition(s.spcon)
    effect::set_condition(d, e5, Some(condition(spcon)));
    // e5:SetTarget(s.sptg)
    effect::set_target(d, e5, Some(cost_target(sptg)));
    // e5:SetOperation(s.spop)
    effect::set_operation(d, e5, Some(operation(spop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
// function s.rmlimit(e,c,tp,r)
fn rmlimit(d: &mut Duel, a: Args) -> bool {
    // return c==e:GetHandler() and r==REASON_EFFECT
    let h = effect::get_handler(d, a.e(0)).nil();
    a.card(1) == Some(h) && a.int(3) == REASON_EFFECT as i64
}
// function s.sprfilter(c)
fn sprfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_METAPHYS) and c:IsAbleToRemoveAsCost() and (c:IsLocation(LOCATION_GRAVE) or c:IsFaceup()) and (c:IsLocation(LOCATION_SZONE) or aux.SpElimFilter(c,true,true))
    card::is_set_card(d, c, &[SET_METAPHYS as u16])
        && card::is_able_to_remove_as_cost(d, c, None)
        && (card::is_location(d, c, LOCATION_GRAVE) || card::is_faceup(d, c))
        && (card::is_location(d, c, LOCATION_SZONE) || aux::sp_elim_filter(d, c, true, true))
}
// function s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return aux.ChkfMMZ(1)(sg,e,tp,mg) and sg:GetClassCount(Card.GetCode)==#sg, sg:GetClassCount(Card.GetCode)~=#sg
    let (chkf, _) = aux::chkf_mmz(1)(d, sg, e, tp, mg, None);
    let first = chkf && group::get_class_count(d, sg, &|d, c| card::get_code(d, c)[0] as i64) == sg.len();
    let second = group::get_class_count(d, sg, &|d, c| card::get_code(d, c)[0] as i64) != sg.len();
    (first, second)
}
// function s.sprcon(e,c)
fn sprcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local g=Duel.GetMatchingGroup(s.sprfilter,tp,LOCATION_ONFIELD|LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&sprfilter), tp, LOCATION_ONFIELD | LOCATION_GRAVE, 0, NO_EX);
    // return aux.SelectUnselectGroup(g,e,tp,5,5,s.rescon,0)
    matches!(
        aux::select_unselect_group(d, &g, Some(e), tp, Some(5), Some(5), Some(&rescon), 0, tp, None, None, None, false),
        Err(true)
    )
}
// function s.sprtg(e,tp,eg,ep,ev,re,r,rp,chk,c)
fn sprtg(d: &mut Duel, a: Args) -> bool {
    let (e, tp) = (a.e(0), a.p(1));
    // local g=Duel.GetMatchingGroup(s.sprfilter,tp,LOCATION_ONFIELD|LOCATION_GRAVE,0,c)
    let ex: Exclude = match a.card(9) {
        Some(c) => c.into(),
        None => NO_EX,
    };
    let g = duel::get_matching_group(d, Some(&sprfilter), tp, LOCATION_ONFIELD | LOCATION_GRAVE, 0, ex);
    // local rg=aux.SelectUnselectGroup(g,e,tp,5,5,s.rescon,1,tp,HINTMSG_REMOVE,nil,nil,true)
    let rg = aux::select_unselect_group(d, &g, Some(e), tp, Some(5), Some(5), Some(&rescon), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true).unwrap_or_default();
    // if #rg>0 then
    if !rg.is_empty() {
        // rg:KeepAlive()
        // e:SetLabelObject(rg)
        effect::set_label_object(d, e, Value::Group(rg));
        // return true
        return true;
    }
    // return false
    false
}
// function s.sprop(e,tp,eg,ep,ev,re,r,rp,c)
fn sprop(d: &mut Duel, a: &EffArgs) {
    // local rg=e:GetLabelObject()
    // if not rg then return end
    if let Value::Group(rg) = effect::get_label_object(d, a.e) {
        // Duel.Remove(rg,POS_FACEUP,REASON_COST)
        duel::remove(d, rg, POS_FACEUP as u8, REASON_COST, None, None);
        // rg:DeleteGroup()
    }
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)>Duel.GetFieldGroupCount(tp,LOCATION_ONFIELD,0)
    let n1 = duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD);
    let n2 = duel::get_field_group_count(d, a.tp, LOCATION_ONFIELD, 0);
    n1 > n2
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_METAPHYS) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_METAPHYS as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsLocation(LOCATION_REMOVED) and chkc:IsControler(tp) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_REMOVED) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_REMOVED,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_REMOVED,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)~=0 then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, e) && duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) != 0 {
            // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
            card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
            // local e2=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, e).nil();
            let e2 = effect::create_effect(d, h);
            // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e2:SetCode(EVENT_PHASE+PHASE_END)
            effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
            // e2:SetCountLimit(1)
            effect::set_count_limit(d, e2, 1, 0, 0, 0);
            // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
            effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
            // e2:SetLabel(Duel.GetTurnCount()+1)
            let turn = duel::get_turn_count(d, None) as i64 + 1;
            effect::set_label(d, e2, &[turn]);
            // e2:SetLabelObject(tc)
            effect::set_label_object(d, e2, Value::Card(tc));
            // e2:SetCondition(s.rmcon)
            effect::set_condition(d, e2, Some(condition(rmcon)));
            // e2:SetOperation(s.rmop)
            effect::set_operation(d, e2, Some(operation(rmop)));
            // e2:SetReset(RESET_PHASE|PHASE_END,2)
            effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 2);
            // Duel.RegisterEffect(e2,tp)
            duel::register_effect(d, e2, tp);
        }
    }
}
// function s.rmcon(e,tp,eg,ep,ev,re,r,rp)
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // if tc:GetFlagEffect(id)~=0 then
    if card::get_flag_effect(d, tc, ID) != 0 {
        // return Duel.GetTurnCount()==e:GetLabel()
        let turn = duel::get_turn_count(d, None) as i64;
        turn == effect::get_label(d, a.e)
    } else {
        // e:Reset()
        effect::reset(d, a.e);
        // return false
        false
    }
}
// function s.rmop(e,tp,eg,ep,ev,re,r,rp)
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
