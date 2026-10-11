//! Libromancer Displaced
//! リブロマンサー・アフェクテッド
use super::prelude::*;

pub const ID: u32 = 88083109;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LIBROMANCER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Return 1 "Libromancer" monster to the hand and take control of opponent's monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_CONTROL);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tgfilter(c,e,tp)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return (c:IsFaceup() and c:IsSetCard(SET_LIBROMANCER) and c:IsControler(tp))
    //     or (c:IsControler(1-tp) and c:IsControlerCanBeChanged(true))
    //     and c:IsCanBeEffectTarget(e)
    (card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_LIBROMANCER as u16]) && card::is_controler(d, c, tp))
        || (card::is_controler(d, c, 1 - tp)
            && card::is_controler_can_be_changed(d, c, true, None)
            && card::is_can_be_effect_target(d, c, Some(e)))
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // local rg=sg:Filter(Card.IsControler,nil,tp)
    let rg = group::filter(d, sg, &|d, c| card::is_controler(d, c, tp), NO_EX);
    // return #rg==1 and Duel.GetMZoneCount(tp,rg,tp,LOCATION_REASON_CONTROL)>0
    let res = rg.len() == 1 && duel::get_m_zone_count(d, tp, (&rg).into(), Some(tp), Some(LOCATION_REASON_CONTROL as u32), None).0 > 0;
    (res, false)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local rg=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,e,tp)
    let rg = duel::get_matching_group(d, Some(&|d, c| tgfilter(d, c, e, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if chk==0 then return aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return matches!(
            aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false),
            Err(true)
        );
    }
    // local tg=aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,1,tp,HINTMSG_TARGET)
    let tg = aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_TARGET as u64), None, None, false)
        .unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // local hg=tg:Filter(Card.IsControler,nil,tp)
    let hg = group::filter(d, &tg, &|d, c| card::is_controler(d, c, tp), NO_EX);
    // e:SetLabelObject(hg:GetFirst())
    let first = match group::get_first(d, &hg) {
        Some(c) => Value::Card(c),
        None => Value::Nil,
    };
    effect::set_label_object(d, e, first);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,hg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(hg), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,tg:Filter(Card.IsControler,nil,1-tp),1,0,0)
    let og = group::filter(d, &tg, &|d, c| card::is_controler(d, c, 1 - tp), NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(og), 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, e);
    // if #g==0 then return end
    let g = g.nil();
    if g.is_empty() {
        return;
    }
    // local tg1=g:GetFirst()
    // local tg2=g:GetNext()
    let order = group::iter(d, &g);
    let mut tg1 = order.first().copied();
    let mut tg2 = order.get(1).copied();
    // if tg2==e:GetLabelObject() then tg1,tg2=tg2,tg1 end
    if tg2 == effect::get_label_object(d, e).as_card() {
        std::mem::swap(&mut tg1, &mut tg2);
    }
    // if tg1:IsControler(tp) and Duel.SendtoHand(tg1,nil,REASON_EFFECT)>0 and tg1:IsLocation(LOCATION_HAND)
    //     and tg2 and tg2:IsControler(1-tp) then
    let tg1c = tg1.nil();
    if card::is_controler(d, tg1c, tp)
        && duel::send_to_hand(d, tg1c, None, REASON_EFFECT, None) > 0
        && card::is_location(d, tg1c, LOCATION_HAND)
        && tg2.is_some_and(|t| card::is_controler(d, t, 1 - tp))
    {
        let tg2c = tg2.nil();
        // Duel.GetControl(tg2,tp)
        duel::get_control(d, tg2c, tp, 0, 0, None, None);
        // if not tg1:IsRitualMonster() then
        if !card::is_ritual_monster(d, tg1c) {
            // local c=e:GetHandler()
            let c = effect::get_handler(d, e).nil();
            // local fid=c:GetFieldID()
            let fid = card::get_field_id(d, c);
            // tg2:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,fid)
            card::register_flag_effect(d, tg2c, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, fid as i64, 0);
            // --Return it to the hand during the End Phase
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
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
            // e1:SetLabelObject(tg2)
            effect::set_label_object(d, e1, Value::Card(tg2c));
            // e1:SetCondition(s.thcon)
            effect::set_condition(d, e1, Some(condition(thcon)));
            // e1:SetOperation(s.thop)
            effect::set_operation(d, e1, Some(operation(thop)));
            // Duel.RegisterEffect(e1,tp)
            duel::register_effect(d, e1, tp);
        }
    }
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // if tc:GetFlagEffectLabel(id)~=e:GetLabel() then
    if card::get_flag_effect_label(d, tc, ID).first().copied() != Some(effect::get_label(d, a.e)) {
        // e:Reset()
        effect::reset(d, a.e);
        // return false
        return false;
    }
    // else return true end
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.SendtoHand(e:GetLabelObject(),nil,REASON_EFFECT)
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
}
