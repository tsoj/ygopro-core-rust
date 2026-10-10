//! Predaplant Triantis
use super::prelude::*;

pub const ID: u32 = 17825378;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_PREDATOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Allow cards in the Pendulum Zones as fusion materials
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_EXTRA_FUSION_MATERIAL)
    effect::set_code(d, e1, EFFECT_EXTRA_FUSION_MATERIAL);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetTargetRange(LOCATION_PZONE,0)
    effect::set_target_range(d, e1, LOCATION_PZONE, 0);
    // e1:SetValue(function(_,c) return c and c:IsAttribute(ATTRIBUTE_DARK) end)
    effect::set_value_fn(d, e1, effect_card_fn(|d, _e, c| card::is_attribute(d, c, ATTRIBUTE_DARK)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Place Predator Counters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e2, CATEGORY_COUNTER);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
    // e2:SetCondition(s.ctcon)
    effect::set_condition(d, e2, Some(condition(ctcon)));
    // e2:SetTarget(s.cttg)
    effect::set_target(d, e2, Some(cost_target(cttg)));
    // e2:SetOperation(s.ctop)
    effect::set_operation(d, e2, Some(operation(ctop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.ctcon(e,tp,eg,ep,ev,re,r,rp)`
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return (r&REASON_FUSION)==REASON_FUSION and c:IsFaceup()
    //   and c:IsLocation(LOCATION_GRAVE|LOCATION_EXTRA)
    (a.r & REASON_FUSION) == REASON_FUSION && card::is_faceup(d, c) && card::is_location(d, c, LOCATION_GRAVE | LOCATION_EXTRA)
}
/// `s.cttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)>0
    //   and Duel.IsExistingMatchingCard(Card.IsCanAddCounter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,COUNTER_PREDATOR,1) end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0) > 0
            && duel::is_existing_matching_card(
                d,
                Some(&|d, c| card::is_can_add_counter(d, c, COUNTER_PREDATOR as u16, 1, false, 0)),
                a.tp,
                LOCATION_MZONE,
                LOCATION_MZONE,
                1,
                NO_EX,
            );
    }
    false
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // local max=Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)
    let max = duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_COUNTER)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_COUNTER as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsCanAddCounter,tp,LOCATION_MZONE,LOCATION_MZONE,1,max,nil,COUNTER_PREDATOR,1)
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&|d, c| card::is_can_add_counter(d, c, COUNTER_PREDATOR as u16, 1, false, 0)),
        a.tp,
        LOCATION_MZONE,
        LOCATION_MZONE,
        1,
        max as u16,
        false,
        NO_EX,
    )
    .nil();
    // if #g<1 then return end
    if g.is_empty() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // if tc:AddCounter(COUNTER_PREDATOR,1) and tc:GetLevel()>1 then
        if card::add_counter(d, tc, COUNTER_PREDATOR as u16, 1, false) && card::get_level(d, tc) > 1 {
            // --Become Level 1
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CHANGE_LEVEL)
            effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // e1:SetCondition(s.lvcon)
            effect::set_condition(d, e1, Some(condition(lvcon)));
            // e1:SetValue(1)
            effect::set_value(d, e1, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
/// `s.lvcon(e)`
fn lvcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetCounter(COUNTER_PREDATOR)>0
    let h = effect::get_handler(d, a.e).nil();
    card::get_counter(d, h, COUNTER_PREDATOR as u16) > 0
}
