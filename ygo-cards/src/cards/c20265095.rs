//! Brotherhood of the Fire Fist - Peacock
//! 慧炎星－コサンジャク
use super::prelude::*;

pub const ID: u32 = 20265095;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FIRE_FORMATION, SET_FIRE_FIST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link summon procedure
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_FIRE_FIST),2,2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_FIRE_FIST as u16], sc, st, Some(tp)))), 2, Some(2), None, None, None);
    // Cannot be targeted for attack while pointing to a "Fire Fist" monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_BE_BATTLE_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_BATTLE_TARGET);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.tgcon)
    effect::set_condition(d, e1, Some(condition(tgcon)));
    // e1:SetValue(aux.imval1)
    effect::set_value_fn(d, e1, aux::imval1());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Take control 1 of opponent's monsters until end phase
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e2, CATEGORY_CONTROL);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.Replaceable(s.ctcost,s.extracon))
    effect::set_cost(d, e2, Some(cost::replaceable(cost_target(ctcost), Some(lua_fn(extracon)))));
    // e2:SetTarget(s.cttg)
    effect::set_target(d, e2, Some(cost_target(cttg)));
    // e2:SetOperation(s.ctop)
    effect::set_operation(d, e2, Some(operation(ctop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.lkfilter(c)`
fn lkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_FIRE_FIST) and c:IsMonster()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_FIRE_FIST as u16]) && card::is_monster(d, c)
}
/// `s.tgcon(e,tp,...)`
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetLinkedGroup():IsExists(s.lkfilter,1,nil)
    let h = effect::get_handler(d, a.e).nil();
    let g = card::get_linked_group(d, h);
    group::is_exists(d, &g, &lkfilter, 1, NO_EX)
}
/// `s.ctcostfilter(c,tp,zones)`
fn ctcostfilter(d: &mut Duel, c: CardId, tp: u8, zones: u32) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_FIRE_FORMATION) and c:IsSpellTrap() and c:IsAbleToGraveAsCost()
    //     and Duel.GetMZoneCount(tp,c,tp,LOCATION_REASON_CONTROL,zones)>0
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_FIRE_FORMATION as u16])
        && card::is_spell_trap(d, c)
        && card::is_able_to_grave_as_cost(d, c)
        && duel::get_m_zone_count(d, tp, c.into(), Some(tp), Some(LOCATION_REASON_CONTROL), Some(zones)).0 > 0
}
/// `s.ctcost(e,tp,...,chk)`
fn ctcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local zones=e:GetHandler():GetLinkedZone()&ZONES_MMZ
    let h = effect::get_handler(d, a.e).nil();
    let zones = card::get_linked_zone(d, h, None) & ZONES_MMZ;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.ctcostfilter,tp,LOCATION_ONFIELD,0,1,nil,tp,zones) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| ctcostfilter(d, c, a.tp, zones)), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.ctcostfilter,tp,LOCATION_ONFIELD,0,1,1,nil,tp,zones)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| ctcostfilter(d, c, a.tp, zones)), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.extracon(base,e,tp)`
fn extracon(d: &mut Duel, a: Args) -> bool {
    // local zones=e:GetHandler():GetLinkedZone()&ZONES_MMZ
    let h = effect::get_handler(d, a.e(1)).nil();
    let zones = card::get_linked_zone(d, h, None) & ZONES_MMZ;
    // return Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_CONTROL,zones)>0
    let tp = a.p(2);
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, Some(tp), Some(LOCATION_REASON_CONTROL), Some(zones)).0 > 0
}
/// `s.cttg(e,tp,...,chk,chkc)`
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsControlerCanBeChanged() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && card::is_controler_can_be_changed(d, chkc, false, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsControlerCanBeChanged,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_controler_can_be_changed(d, c, false, None)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectTarget(tp,Card.IsControlerCanBeChanged,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_controler_can_be_changed(d, c, false, None)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 1, 0, 0);
    true
}
/// `s.ctop(e,tp,...)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and tc then
    if card::is_relate_to_effect(d, c, a.e) {
        if let Some(tc) = tc {
            // local zone=c:GetLinkedZone()&ZONES_MMZ
            let zone = card::get_linked_zone(d, c, None) & ZONES_MMZ;
            // if Duel.GetControl(tc,tp,PHASE_END,1,zone)~=0 then
            if duel::get_control(d, tc, a.tp, PHASE_END as u16, 1, Some(zone), None) != 0 {
                // Cannot attack
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetDescription(3206)
                effect::set_description(d, e1, 3206);
                // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
                effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetCode(EFFECT_CANNOT_ATTACK)
                effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
                // e1:SetReset(RESET_EVENT|(RESETS_STANDARD_PHASE_END&~RESET_TURN_SET))
                effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_PHASE_END & !RESET_TURN_SET), 1);
                // tc:RegisterEffect(e1)
                card::register_effect(d, tc, e1, false);
            }
        }
    }
}
