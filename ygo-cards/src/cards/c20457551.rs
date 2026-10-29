//! Iron Core Armor
//! 鋼核収納
use super::prelude::*;

pub const ID: u32 = 20457551;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_KOAKI_MEIRU],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsSetCard,SET_KOAKI_MEIRU))
    aux::add_equip_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_set_card(d, c, &[SET_KOAKI_MEIRU as u16]))),
        None,
        None,
        None,
        None,
        None,
        None,
    );
    // atk down
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e3, 0, LOCATION_MZONE);
    // e3:SetCondition(s.atkcon)
    effect::set_condition(d, e3, Some(condition(atkcon)));
    // e3:SetTarget(s.atktg)
    effect::set_target(d, e3, Some(effect_card_fn(atktg)));
    // e3:SetValue(s.atkval)
    effect::set_value_fn(d, e3, effect_value(atkval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // destroy sub
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_EQUIP+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_EQUIP + EFFECT_TYPE_CONTINUOUS);
    // e4:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e4, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e4:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e4, EFFECT_DESTROY_REPLACE);
    // e4:SetTarget(s.desreptg)
    effect::set_target(d, e4, Some(cost_target(desreptg)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.atkcon(e)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsPhase(PHASE_DAMAGE_CAL) and e:GetHandler():GetEquipTarget():GetBattleTarget()
    if !duel::is_phase(d, PHASE_DAMAGE_CAL) {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    let et = card::get_equip_target(d, h).nil();
    card::get_battle_target(d, et).is_some()
}
// function s.atktg(e,c)
fn atktg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c==e:GetHandler():GetEquipTarget():GetBattleTarget()
    let h = effect::get_handler(d, e).nil();
    let et = card::get_equip_target(d, h).nil();
    card::get_battle_target(d, et) == Some(c)
}
// function s.atkval(e,c)
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    // return e:GetHandler():GetEquipTarget():GetLevel()*-100
    let h = effect::get_handler(d, e).nil();
    let et = card::get_equip_target(d, h).nil();
    card::get_level(d, et) as i64 * -100
}
// function s.desreptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:GetEquipTarget():IsReason(REASON_REPLACE) and Duel.IsPhase(PHASE_END) end
    if a.chk == Some(0) {
        let et = card::get_equip_target(d, c).nil();
        return !card::is_reason(d, et, REASON_REPLACE) && duel::is_phase(d, PHASE_END);
    }
    // if Duel.SelectEffectYesNo(tp,c,96) then
    if duel::select_effect_yes_no(d, a.tp, c, Some(96)) {
        // Duel.SendtoGrave(c,REASON_EFFECT)
        duel::send_to_grave(d, c, REASON_EFFECT, None, None);
        // return true
        return true;
    }
    // else return false end
    false
}
