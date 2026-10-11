//! Bone Temple Block
use super::prelude::*;

pub const ID: u32 = 47778083;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.filter(c,e,tp)
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(4) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_below(d, c, 4) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // return Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)~=0
        //     and Duel.IsExistingTarget(s.filter,tp,0,LOCATION_GRAVE,1,nil,e,tp)
        //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and Duel.IsExistingTarget(s.filter,1-tp,0,LOCATION_GRAVE,1,nil,e,1-tp)
        //     and Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0
        return duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) != 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, 0, LOCATION_GRAVE, 1, NO_EX)
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.e, 1 - a.tp)), 1 - a.tp, 0, LOCATION_GRAVE, 1, NO_EX)
            && duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sg=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_GRAVE,1,1,nil,e,tp)
    let sg = duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_SPSUMMON as u64);
    // local og=Duel.SelectTarget(1-tp,s.filter,1-tp,0,LOCATION_GRAVE,1,1,nil,e,1-tp)
    let og = duel::select_target(d, 1 - a.tp, Some(&|d, c| filter(d, c, a.e, 1 - a.tp)), 1 - a.tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
    // local sc=sg:GetFirst()
    let sc = group::get_first(d, &sg).nil();
    // local oc=og:GetFirst()
    let oc = group::get_first(d, &og).nil();
    // local g=Group.FromCards(sc,oc)
    let mut g = CardSet::from_card(sc);
    group::add_card(&mut g, &CardSet::from_card(oc));
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,2,PLAYER_ALL,1-g:GetFirst():GetOwner())
    let first = group::get_first(d, &g).nil();
    let param = 1 - card::get_owner(d, first) as i32;
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 2, PLAYER_ALL as u8, param);
    // e:SetLabelObject(sc)
    effect::set_label_object(d, a.e, Value::Card(sc));
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // if Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_EFFECT|REASON_DISCARD)==0 then return end
    if duel::discard_hand(d, a.tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX) == 0 {
        return;
    }
    // local sc=e:GetLabelObject()
    let sc = effect::get_label_object(d, a.e).as_card().nil();
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]).remove(0).as_group().cloned().nil();
    // local oc=g:GetFirst()
    let order = group::iter(d, &g);
    let mut oc = order.first().copied();
    // if oc==sc then oc=g:GetNext() end
    if oc == Some(sc) {
        oc = order.get(1).copied();
    }
    // if sc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, sc, a.e) {
        // Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        // e1:SetCondition(s.descon)
        effect::set_condition(d, e1, Some(condition(descon)));
        // e1:SetOperation(s.desop)
        effect::set_operation(d, e1, Some(operation(desop)));
        // e1:SetReset(RESETS_STANDARD_PHASE_END,2)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 2);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetLabel(Duel.GetTurnCount())
        let turn = duel::get_turn_count(d, None) as i64;
        effect::set_label(d, e1, &[turn]);
        // sc:RegisterEffect(e1,true)
        card::register_effect(d, sc, e1, true);
    }
    // if oc:IsRelateToEffect(e) then
    let oc = oc.nil();
    if card::is_relate_to_effect(d, oc, a.e) {
        // Duel.SpecialSummonStep(oc,0,1-tp,1-tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, oc, 0, 1 - a.tp, 1 - a.tp, false, false, POS_FACEUP as u8, None);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        // e1:SetCondition(s.descon)
        effect::set_condition(d, e1, Some(condition(descon)));
        // e1:SetOperation(s.desop)
        effect::set_operation(d, e1, Some(operation(desop)));
        // e1:SetReset(RESETS_STANDARD_PHASE_END,2)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 2);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetLabel(Duel.GetTurnCount())
        let turn = duel::get_turn_count(d, None) as i64;
        effect::set_label(d, e1, &[turn]);
        // oc:RegisterEffect(e1,true)
        card::register_effect(d, oc, e1, true);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
// function s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetTurnCount()~=e:GetLabel()
    duel::get_turn_count(d, None) as i64 != effect::get_label(d, a.e)
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::destroy(d, h, REASON_EFFECT, None, None);
}
