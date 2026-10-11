//! Genex Ally Triarm
use super::prelude::*;

pub const ID: u32 = 17760003;

pub const INFO: ScriptInfo = ScriptInfo { material: &[68505803], listed_names: &[68505803], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    // Synchro.AddProcedure(c,aux.FilterSummonCode(68505803),1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, Some(aux::filter_summon_code(&[68505803])), 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // material verification check
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e1, EFFECT_MATERIAL_CHECK);
    // e1:SetValue(s.valcheck)
    effect::set_value_fn(d, e1, effect_card_value(valcheck));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // effect registration
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCondition(s.regcon)
    effect::set_condition(d, e2, Some(condition(regcon)));
    // e2:SetOperation(s.regop)
    effect::set_operation(d, e2, Some(operation(regop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
}
/// `s.valcheck(e,c)`: a value function that returns nothing (read as 0)
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // local att=0
    let mut att: u32 = 0;
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // if not tc:IsCode(68505803) or not tc:IsType(TYPE_TUNER) then
        if !card::is_code(d, tc, &[68505803]) || !card::is_type(d, tc, TYPE_TUNER) {
            // att=(att|tc:GetAttribute())
            att |= card::get_attribute(d, tc);
        }
    }
    // att=att&(ATTRIBUTE_WIND|ATTRIBUTE_WATER|ATTRIBUTE_DARK)
    att &= ATTRIBUTE_WIND | ATTRIBUTE_WATER | ATTRIBUTE_DARK;
    // e:SetLabel(att)
    effect::set_label(d, e, &[att as i64]);
    0
}
/// `s.regcon(e,tp,eg,ep,ev,re,r,rp)`
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsSynchroSummoned() and e:GetLabelObject():GetLabel()~=0
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_synchro_summoned(d, h) {
        return false;
    }
    let label_obj = effect::get_label_object(d, a.e).as_effect().nil();
    effect::get_label(d, label_obj) != 0
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local att=e:GetLabelObject():GetLabel()
    let label_obj = effect::get_label_object(d, a.e).as_effect().nil();
    let att = effect::get_label(d, label_obj) as u32;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if (att&ATTRIBUTE_WIND)~=0 then
    if (att & ATTRIBUTE_WIND) != 0 {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,0))
        effect::set_description(d, e1, aux::stringid(ID, 0));
        // e1:SetType(EFFECT_TYPE_IGNITION)
        effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
        effect::set_count_limit(d, e1, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
        // e1:SetCost(s.cost)
        effect::set_cost(d, e1, Some(cost_target(cost_fn)));
        // e1:SetTarget(s.target1)
        effect::set_target(d, e1, Some(cost_target(target1)));
        // e1:SetOperation(s.operation1)
        effect::set_operation(d, e1, Some(operation(operation1)));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // c:RegisterFlagEffect(0,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,3))
        card::register_flag_effect(d, c, 0, RESET_EVENT + RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 3));
    }
    // if (att&ATTRIBUTE_WATER)~=0 then
    if (att & ATTRIBUTE_WATER) != 0 {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,1))
        effect::set_description(d, e1, aux::stringid(ID, 1));
        // e1:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, e1, CATEGORY_DESTROY);
        // e1:SetType(EFFECT_TYPE_IGNITION)
        effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
        // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
        effect::set_count_limit(d, e1, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
        // e1:SetCost(s.cost)
        effect::set_cost(d, e1, Some(cost_target(cost_fn)));
        // e1:SetTarget(s.target2)
        effect::set_target(d, e1, Some(cost_target(target2)));
        // e1:SetOperation(s.operation2)
        effect::set_operation(d, e1, Some(operation(operation2)));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // c:RegisterFlagEffect(0,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,4))
        card::register_flag_effect(d, c, 0, RESET_EVENT + RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 4));
    }
    // if (att&ATTRIBUTE_DARK)~=0 then
    if (att & ATTRIBUTE_DARK) != 0 {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,2))
        effect::set_description(d, e1, aux::stringid(ID, 2));
        // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DRAW)
        effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DRAW);
        // e1:SetType(EFFECT_TYPE_IGNITION)
        effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
        // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
        effect::set_count_limit(d, e1, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
        // e1:SetCost(s.cost)
        effect::set_cost(d, e1, Some(cost_target(cost_fn)));
        // e1:SetTarget(s.target3)
        effect::set_target(d, e1, Some(cost_target(target3)));
        // e1:SetOperation(s.operation3)
        effect::set_operation(d, e1, Some(operation(operation3)));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // c:RegisterFlagEffect(0,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,5))
        card::register_flag_effect(d, c, 0, RESET_EVENT + RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 5));
    }
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), tp, LOCATION_HAND, 0, 1, h.into());
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.target1(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target1(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, tp, 0, LOCATION_HAND) > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,1-tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, 1 - tp, LOCATION_HAND as i32);
    true
}
/// `s.operation1(e,tp,eg,ep,ev,re,r,rp)`
fn operation1(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
    let g = duel::get_field_group(d, tp, 0, LOCATION_HAND);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local sg=g:RandomSelect(tp,1)
    let sg = group::random_select(d, &g, tp, 1);
    // Duel.SendtoGrave(sg,REASON_EFFECT)
    duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
}
/// `s.target2(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsSpellTrap() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_spell_trap(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsSpellTrap,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_spell_trap(d, c)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,Card.IsSpellTrap,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_spell_trap(d, c)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
/// `s.operation2(e,tp,eg,ep,ev,re,r,rp)`
fn operation2(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
/// `s.filter3(c)`
fn filter3(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttribute(ATTRIBUTE_LIGHT)
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_LIGHT)
}
/// `s.target3(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target3(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.filter3(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter3(d, chkc);
    }
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1)
    //     and Duel.IsExistingTarget(s.filter3,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 1) && duel::is_existing_target(d, Some(&filter3), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.filter3,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&filter3), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
/// `s.operation3(e,tp,eg,ep,ev,re,r,rp)`
fn operation3(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, tp, 1, REASON_EFFECT);
    }
}
