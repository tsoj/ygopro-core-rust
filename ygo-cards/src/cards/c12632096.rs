//! Neo Galaxy-Eyes Cipher Dragon
use super::prelude::*;

pub const ID: u32 = 12632096;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CIPHER], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 3 Level 9 monsters
    // Xyz.AddProcedure(c,nil,9,3)
    xyz::add_procedure(d, c, None, Some(9), 3, None, None, None, None, false, None);
    // Monsters you control cannot attack your opponent directly for the rest of this turn, except this card,
    // also for each Material detached, take control of 1 opponent's face-up monster until the End Phase
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_CONTROL);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():GetOverlayGroup():IsExists(Card.IsSetCard,1,nil,SET_CIPHER) end)
    effect::set_condition(
        d,
        e1,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            let og = card::get_overlay_group(d, h);
            group::is_exists(d, &og, &|d, x| card::is_set_card(d, x, &[SET_CIPHER as u16]), 1, NO_EX)
        })),
    );
    // e1:SetCost(Cost.DetachFromSelf(1,s.ctcostmax,function(e,og) e:SetLabel(#og) end))
    effect::set_cost(
        d,
        e1,
        Some(cost::detach_from_self(
            cost::Count::N(1),
            Some(cost::Count::F(std::sync::Arc::new(ctcostmax))),
            Some(std::sync::Arc::new(|d: &mut Duel, e: EffectId, og: &CardSet| effect::set_label(d, e, &[og.len() as i64]))),
        )),
    );
    // e1:SetTarget(s.cttg)
    effect::set_target(d, e1, Some(cost_target(cttg)));
    // e1:SetOperation(s.ctop)
    effect::set_operation(d, e1, Some(operation(ctop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.ctcostmax(e,tp)`
fn ctcostmax(d: &mut Duel, _e: EffectId, tp: u8) -> i64 {
    // local ct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsAbleToChangeControler),tp,0,LOCATION_MZONE,nil)
    let ct = duel::get_matching_group_count(d, Some(&|d, x| card::is_faceup(d, x) && card::is_able_to_change_controler(d, x)), tp, 0, LOCATION_MZONE, NO_EX) as i64;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_CONTROL)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, Some(tp), Some(LOCATION_REASON_CONTROL), None).0 as i64;
    // return math.min(ct,ft,3)
    ct.min(ft).min(3)
}
/// `s.cttg(e,tp,...,chk)`
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,nil,e:GetLabel(),tp,0)
    let label = effect::get_label(d, a.e);
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, None, label as u8, a.tp, 0);
    true
}
/// `s.ctop(e,tp,...)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local fid=c:IsRelateToEffect(e) and c:GetFieldID() or -1
    let fid: i64 = if card::is_relate_to_effect(d, c, a.e) { card::get_field_id(d, c) as i64 } else { -1 };
    // --Monsters you control cannot attack your opponent directly for the rest of this turn, except this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(function(e,c) return fid~=c:GetFieldID() end)
    effect::set_target(d, e1, Some(effect_card_fn(move |d, _e, x| fid != card::get_field_id(d, x) as i64)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // --For each Material detached, take control of 1 opponent's face-up monster until the End Phase
    // local ct=math.min(e:GetLabel(),Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_CONTROL))
    let label = effect::get_label(d, a.e);
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, Some(a.tp), Some(LOCATION_REASON_CONTROL), None).0 as i64;
    let ct = label.min(ft) as u16;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.FaceupFilter(Card.IsAbleToChangeControler),tp,0,LOCATION_MZONE,ct,ct,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| card::is_faceup(d, x) && card::is_able_to_change_controler(d, x)), a.tp, 0, LOCATION_MZONE, ct, ct, false, NO_EX).nil();
    // if #g==ct and Duel.GetControl(g,tp,PHASE_END,1) then
    // (GetControl returns a number, which is always true in Lua)
    let full = g.len() == ct as usize;
    if full {
        duel::get_control(d, g, a.tp, PHASE_END as u16, 1, None, None);
        // local og=Duel.GetOperatedGroup()
        let og = duel::get_operated_group(d);
        // if #og==0 then return end
        if og.is_empty() {
            return;
        }
        // for tc in og:Iter() do
        for tc in group::iter(d, &og) {
            // ---Their effects are negated
            // tc:NegateEffects(c,RESET_CONTROL)
            card::negate_effects(d, tc, c, Some(RESET_CONTROL), false, None);
            // --Their ATK's become 4500
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
            effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
            // e1:SetValue(4500)
            effect::set_value(d, e1, 4500);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_CONTROL)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_CONTROL, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // --Their names become "Neo Galaxy-Eyes Cipher Dragon"
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_CHANGE_CODE)
            effect::set_code(d, e2, EFFECT_CHANGE_CODE);
            // e2:SetValue(id)
            effect::set_value(d, e2, ID as i64);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
}
