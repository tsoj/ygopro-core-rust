//! Dark Blade the Captain of the Evil World
use super::prelude::*;

pub const ID: u32 = 57784563;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // equip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.eqcon)
    effect::set_condition(d, e1, Some(condition(eqcon)));
    // e1:SetCost(s.eqcost)
    effect::set_cost(d, e1, Some(cost_target(eqcost)));
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.eqcon(e,tp,eg,ep,ev,re,r,rp)`
fn eqcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=e:GetLabelObject()
    // return ec==nil or ec:GetFlagEffect(id)==0
    match effect::get_label_object(d, a.e).as_card() {
        None => true,
        Some(ec) => card::get_flag_effect(d, ec, ID) == 0,
    }
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.eqcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn eqcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c = e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,e:GetHandler())
    let g = duel::select_matching_card(d, tp, Some(&|d, c| cfilter(d, c)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, c.into()).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsLevelBelow(4) and c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsAbleToChangeControler()
    card::is_faceup(d, c)
        && card::is_level_below(d, c, 4)
        && card::is_attribute(d, c, ATTRIBUTE_LIGHT)
        && card::is_able_to_change_controler(d, c)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::get_controler(d, chkc) == 1 - tp && filter(d, chkc);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingTarget(s.filter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, g, 1, 0, 0);
    true
}
/// `s.eqlimit(e,c)`: `return e:GetOwner()==c`
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    effect::get_owner(d, e) == Some(c)
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsFaceup() and tc:IsRelateToEffect(e) and tc:IsMonster() then
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, e) && card::is_monster(d, tc) {
            // if not Duel.Equip(tp,tc,c,false) then return end
            if !duel::equip(d, tp, tc, c, false, false) {
                return;
            }
            // Add Equip limit
            // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,0)
            card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 0, 0, 0);
            // e:SetLabelObject(tc)
            effect::set_label_object(d, e, Value::Card(tc));
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_EQUIP_LIMIT)
            effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
            // e1:SetProperty(EFFECT_FLAG_COPY_INHERIT+EFFECT_FLAG_OWNER_RELATE)
            effect::set_property(d, e1, EFFECT_FLAG_COPY_INHERIT + EFFECT_FLAG_OWNER_RELATE, 0);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // e1:SetValue(s.eqlimit)
            effect::set_value_fn(d, e1, effect_card_fn(|d, e, c| eqlimit(d, e, c)));
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_EQUIP)
            effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
            // e2:SetProperty(EFFECT_FLAG_OWNER_RELATE+EFFECT_FLAG_IGNORE_IMMUNE)
            effect::set_property(d, e2, EFFECT_FLAG_OWNER_RELATE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
            // e2:SetCode(EFFECT_DESTROY_SUBSTITUTE)
            effect::set_code(d, e2, EFFECT_DESTROY_SUBSTITUTE);
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            // e2:SetValue(1)
            effect::set_value(d, e2, 1);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
}
