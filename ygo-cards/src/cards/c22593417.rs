//! Topologic Gumblar Dragon
//! トポロジック・ガンブラー・ドラゴン
use super::prelude::*;

pub const ID: u32 = 22593417;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))), 2, None, None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // discard when a monster is summoned
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_HANDES);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.hdcon)
    effect::set_condition(d, e1, Some(condition(hdcon)));
    // e1:SetTarget(s.hdtg)
    effect::set_target(d, e1, Some(cost_target(hdtg)));
    // e1:SetOperation(s.hdop)
    effect::set_operation(d, e1, Some(operation(hdop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // discard (ignition)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_HANDES+CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_HANDES + CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.hdcon2)
    effect::set_condition(d, e2, Some(condition(hdcon2)));
    // e2:SetTarget(s.hdtg2)
    effect::set_target(d, e2, Some(cost_target(hdtg2)));
    // e2:SetOperation(s.hdop2)
    effect::set_operation(d, e2, Some(operation(hdop2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c,ec)`
fn cfilter(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    // if c:IsLocation(LOCATION_MZONE) then
    if card::is_location(d, c, LOCATION_MZONE) {
        // return ec:GetLinkedGroup():IsContains(c)
        card::get_linked_group(d, ec).contains(c)
    } else {
        // return (ec:GetLinkedZone(c:GetPreviousControler())&(1<<c:GetPreviousSequence()))~=0
        let pcp = card::get_previous_controler(d, c);
        let zone = card::get_linked_zone(d, ec, Some(pcp));
        let pseq = card::get_previous_sequence(d, c);
        (zone & (1u32 << pseq)) != 0
    }
}
/// `s.hdcon(e,tp,eg,ep,ev,re,r,rp)`
fn hdcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg();
    // if eg:IsContains(e:GetHandler()) then return false end
    let h = effect::get_handler(d, a.e).nil();
    if eg.contains(h) {
        return false;
    }
    // for tc in aux.Next(Duel.GetMatchingGroup(Card.IsType,tp,LOCATION_MZONE,LOCATION_MZONE,nil,TYPE_LINK)) do
    let mg = duel::get_matching_group(d, Some(&|d, c| card::is_type(d, c, TYPE_LINK)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    for tc in group::iter(d, &mg) {
        // if eg:IsExists(s.cfilter,1,nil,tc) then return true end
        if group::is_exists(d, &eg, &|d, c| cfilter(d, c, tc), 1, NO_EX) {
            return true;
        }
    }
    // return false
    false
}
/// `s.hdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn hdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local rt1=math.min(Duel.GetMatchingGroupCount(Card.IsDiscardable,tp,LOCATION_HAND,0,nil,REASON_EFFECT),2)
    let rt1 = duel::get_matching_group_count(d, Some(&|d, c| card::is_discardable(d, c, Some(REASON_EFFECT), None)), tp, LOCATION_HAND, 0, NO_EX).min(2);
    // if rt1>0 then
    if rt1 > 0 {
        // local rt2=math.min(Duel.GetMatchingGroupCount(Card.IsDiscardable,tp,0,LOCATION_HAND,nil,REASON_EFFECT),2)
        let rt2 = duel::get_matching_group_count(d, Some(&|d, c| card::is_discardable(d, c, Some(REASON_EFFECT), None)), tp, 0, LOCATION_HAND, NO_EX).min(2);
        // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,PLAYER_ALL,math.min(rt1,rt2))
        duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, PLAYER_ALL as u8, rt1.min(rt2) as i32);
    }
    true
}
/// `s.hdop(e,tp,eg,ep,ev,re,r,rp)`
fn hdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=Duel.GetMatchingGroupCount(Card.IsDiscardable,tp,LOCATION_HAND,0,nil,REASON_EFFECT)
    let ct = duel::get_matching_group_count(d, Some(&|d, c| card::is_discardable(d, c, Some(REASON_EFFECT), None)), tp, LOCATION_HAND, 0, NO_EX);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // local sel
    // if ct<2 then
    let sel = if ct < 2 {
        // sel=1
        1
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NUMBER)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NUMBER as u64);
        // sel=Duel.AnnounceNumber(tp,1,2)
        duel::announce_number(d, tp, &[1, 2]).nil().0 as usize
    };
    // local g1=Duel.GetMatchingGroup(Card.IsDiscardable,tp,LOCATION_HAND,0,nil,REASON_EFFECT):RandomSelect(tp,sel)
    let g1m = duel::get_matching_group(d, Some(&|d, c| card::is_discardable(d, c, Some(REASON_EFFECT), None)), tp, LOCATION_HAND, 0, NO_EX);
    let g1 = group::random_select(d, &g1m, tp, sel);
    // local rt=Duel.SendtoGrave(g1,REASON_EFFECT|REASON_DISCARD)
    let rt = duel::send_to_grave(d, g1, REASON_EFFECT | REASON_DISCARD, None, None);
    // if rt==0 or rt>Duel.GetMatchingGroupCount(Card.IsDiscardable,tp,0,LOCATION_HAND,nil,REASON_EFFECT) then return end
    if rt == 0 || rt as usize > duel::get_matching_group_count(d, Some(&|d, c| card::is_discardable(d, c, Some(REASON_EFFECT), None)), tp, 0, LOCATION_HAND, NO_EX) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g2=Duel.GetMatchingGroup(Card.IsDiscardable,tp,0,LOCATION_HAND,nil,REASON_EFFECT):Select(1-tp,rt,rt,nil)
    let g2m = duel::get_matching_group(d, Some(&|d, c| card::is_discardable(d, c, Some(REASON_EFFECT), None)), tp, 0, LOCATION_HAND, NO_EX);
    let g2 = group::select(d, &g2m, 1 - tp, rt as u16, rt as u16, false, NO_EX).nil();
    // Duel.SendtoGrave(g2,REASON_EFFECT|REASON_DISCARD)
    duel::send_to_grave(d, g2, REASON_EFFECT | REASON_DISCARD, None, None);
}
/// `s.hdcon2(e)`
fn hdcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsExtraLinked()
    let h = effect::get_handler(d, a.e).nil();
    card::is_extra_linked(d, h)
}
/// `s.hdtg2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn hdtg2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetFieldGroupCount(1-tp,LOCATION_HAND,0)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, 1 - tp, LOCATION_HAND, 0) > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,1-tp,2)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - tp, 2);
    true
}
/// `s.hdop2(e,tp,eg,ep,ev,re,r,rp)`
fn hdop2(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=2
    let mut ct = 2;
    // if Duel.GetFieldGroupCount(1-tp,LOCATION_HAND,0)==1 then ct=1 end
    if duel::get_field_group_count(d, 1 - tp, LOCATION_HAND, 0) == 1 {
        ct = 1;
    }
    // if Duel.DiscardHand(1-tp,nil,ct,ct,REASON_EFFECT|REASON_DISCARD)~=0
    //     and Duel.GetFieldGroupCount(1-tp,LOCATION_HAND,0)==0 then
    if duel::discard_hand(d, 1 - tp, None, ct, ct, REASON_EFFECT | REASON_DISCARD, NO_EX) != 0 && duel::get_field_group_count(d, 1 - tp, LOCATION_HAND, 0) == 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Damage(1-tp,3000,REASON_EFFECT)
        duel::damage(d, 1 - tp, 3000, REASON_EFFECT, false, None);
    }
}
