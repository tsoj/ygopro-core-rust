//! Gravity Balance
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 37930737;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_G_GOLEM], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Substitute destruction
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.reptg)
    effect::set_target(d, e2, Some(cost_target(reptg)));
    // e2:SetValue(s.repval)
    effect::set_value_fn(d, e2, effect_card_fn(repval));
    // e2:SetOperation(s.repop)
    effect::set_operation(d, e2, Some(operation(repop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_EARTH) and c:IsFaceup() and c:IsCanBeEffectTarget(e)
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_attribute(d, c, ATTRIBUTE_EARTH)
        && card::is_faceup(d, c)
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
// function s.spcheck(sg,e,tp,mg)
fn spcheck(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetCode)==1
    (group::get_class_count(d, sg, &|d, c| card::get_code(d, c)[0] as i64) == 1, false)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, NO_EX);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>1
    //   and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //   and aux.SelectUnselectGroup(g,e,tp,2,2,s.spcheck,0) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 1
            && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && matches!(
                aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&spcheck), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,s.spcheck,1,tp,HINTMSG_SPSUMMON)
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&spcheck), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
        .unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,tg,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(tg), 2, tp, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<2 or Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 2
        || duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()
    {
        return;
    }
    // local g=Duel.GetTargetCards(e):Filter(Card.IsCanBeSpecialSummoned,nil,e,0,tp,false,false,POS_FACEUP_DEFENSE)
    let targets = duel::get_target_cards(d, a.e).nil();
    let g = group::filter(
        d,
        &targets,
        &|d, c| card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None),
        NO_EX,
    );
    // if #g<2 then return end
    if g.len() < 2 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local fid=c:GetFieldID()
    let fid = card::get_field_id(d, c);
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,fid)
        card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, fid as i64, 0);
        // Negate its effects
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1,true)
        card::register_effect(d, tc, e1, true);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        // e2:SetValue(RESET_TURN_SET)
        effect::set_value(d, e2, RESET_TURN_SET as i64);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e2,true)
        card::register_effect(d, tc, e2, true);
        // Change its ATK/DEF to 0
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e3, EFFECT_SET_ATTACK_FINAL);
        // e3:SetValue(0)
        effect::set_value(d, e3, 0);
        // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e3,true)
        card::register_effect(d, tc, e3, true);
        // local e4=e3:Clone()
        let e4 = effect::clone(d, e3);
        // e4:SetCode(EFFECT_SET_DEFENSE_FINAL)
        effect::set_code(d, e4, EFFECT_SET_DEFENSE_FINAL);
        // tc:RegisterEffect(e4,true)
        card::register_effect(d, tc, e4, true);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
    // g:KeepAlive()
    // Destroy them during the End Phase
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e5:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e5, EVENT_PHASE + PHASE_END);
    // e5:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e5, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e5:SetCountLimit(1)
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    // e5:SetLabel(fid)
    effect::set_label(d, e5, &[fid as i64]);
    // e5:SetLabelObject(g)
    effect::set_label_object(d, e5, Value::Group(g.clone()));
    // e5:SetCondition(s.descon)
    effect::set_condition(d, e5, Some(condition(descon)));
    // e5:SetOperation(s.desop)
    effect::set_operation(d, e5, Some(operation(desop)));
    // Duel.RegisterEffect(e5,tp)
    duel::register_effect(d, e5, tp);
}
// function s.desfilter(c,fid)
fn desfilter(d: &mut Duel, c: CardId, fid: i64) -> bool {
    // return c:GetFlagEffectLabel(id)==fid
    card::get_flag_effect_label(d, c, ID).first().copied() == Some(fid)
}
// function s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let label = effect::get_label(d, a.e);
    // if not g:IsExists(s.desfilter,1,nil,e:GetLabel()) then
    if !group::is_exists(d, &g, &|d, c| desfilter(d, c, label), 1, NO_EX) {
        // g:DeleteGroup()
        // e:Reset()
        effect::reset(d, a.e);
        // return false
        return false;
    }
    // else return true end
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    // local tg=g:Filter(s.desfilter,nil,e:GetLabel())
    let label = effect::get_label(d, a.e);
    let tg = group::filter(d, &g, &|d, c| desfilter(d, c, label), NO_EX);
    // Duel.Destroy(tg,REASON_EFFECT)
    duel::destroy(d, tg, REASON_EFFECT, None, None);
}
// function s.repfilter(c,tp)
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsLocation(LOCATION_MZONE) and c:IsControler(tp) and c:IsSetCard(SET_G_GOLEM)
    //   and not c:IsReason(REASON_REPLACE) and (c:IsReason(REASON_BATTLE) or (c:IsReason(REASON_EFFECT) and c:GetReasonPlayer()==1-tp))
    card::is_faceup(d, c)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_controler(d, c, tp)
        && card::is_set_card(d, c, &[SET_G_GOLEM as u16])
        && !card::is_reason(d, c, REASON_REPLACE)
        && (card::is_reason(d, c, REASON_BATTLE) || (card::is_reason(d, c, REASON_EFFECT) && card::get_reason_player(d, c) == 1 - tp))
}
// function s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemove() and eg:IsExists(s.repfilter,1,nil,tp) end
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, c, None, None, None)
            && group::is_exists(d, &a.eg.clone().nil(), &|d, c| repfilter(d, c, tp), 1, NO_EX);
    }
    // return Duel.SelectEffectYesNo(tp,c,96)
    duel::select_effect_yes_no(d, tp, c, Some(96))
}
// function s.repval(e,c)
fn repval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return s.repfilter(c,e:GetHandlerPlayer())
    repfilter(d, c, effect::get_handler_player(d, e))
}
// function s.repop(e,tp,eg,ep,ev,re,r,rp)
fn repop(d: &mut Duel, a: &EffArgs) {
    // Duel.Remove(e:GetHandler(),POS_FACEUP,REASON_EFFECT)
    let c = effect::get_handler(d, a.e).nil();
    duel::remove(d, c, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
