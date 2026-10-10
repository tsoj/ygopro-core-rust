//! Union Driver
use super::prelude::*;

pub const ID: u32 = 99249638;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Union procedure
    // aux.AddUnionProcedure(c)
    aux::add_union_procedure(d, c, None, false, None);
    // Equip 1 appropriate Union monster from your Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():GetEquipTarget() end)
    effect::set_condition(d, e1, Some(lua_fn(eqcon)));
    // e1:SetCost(s.eqcost)
    effect::set_cost(d, e1, Some(cost_target(eqcost)));
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `function(e) return e:GetHandler():GetEquipTarget() end`
fn eqcon(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::get_equip_target(d, h).is_some()
}
/// `s.eqcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn eqcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemoveAsCost() end
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, c, None);
    }
    // e:SetLabelObject(c:GetEquipTarget())
    let target = card::get_equip_target(d, c);
    effect::set_label_object(d, a.e, Value::card(target));
    // Duel.Remove(c,POS_FACEUP,REASON_COST)
    duel::remove(d, c, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.eqfilter(c,ec,ign_ct)` (`aux.CheckUnionEquip` ignores the third argument)
fn eqfilter(d: &mut Duel, c: CardId, ec: CardId, _ign_ct: i64) -> bool {
    // return c:IsLevelBelow(4) and c:IsType(TYPE_UNION) and c:CheckUnionTarget(ec) and aux.CheckUnionEquip(c,ec,ign_ct)
    card::is_level_below(d, c, 4) && card::is_type(d, c, TYPE_UNION) && card::check_union_target(d, c, ec) && aux::check_union_equip(d, c, ec)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.eqfilter,tp,LOCATION_DECK,0,1,nil,e:GetHandler():GetEquipTarget(),1) end
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        let ec = card::get_equip_target(d, c).nil();
        return duel::is_existing_matching_card(d, Some(&|d, x| eqfilter(d, x, ec, 1)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // e:GetLabelObject():CreateEffectRelation(e)
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    card::create_effect_relation(d, tc, a.e);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and Duel.GetLocationCount(tp,LOCATION_SZONE)>0 then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
        // local ec=Duel.SelectMatchingCard(tp,s.eqfilter,tp,LOCATION_DECK,0,1,1,nil,tc,0):GetFirst()
        let g = duel::select_matching_card(d, a.tp, Some(&|d, c| eqfilter(d, c, tc, 0)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let ec = group::get_first(d, &g);
        // if ec and aux.CheckUnionEquip(ec,tc) and Duel.Equip(tp,ec,tc) then
        if let Some(ec) = ec {
            if aux::check_union_equip(d, ec, tc) && duel::equip(d, a.tp, ec, tc, true, false) {
                // aux.SetUnionState(ec)
                aux::set_union_state(d, ec);
            }
        }
    }
}
