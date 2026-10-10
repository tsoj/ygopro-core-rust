//! Beat Cop from the Underworld
use super::prelude::*;

pub const ID: u32 = 99011763;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[0x1049], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,nil,2,2)
    link::add_procedure(d, c, None, 2, Some(2), None, None, None);
    // effect gain
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e1, EFFECT_MATERIAL_CHECK);
    // e1:SetValue(s.matcheck)
    effect::set_value_fn(d, e1, effect_card_value(matcheck));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.matcheck(e,c)`
fn matcheck(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // if #g==2 and g:GetClassCount(Card.GetCode)==#g and not g:IsExists(aux.NOT(Card.IsAttribute),1,nil,ATTRIBUTE_DARK) then
    if g.len() == 2
        && group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) == g.len()
        && !group::is_exists(d, &g, &|d, cc| !card::is_attribute(d, cc, ATTRIBUTE_DARK), 1, NO_EX)
    {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,0))
        effect::set_description(d, e1, aux::stringid(ID, 0));
        // e1:SetCategory(CATEGORY_COUNTER)
        effect::set_category(d, e1, CATEGORY_COUNTER);
        // e1:SetType(EFFECT_TYPE_IGNITION)
        effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
        // e1:SetCountLimit(1,id)
        effect::set_count_limit(d, e1, 1, ID, 0, 0);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
        // e1:SetCost(s.addcost)
        effect::set_cost(d, e1, Some(cost_target(addcost)));
        // e1:SetTarget(s.addtg)
        effect::set_target(d, e1, Some(cost_target(addtg)));
        // e1:SetOperation(s.addop)
        effect::set_operation(d, e1, Some(operation(addop)));
        // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
    // (the Lua function returns nothing: the core reads 0)
    0
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return Duel.IsExistingTarget(Card.IsCanAddCounter,0,LOCATION_ONFIELD,LOCATION_ONFIELD,1,c,0x1049,1)
    duel::is_existing_target(d, Some(&|d, cc| card::is_can_add_counter(d, cc, 0x1049, 1, false, 0)), 0, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into())
}
/// `s.addcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn addcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,nil,nil) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&cfilter), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,nil,nil)
    let g = duel::select_release_group_cost(d, a.tp, Some(&cfilter), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.addtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn addtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_COUNTER)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_COUNTER as u64);
    // Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    true
}
/// `s.addop(e,tp,eg,ep,ev,re,r,rp)`
fn addop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // tc:AddCounter(0x1049,1)
        card::add_counter(d, tc, 0x1049, 1, false);
        // if tc:GetFlagEffect(id)~=0 then return end
        if card::get_flag_effect(d, tc, ID) != 0 {
            return;
        }
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EFFECT_DESTROY_REPLACE)
        effect::set_code(d, e1, EFFECT_DESTROY_REPLACE);
        // e1:SetTarget(s.reptg)
        effect::set_target(d, e1, Some(cost_target(reptg)));
        // e1:SetOperation(s.repop)
        effect::set_operation(d, e1, Some(operation(repop)));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,0)
        card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 0, 0, 0);
    }
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return not e:GetHandler():IsReason(REASON_REPLACE+REASON_RULE) and e:GetHandler():GetCounter(0x1049)>0 end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return !card::is_reason(d, h, REASON_REPLACE + REASON_RULE) && card::get_counter(d, h, 0x1049) > 0;
    }
    // return true
    true
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn repop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():RemoveCounter(tp,0x1049,1,REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    card::remove_counter(d, h, a.tp, 0x1049, 1, REASON_EFFECT);
}
