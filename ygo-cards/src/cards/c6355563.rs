//! Y-Yare Head
//! scripted by pyrQ
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 6355563;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Equip 1 Level 4 LIGHT Machine monster from your hand or GY to this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.AddEREquipLimit(c,nil,function(ec,c,tp) return ec:IsControler(tp) and s.eqfilter(ec,tp) end,Card.EquipByEffectAndLimitRegister,e1)
    aux::add_er_equip_limit(
        d,
        c,
        None,
        lua_fn(|d, a| eqval(d, a.c(0), a.p(2))),
        lua_fn(|d, a| card::equip_by_effect_and_limit_register(d, a.c(0), a.e(1), a.p(2), a.c(3), None, None)),
        e1,
        None,
        None,
        None,
    );
    // Union procedure
    // aux.AddUnionProcedure(c,aux.FilterBoolFunction(Card.IsRace,RACE_MACHINE))
    aux::add_union_procedure(d, c, Some(Arc::new(|d: &mut Duel, x: CardId| card::is_race(d, x, RACE_MACHINE))), false, None);
}
/// `s.eqfilter(c,tp)`
fn eqfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsLevel(4) and c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_MACHINE) and c:CheckUniqueOnField(tp)
    //     and not c:IsForbidden()
    card::is_level(d, c, &[4])
        && card::is_attribute(d, c, ATTRIBUTE_LIGHT)
        && card::is_race(d, c, RACE_MACHINE)
        && card::check_unique_on_field(d, c, tp, None, None)
        && !card::is_forbidden(d, c)
}
/// `function(ec,c,tp) return ec:IsControler(tp) and s.eqfilter(ec,tp) end`
fn eqval(d: &mut Duel, ec: CardId, tp: u8) -> bool {
    card::is_controler(d, ec, tp) && eqfilter(d, ec, tp)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingMatchingCard(s.eqfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| eqfilter(d, c, tp)), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)>0 and c:IsRelateToEffect(e) and c:IsFaceup() then
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0 && card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
        // local ec=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.eqfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,tp):GetFirst()
        let g = duel::select_matching_card(
            d,
            tp,
            Some(&aux::necro_valley_filter(|d, x| eqfilter(d, x, tp))),
            tp,
            LOCATION_HAND | LOCATION_GRAVE,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        let ec = group::get_first(d, &g);
        // if ec then
        if let Some(ec) = ec {
            // c:EquipByEffectAndLimitRegister(e,tp,ec,nil,true)
            card::equip_by_effect_and_limit_register(d, c, a.e, tp, ec, None, Some(true));
        }
    }
    // Cannot Special Summon from the Extra Deck for the rest of this turn, except LIGHT monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsAttribute(ATTRIBUTE_LIGHT) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, cc| card::is_location(d, cc, LOCATION_EXTRA) && !card::is_attribute(d, cc, ATTRIBUTE_LIGHT))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // aux.addTempLizardCheck(c,tp,function(e,c) return not c:IsOriginalAttribute(ATTRIBUTE_LIGHT) end)
    aux::add_temp_lizard_check(d, c, tp, Some(effect_card_fn(|d, _e, cc| !card::is_original_attribute(d, cc, ATTRIBUTE_LIGHT))), None, None, None, None);
}
