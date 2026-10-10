//! Forbidden Apocrypha
use super::prelude::*;

pub const ID: u32 = 3211439;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Make players send monsters to the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local b1=Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsType,TYPE_FUSION),tp,LOCATION_MZONE,LOCATION_MZONE,2,nil)
    let b1 = duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_FUSION)), tp, LOCATION_MZONE, LOCATION_MZONE, 2, NO_EX);
    // local b2=Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsType,TYPE_SYNCHRO),tp,LOCATION_MZONE,LOCATION_MZONE,2,nil)
    let b2 = duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_SYNCHRO)), tp, LOCATION_MZONE, LOCATION_MZONE, 2, NO_EX);
    // local b3=Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsType,TYPE_XYZ),tp,LOCATION_MZONE,LOCATION_MZONE,2,nil)
    let b3 = duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ)), tp, LOCATION_MZONE, LOCATION_MZONE, 2, NO_EX);
    // if chk==0 then return b1 or b2 or b3 end
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CARDTYPE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CARDTYPE as u64);
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)},
    //     {b3,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3))]).unwrap_or(0);
    // local opval={TYPE_FUSION,TYPE_SYNCHRO,TYPE_XYZ}
    let opval = [TYPE_FUSION, TYPE_SYNCHRO, TYPE_XYZ];
    // e:SetLabel(opval[op])
    let label = if (1..=3).contains(&op) { opval[op - 1] as i64 } else { 0 };
    effect::set_label(d, a.e, &[label]);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsType,e:GetLabel()),tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let ty = effect::get_label(d, a.e) as u32;
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c) && card::is_type(d, c, ty)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #g<2 then return end
    if g.len() < 2 {
        return;
    }
    // local g1,g2=g:Split(Card.IsControler,nil,tp)
    let (g1, g2) = group::split(d, &g, &|d, c| card::is_controler(d, c, tp), NO_EX);
    // if #g1>0 then
    if !g1.is_empty() {
        // Duel.SendtoGrave(g1,REASON_RULE,PLAYER_NONE,tp)
        duel::send_to_grave(d, g1, REASON_RULE, Some(PLAYER_NONE as u8), Some(tp));
    }
    // if #g2>0 then
    if !g2.is_empty() {
        // Duel.SendtoGrave(g2,REASON_RULE,PLAYER_NONE,1-tp)
        duel::send_to_grave(d, g2, REASON_RULE, Some(PLAYER_NONE as u8), Some(1 - tp));
    }
}
