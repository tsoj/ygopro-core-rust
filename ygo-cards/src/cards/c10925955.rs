//! Contact with the Aquamirror
//! 儀水鏡との交信
use super::prelude::*;

pub const ID: u32 = 10925955;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.cfilter(c,rit)
fn cfilter(d: &mut Duel, c: CardId, rit: bool) -> bool {
    // return c:IsFaceup() and c:IsAttribute(ATTRIBUTE_WATER) and (not rit or c:IsType(TYPE_RITUAL))
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_WATER) && (!rit || card::is_type(d, c, TYPE_RITUAL))
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, false)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() and c:GetSequence()~=5
    card::is_facedown(d, c) && card::get_sequence(d, c) != 5
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>1
    //     or Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)>1
    //     or Duel.IsExistingMatchingCard(s.filter,tp,0,LOCATION_SZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 1
            || duel::get_field_group_count(d, a.tp, 0, LOCATION_DECK) > 1
            || duel::is_existing_matching_card(d, Some(&filter), a.tp, 0, LOCATION_SZONE, 1, NO_EX);
    }
    // local sel=0
    let mut sel = 0;
    // local ac=0
    let ac: i32;
    // if Duel.IsExistingMatchingCard(s.filter,tp,0,LOCATION_SZONE,1,nil) then sel=sel+1 end
    if duel::is_existing_matching_card(d, Some(&filter), a.tp, 0, LOCATION_SZONE, 1, NO_EX) {
        sel += 1;
    }
    // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>1 or Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)>1 then sel=sel+2 end
    if duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 1 || duel::get_field_group_count(d, a.tp, 0, LOCATION_DECK) > 1 {
        sel += 2;
    }
    if sel == 1 {
        // ac=Duel.SelectOption(tp,aux.Stringid(id,0))
        ac = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 0)]);
    } else if sel == 2 {
        // ac=Duel.SelectOption(tp,aux.Stringid(id,1))+1
        ac = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1)]) + 1;
    } else if duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, true)), a.tp, LOCATION_MZONE, 0, 1, NO_EX) {
        // ac=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1),aux.Stringid(id,2))
        ac = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1), aux::stringid(ID, 2)]);
    } else {
        // ac=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1))
        ac = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)]);
    }
    // e:SetLabel(ac)
    effect::set_label(d, a.e, &[ac as i64]);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local ac=e:GetLabel()
    let ac = effect::get_label(d, a.e);
    // if ac==0 or ac==2 then
    if ac == 0 || ac == 2 {
        // local g=Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_SZONE,nil)
        let g = duel::get_matching_group(d, Some(&filter), a.tp, 0, LOCATION_SZONE, NO_EX);
        // Duel.ConfirmCards(tp,g)
        duel::confirm_cards(d, a.tp, g);
    }
    // if ac==1 or ac==2 then
    if ac == 1 || ac == 2 {
        // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>1 and Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)>1 then
        if duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 1 && duel::get_field_group_count(d, a.tp, 0, LOCATION_DECK) > 1 {
            // local st=Duel.SelectOption(tp,aux.Stringid(id,3),aux.Stringid(id,4))
            let st = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 3), aux::stringid(ID, 4)]);
            // if st==0 then Duel.SortDecktop(tp,tp,2)
            if st == 0 {
                duel::sort_decktop(d, a.tp, a.tp, 2);
            } else {
                // else Duel.SortDecktop(tp,1-tp,2) end
                duel::sort_decktop(d, a.tp, 1 - a.tp, 2);
            }
        } else if duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 1 {
            // elseif Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>1 then Duel.SortDecktop(tp,tp,2)
            duel::sort_decktop(d, a.tp, a.tp, 2);
        } else if duel::get_field_group_count(d, a.tp, 0, LOCATION_DECK) > 1 {
            // elseif Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)>1 then Duel.SortDecktop(tp,1-tp,2)
            duel::sort_decktop(d, a.tp, 1 - a.tp, 2);
        }
    }
}
