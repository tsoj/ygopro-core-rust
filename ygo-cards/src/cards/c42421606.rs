//! Number 85: Crazy Box
use super::prelude::*;

pub const ID: u32 = 42421606;
pub const INFO: ScriptInfo = ScriptInfo { roll_dice: true, xyz_number: Some(85), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,nil,4,2)
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // prevent itself from attacking
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // roll die
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DICE+CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_DICE + CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e2:SetTarget(s.eftg)
    effect::set_target(d, e2, Some(cost_target(eftg)));
    // e2:SetOperation(s.efop)
    effect::set_operation(d, e2, Some(operation(efop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.eftg(e,tp,eg,ep,ev,re,r,rp,chk)
fn eftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DICE,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, a.tp, 1);
    true
}
// function s.efop(e,tp,eg,ep,ev,re,r,rp)
fn efop(d: &mut Duel, a: &EffArgs) {
    // local dc=Duel.TossDice(tp,1)
    let dc = duel::toss_dice(d, a.tp, 1, 0)[0];
    // if dc==1 then
    if dc == 1 {
        // Duel.SetLP(tp,Duel.GetLP(tp)/2)
        let lp = duel::get_lp(d, a.tp);
        duel::set_lp(d, a.tp, lua_int(lp as f64 / 2.0) as i32);
    // elseif dc==2 then
    } else if dc == 2 {
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, a.tp, 1, REASON_EFFECT);
    // elseif dc==3 then
    } else if dc == 3 {
        // if Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)>0 then
        if duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) > 0 {
            // Duel.DiscardHand(1-tp,nil,1,1,REASON_EFFECT|REASON_DISCARD)
            duel::discard_hand(d, 1 - a.tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
        }
    // elseif dc==4 then
    } else if dc == 4 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
        // local g=Duel.SelectMatchingCard(tp,Card.IsFaceup,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // local tc=g:GetFirst()
            let tc = group::get_first(d, &g).nil();
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e1, EFFECT_DISABLE);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=Effect.CreateEffect(e:GetHandler())
            let e2 = effect::create_effect(d, h);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e2:SetCode(EFFECT_DISABLE_EFFECT)
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            // e2:SetValue(RESET_TURN_SET)
            effect::set_value(d, e2, RESET_TURN_SET as i64);
            // e2:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    // elseif dc==5 then
    } else if dc == 5 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&|_d, _c| true), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // Duel.Destroy(g,REASON_EFFECT)
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    // else
    } else {
        // if e:GetHandler():IsRelateToEffect(e) then
        let h = effect::get_handler(d, a.e).nil();
        if card::is_relate_to_effect(d, h, a.e) {
            // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
            duel::destroy(d, CardSet::from_card(h), REASON_EFFECT, None, None);
        }
    }
}
