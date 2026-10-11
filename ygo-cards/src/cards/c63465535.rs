//! 地底のアラクネー (Underground Arachnid)
use super::prelude::*;

pub const ID: u32 = 63465535;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 DARK Tuner + 1 non-Tuner Insect monster
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_DARK),1,1,Synchro.NonTunerEx(Card.IsRace,RACE_INSECT),1,1)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_DARK, sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner_ex(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_INSECT, sc, st, Some(tp)))),
        1,
        1,
        None,
        None,
        None,
        None,
        None,
    );
    // If this card attacks, your opponent cannot activate Spells/Traps until the end of the Damage Step
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetCondition(function(e) return Duel.GetAttacker()==e:GetHandler() end)
    effect::set_condition(
        d,
        e1,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            duel::get_attacker(d) == Some(h)
        })),
    );
    // e1:SetValue(function(e,re,tp) return re:IsHasType(EFFECT_TYPE_ACTIVATE) end)
    effect::set_value_fn(d, e1, lua_fn(|d, a: Args| effect::is_has_type(d, a.e(1), EFFECT_TYPE_ACTIVATE)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Equip 1 opponent's face-up monster to this card (only 1 monster can be equipped to this card with this effect)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e2, CATEGORY_EQUIP);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.eqcon)
    effect::set_condition(d, e2, Some(condition(eqcon)));
    // e2:SetTarget(s.eqtg)
    effect::set_target(d, e2, Some(cost_target(eqtg)));
    // e2:SetOperation(s.eqop)
    effect::set_operation(d, e2, Some(operation(eqop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // aux.AddEREquipLimit(c,s.eqcon,function(ec,c,tp) return ec:IsControler(1-tp) end,function(c,e,tp,tc) c:EquipByEffectAndLimitRegister(e,tp,tc,id) end,e2)
    aux::add_er_equip_limit(d, c, Some(condition(eqcon)), lua_fn(eqval), lua_fn(equipop), e2, None, None, None);
    // If this card would be destroyed by battle, you can destroy that equipped monster instead
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e3, EFFECT_DESTROY_REPLACE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTarget(s.desreptg)
    effect::set_target(d, e3, Some(cost_target(desreptg)));
    // e3:SetOperation(s.desrepop)
    effect::set_operation(d, e3, Some(operation(desrepop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(ec,c,tp) return ec:IsControler(1-tp) end`
fn eqval(d: &mut Duel, a: Args) -> bool {
    let ec = a.c(0);
    let tp = a.p(2);
    card::is_controler(d, ec, 1 - tp)
}
/// `function(c,e,tp,tc) c:EquipByEffectAndLimitRegister(e,tp,tc,id) end`
fn equipop(d: &mut Duel, a: Args) -> Value {
    let c = a.c(0);
    let e = a.e(1);
    let tp = a.p(2);
    let tc = a.c(3);
    card::equip_by_effect_and_limit_register(d, c, e, tp, tc, Some(ID), None);
    Value::Nil
}
/// `s.eqcon(e,tp,eg,ep,ev,re,r,rp)`
fn eqcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not e:GetHandler():GetEquipGroup():IsExists(Card.HasFlagEffect,1,nil,id)
    let h = effect::get_handler(d, a.e).nil();
    let eg = card::get_equip_group(d, h);
    !group::is_exists(d, &eg, &|d, c| card::has_flag_effect(d, c, ID, None), 1, NO_EX)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsAbleToChangeControler() and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, 1 - a.tp)
            && card::is_able_to_change_controler(d, chkc)
            && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingTarget(aux.FaceupFilter(Card.IsAbleToChangeControler),tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c) && card::is_able_to_change_controler(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsAbleToChangeControler),tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c) && card::is_able_to_change_controler(d, c)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(g), 1, a.tp, 0);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsControler(1-tp) and tc:IsFaceup() and tc:IsMonster() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_controler(d, tc, 1 - a.tp) && card::is_faceup(d, tc) && card::is_monster(d, tc) {
        // e:GetHandler():EquipByEffectAndLimitRegister(e,tp,tc,id)
        let h = effect::get_handler(d, a.e).nil();
        card::equip_by_effect_and_limit_register(d, h, a.e, a.tp, tc, Some(ID), None);
    }
}
/// `s.desrepfilter(c,e)`
fn desrepfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:HasFlagEffect(id) and c:IsDestructable(e) and not c:IsStatus(STATUS_DESTROY_CONFIRMED)
    card::has_flag_effect(d, c, ID, None) && card::is_destructable(d, c, Some(e)) && !card::is_status(d, c, STATUS_DESTROY_CONFIRMED)
}
/// `s.desreptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsReason(REASON_BATTLE) and c:GetEquipGroup():IsExists(s.desrepfilter,1,nil,e) end
    if a.chk == Some(0) {
        let eg = card::get_equip_group(d, c);
        return card::is_reason(d, c, REASON_BATTLE) && group::is_exists(d, &eg, &|d, x| desrepfilter(d, x, a.e), 1, NO_EX);
    }
    // return Duel.SelectEffectYesNo(tp,c,96)
    duel::select_effect_yes_no(d, a.tp, c, Some(96))
}
/// `s.desrepop(e,tp,eg,ep,ev,re,r,rp)`
fn desrepop(d: &mut Duel, a: &EffArgs) {
    // local ec=e:GetHandler():GetEquipGroup():Match(s.desrepfilter,nil,e)
    let h = effect::get_handler(d, a.e).nil();
    let mut ec = card::get_equip_group(d, h);
    group::match_(d, &mut ec, &|d, x| desrepfilter(d, x, a.e), NO_EX);
    // Duel.Destroy(ec,REASON_EFFECT|REASON_REPLACE)
    duel::destroy(d, ec, REASON_EFFECT | REASON_REPLACE, None, None);
}
