//! S:P Little Knight
//! Scripted by Larry126
use super::prelude::*;

pub const ID: u32 = 29301450;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),2,2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))), 2, Some(2), None, None, None);
    // Banish 1 card from the field or GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsLinkSummoned() and e:GetLabel()==1 end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_link_summoned(d, h) && effect::get_label(d, a.e) == 1
    })));
    // e1:SetTarget(s.rmtg)
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e2, EFFECT_MATERIAL_CHECK);
    // e2:SetValue(s.valcheck)
    effect::set_value_fn(d, e2, effect_card_value(valcheck));
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Banish 2 face-up monsters until the End Phase
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e3, CATEGORY_REMOVE);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(function(_,tp,_,ep) return ep==1-tp end)
    effect::set_condition(d, e3, Some(condition(|_d, a| a.ep == 1 - a.tp)));
    // e3:SetTarget(s.tmprmtg)
    effect::set_target(d, e3, Some(cost_target(tmprmtg)));
    // e3:SetOperation(s.tmprmop)
    effect::set_operation(d, e3, Some(operation(tmprmop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.rmtg(e,tp,...,chk,chkc)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_ONFIELD|LOCATION_GRAVE) and chkc:IsAbleToRemove() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_ONFIELD | LOCATION_GRAVE) && card::is_able_to_remove(d, chkc, None, None, None);
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return Duel.IsExistingTarget(Card.IsAbleToRemove,tp,LOCATION_ONFIELD|LOCATION_GRAVE,LOCATION_ONFIELD|LOCATION_GRAVE,1,nil)
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, LOCATION_ONFIELD | LOCATION_GRAVE, LOCATION_ONFIELD | LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToRemove,tp,LOCATION_ONFIELD|LOCATION_GRAVE,LOCATION_ONFIELD|LOCATION_GRAVE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, LOCATION_ONFIELD | LOCATION_GRAVE, LOCATION_ONFIELD | LOCATION_GRAVE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
    true
}
/// `s.rmop(e,tp,...)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // --Your monsters cannot attack directly this turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // aux.RegisterClientHint(c,0,tp,1,0,aux.Stringid(id,2))
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // if c:GetMaterial():IsExists(Card.IsType,1,nil,TYPE_FUSION|TYPE_SYNCHRO|TYPE_XYZ|TYPE_LINK,c,SUMMON_TYPE_LINK,e:GetHandlerPlayer()) then
    let hp = effect::get_handler_player(d, e);
    let mat = card::get_material(d, c);
    if group::is_exists(d, &mat, &|d, mc| card::is_type_ctx(d, mc, TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ | TYPE_LINK, Some(c), SUMMON_TYPE_LINK as u64, Some(hp)), 1, NO_EX) {
        // e:GetLabelObject():SetLabel(1)
        let lo = effect::get_label_object(d, e).as_effect().nil();
        effect::set_label(d, lo, &[1]);
    }
    // (the function returns nothing: nil, which the core reads as 0)
    0
}
/// `s.tmprmfilter(c,e)`
fn tmprmfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:IsCanBeEffectTarget(e) and c:IsAbleToRemove()
    card::is_faceup(d, c) && card::is_can_be_effect_target(d, c, Some(e)) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(Card.IsControler,1,nil,tp)
    (group::is_exists(d, sg, &|d, c| card::is_controler(d, c, tp), 1, NO_EX), false)
}
/// `s.tmprmtg(e,tp,...,chk,chkc)`
fn tmprmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.tmprmfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| tmprmfilter(d, c, a.e)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if chk==0 then return #g>=2 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return g.len() >= 2 && matches!(aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 0, a.tp, None, None, None, false), Err(true));
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_REMOVE)
    let tg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 1, a.tp, Some(HINTMSG_REMOVE as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,tg,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(tg), 2, a.tp, 0);
    true
}
/// `s.tmprmop(e,tp,...)`
fn tmprmop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e):Filter(Card.IsAbleToRemove,nil)
    let tg = duel::get_target_cards(d, a.e).nil();
    let tg = group::filter(d, &tg, &|d, c| card::is_able_to_remove(d, c, None, None, None), NO_EX);
    // if #tg==2 then
    if tg.len() == 2 {
        // aux.RemoveUntil(tg,nil,REASON_EFFECT,PHASE_END,id,e,tp,aux.DefaultFieldReturnOp)
        aux::remove_until(d, tg, None, REASON_EFFECT, PHASE_END, ID, a.e, a.tp, Some(aux::default_field_return()), None, None, None, None, None);
    }
}
