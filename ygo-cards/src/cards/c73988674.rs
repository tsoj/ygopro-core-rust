//! Tri-and-Guess
use super::prelude::*;

pub const ID: u32 = 73988674;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetMatchingGroupCount(Card.IsFacedown,tp,LOCATION_EXTRA,0,nil)>0
    //     and Duel.GetMatchingGroupCount(Card.IsFacedown,tp,0,LOCATION_EXTRA,nil)>0 end
    if a.chk == Some(0) {
        return duel::get_matching_group_count(d, Some(&|d, c| card::is_facedown(d, c)), a.tp, LOCATION_EXTRA, 0, NO_EX) > 0
            && duel::get_matching_group_count(d, Some(&|d, c| card::is_facedown(d, c)), a.tp, 0, LOCATION_EXTRA, NO_EX) > 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CARDTYPE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CARDTYPE as u64);
    // local op=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1),aux.Stringid(id,2))
    let op = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1), aux::stringid(ID, 2)]);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local g1=Duel.GetFieldGroup(tp,LOCATION_EXTRA,0)
    let g1 = duel::get_field_group(d, a.tp, LOCATION_EXTRA, 0);
    // Duel.ConfirmCards(1-tp,g1)
    duel::confirm_cards(d, 1 - a.tp, g1.clone());
    // local g2=Duel.GetFieldGroup(tp,0,LOCATION_EXTRA)
    let g2 = duel::get_field_group(d, a.tp, 0, LOCATION_EXTRA);
    // Duel.ConfirmCards(tp,g2)
    duel::confirm_cards(d, a.tp, g2.clone());
    // local tpe=0
    // if e:GetLabel()==0 then tpe=TYPE_FUSION
    // elseif e:GetLabel()==1 then tpe=TYPE_SYNCHRO
    // else tpe=TYPE_XYZ end
    let label = effect::get_label(d, a.e);
    let tpe: u32 = if label == 0 {
        TYPE_FUSION
    } else if label == 1 {
        TYPE_SYNCHRO
    } else {
        TYPE_XYZ
    };
    // local ct1=g1:FilterCount(Card.IsType,nil,tpe)
    let ct1 = group::filter_count(d, &g1, &|d, c| card::is_type(d, c, tpe), NO_EX);
    // local ct2=g2:FilterCount(Card.IsType,nil,tpe)
    let ct2 = group::filter_count(d, &g2, &|d, c| card::is_type(d, c, tpe), NO_EX);
    // if ct1>ct2 then
    if ct1 > ct2 {
        // Duel.Recover(tp,3000,REASON_EFFECT)
        duel::recover(d, a.tp, 3000, REASON_EFFECT, false, None);
    // elseif ct1<ct2 then
    } else if ct1 < ct2 {
        // Duel.Recover(1-tp,3000,REASON_EFFECT)
        duel::recover(d, 1 - a.tp, 3000, REASON_EFFECT, false, None);
    }
    // Duel.ShuffleExtra(1-tp)
    duel::shuffle_extra(d, 1 - a.tp);
    // Duel.ShuffleExtra(tp)
    duel::shuffle_extra(d, a.tp);
}
