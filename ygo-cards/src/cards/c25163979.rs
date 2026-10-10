//! World Legacy's Nightmare
use super::prelude::*;

pub const ID: u32 = 25163979;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_KNIGHTMARE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --avoid battle damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e2, EFFECT_AVOID_BATTLE_DAMAGE);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(s.efilter)
    effect::set_target(d, e2, Some(effect_card_fn(efilter)));
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --move
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.mvtg)
    effect::set_target(d, e3, Some(cost_target(mvtg)));
    // e3:SetOperation(s.mvop)
    effect::set_operation(d, e3, Some(operation(mvop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.efilter(e,c)
fn efilter(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:GetMutualLinkedGroupCount()>0
    card::get_mutual_linked_group_count(d, c) > 0
}
// function s.mvfilter1(c)
fn mvfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_KNIGHTMARE)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_KNIGHTMARE as u16])
}
// function s.mvfilter2(c,tp)
fn mvfilter2(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_KNIGHTMARE) and c:GetSequence()<5
    //   and Duel.IsExistingMatchingCard(s.mvfilter3,tp,LOCATION_MZONE,0,1,c)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_KNIGHTMARE as u16])
        && card::get_sequence(d, c) < 5
        && duel::is_existing_matching_card(d, Some(&mvfilter3), tp, LOCATION_MZONE, 0, 1, c.into())
}
// function s.mvfilter3(c)
fn mvfilter3(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_KNIGHTMARE) and c:GetSequence()<5
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_KNIGHTMARE as u16]) && card::get_sequence(d, c) < 5
}
// function s.mvtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn mvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local b1=Duel.IsExistingMatchingCard(s.mvfilter1,tp,LOCATION_MZONE,0,1,nil)
    //   and Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_CONTROL)>0
    let b1 = duel::is_existing_matching_card(d, Some(&mvfilter1), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, Some(a.tp), Some(LOCATION_REASON_CONTROL), None).0 > 0;
    // local b2=Duel.IsExistingMatchingCard(s.mvfilter2,tp,LOCATION_MZONE,0,1,nil,tp)
    let tp = a.tp;
    let b2 = duel::is_existing_matching_card(d, Some(&|d, c| mvfilter2(d, c, tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=0
    // if b1 and b2 then op=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))
    let op = if b1 && b2 {
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)])
    }
    // elseif b1 then op=Duel.SelectOption(tp,aux.Stringid(id,1))
    else if b1 {
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1)])
    }
    // else op=Duel.SelectOption(tp,aux.Stringid(id,2))+1 end
    else {
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 2)]) + 1
    };
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    true
}
// function s.mvop(e,tp,eg,ep,ev,re,r,rp)
fn mvop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // if e:GetLabel()==0 then
    if effect::get_label(d, a.e) == 0 {
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 0));
        // local g=Duel.SelectMatchingCard(tp,s.mvfilter1,tp,LOCATION_MZONE,0,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&mvfilter1), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 and not g:GetFirst():IsImmuneToEffect(e) then
        if !g.is_empty() {
            let first = group::get_first(d, &g).nil();
            if !card::is_immune_to_effect(d, first, a.e) {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOZONE)
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOZONE as u64);
                // local s=Duel.SelectDisableField(tp,1,LOCATION_MZONE,0,0)
                let s = duel::select_disable_field(d, a.tp, 1, LOCATION_MZONE, 0, Some(0), false);
                // local nseq=math.log(s,2)
                let nseq = s.trailing_zeros();
                // Duel.MoveSequence(g:GetFirst(),nseq)
                let first = group::get_first(d, &g).nil();
                duel::move_sequence(d, first, nseq as u8, None);
            }
        }
    }
    // else
    else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
        // local g1=Duel.SelectMatchingCard(tp,s.mvfilter2,tp,LOCATION_MZONE,0,1,1,nil,tp)
        let tp = a.tp;
        let g1 = duel::select_matching_card(d, a.tp, Some(&|d, c| mvfilter2(d, c, tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        // local tc1=g1:GetFirst()
        let tc1 = group::get_first(d, &g1);
        // if not tc1 then return end
        let Some(tc1) = tc1 else { return };
        // Duel.HintSelection(g1)
        duel::hint_selection(d, g1.clone(), true);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
        // local g2=Duel.SelectMatchingCard(tp,s.mvfilter3,tp,LOCATION_MZONE,0,1,1,tc1)
        let g2 = duel::select_matching_card(d, a.tp, Some(&mvfilter3), a.tp, LOCATION_MZONE, 0, 1, 1, false, tc1.into()).nil();
        // Duel.HintSelection(g2)
        duel::hint_selection(d, g2.clone(), true);
        // local tc2=g2:GetFirst()
        let tc2 = group::get_first(d, &g2).nil();
        // Duel.SwapSequence(tc1,tc2)
        duel::swap_sequence(d, tc1, tc2);
    }
}
