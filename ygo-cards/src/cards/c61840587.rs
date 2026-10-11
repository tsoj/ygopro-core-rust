//! Portable Battery Pack
//! 携帯型バッテリー
use super::prelude::*;

pub const ID: u32 = 61840587;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BATTERYMAN], ..ScriptInfo::DEFAULT };

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
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy the Special Summoned monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e2:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy this card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    // e3:SetCondition(s.descon2)
    effect::set_condition(d, e3, Some(condition(descon2)));
    // e3:SetOperation(s.desop2)
    effect::set_operation(d, e3, Some(operation(desop2)));
    // e3:SetLabelObject(e1)
    effect::set_label_object(d, e3, Value::Effect(e1));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_BATTERYMAN) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_BATTERYMAN as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && filter(d, chkc, e, tp);
    }
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>1
    //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,2,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 1
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 2, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,2,2,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 2, 2, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 2, 0, 0);
    // if e:GetLabelObject() then e:GetLabelObject():DeleteGroup() end
    // e:SetLabelObject(nil)
    effect::set_label_object(d, e, Value::Nil);
    true
}
/// `s.sfilter(c,e,tp)`
fn sfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRelateToEffect(e) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_relate_to_effect(d, c, e) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]).remove(0).as_group().cloned().nil();
    // local sg=g:Filter(s.sfilter,nil,e,tp)
    let mut sg = group::filter(d, &g, &|d, x| sfilter(d, x, e, tp), NO_EX);
    // local sct=#sg
    let sct = sg.len();
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if sct==0 or ft<=0 or (sct>1 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)) then return end
    if sct == 0 || ft <= 0 || (sct > 1 && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()) {
        return;
    }
    // if sct>ft then
    if sct as i64 > ft as i64 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // sg=sg:Select(tp,ft,ft,nil)
        sg = group::select(d, &sg, tp, ft as u16, ft as u16, false, NO_EX).nil();
        // sct=ft
    }
    // local tc=sg:GetFirst()
    let _ = group::get_first(d, &sg);
    // while tc and Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP_ATTACK) do
    for tc in group::iter(d, &sg) {
        if !duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP_ATTACK as u8, None) {
            break;
        }
        // c:SetCardTarget(tc)
        card::set_card_target(d, c, tc);
        // tc:CreateRelation(c,RESET_EVENT|RESET_TURN_SET|RESET_TOFIELD)
        card::create_relation(d, tc, c, RESET_EVENT | RESET_TURN_SET | RESET_TOFIELD);
    }
    // e:SetLabelObject(sg)
    effect::set_label_object(d, e, Value::Group(sg.clone()));
    // sg:KeepAlive()
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.desfilter1(c,rc)`
fn desfilter1(d: &mut Duel, c: CardId, rc: CardId) -> bool {
    // return c:IsRelateToCard(rc) and c:IsLocation(LOCATION_MZONE) and not c:IsStatus(STATUS_DESTROY_CONFIRMED+STATUS_BATTLE_DESTROYED)
    card::is_relate_to_card(d, c, rc) && card::is_location(d, c, LOCATION_MZONE) && !card::is_status(d, c, STATUS_DESTROY_CONFIRMED + STATUS_BATTLE_DESTROYED)
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local sg=e:GetLabelObject():GetLabelObject()
    let e1 = effect::get_label_object(d, e).as_effect().nil();
    let sg = effect::get_label_object(d, e1).as_group().cloned();
    // if not sg then return end
    let Some(sg) = sg else {
        return;
    };
    // Duel.Destroy(sg:Filter(s.desfilter1,nil,c),REASON_EFFECT)
    let dg = group::filter(d, &sg, &|d, x| desfilter1(d, x, c), NO_EX);
    duel::destroy(d, dg, REASON_EFFECT, None, None);
    // sg:DeleteGroup()
    // e:GetLabelObject():SetLabelObject(nil)
    effect::set_label_object(d, e1, Value::Nil);
}
/// `s.descon2(e,tp,eg,ep,ev,re,r,rp)`
fn descon2(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local sg=e:GetLabelObject():GetLabelObject()
    let e1 = effect::get_label_object(d, e).as_effect().nil();
    let sg = effect::get_label_object(d, e1).as_group().cloned();
    // if not sg or c:IsStatus(STATUS_DESTROY_CONFIRMED) then return false end
    let Some(mut sg) = sg else {
        return false;
    };
    if card::is_status(d, c, STATUS_DESTROY_CONFIRMED) {
        return false;
    }
    // local rg=eg:Filter(Card.IsRelateToCard,nil,c)
    let rg = group::filter(d, &a.eg.clone().nil(), &|d, x| card::is_relate_to_card(d, x, c), NO_EX);
    // local tc=rg:GetFirst()
    let _ = group::get_first(d, &rg);
    // for tc in aux.Next(rg) do sg:RemoveCard(tc) end
    for tc in group::iter(d, &rg) {
        group::remove_card(&mut sg, &CardSet::from_card(tc));
    }
    // sg is also the label object of e1: store the change back
    effect::set_label_object(d, e1, Value::Group(sg.clone()));
    // return #sg==0
    sg.is_empty()
}
/// `s.desop2(e,tp,eg,ep,ev,re,r,rp)`
fn desop2(d: &mut Duel, a: &EffArgs) {
    // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::destroy(d, CardSet::from_card(h), REASON_EFFECT, None, None);
}
