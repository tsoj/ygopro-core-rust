//! Unity
use super::prelude::*;

pub const ID: u32 = 14731897;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DEFCHANGE)
    effect::set_category(d, e1, CATEGORY_DEFCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    // e1:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsDefenseAbove(0)
    card::is_faceup(d, c) && card::is_defense_above(d, c, 0)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, 0, NO_EX);
        // local def=0
        let mut def: i64 = 0;
        // for sc in g:Iter() do
        for sc in group::iter(d, &g) {
            // local cdef=sc:GetBaseDefense()
            let mut cdef = card::get_base_defense(d, sc) as i64;
            // if cdef<0 then cdef=0 end
            if cdef < 0 {
                cdef = 0;
            }
            // def=def+cdef
            def += cdef;
        }
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_DEFENSE)
        effect::set_code(d, e1, EFFECT_SET_DEFENSE);
        // e1:SetValue(def)
        effect::set_value(d, e1, def);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
