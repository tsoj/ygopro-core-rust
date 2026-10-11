//! Amaze Attraction Majestic Merry-Go-Round
use super::prelude::*;

pub const ID: u32 = 29867611;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_AMAZEMENT, SET_ATTRACTION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // From cards_specific_functions.lua
    // aux.AddAttractionEquipProc(c)
    aux::add_attraction_equip_proc(d, c);
    // You: The equipped monster gains 500 ATK, also if it would be destroyed by battle or card effect, you can send this card to the GY instead.
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_card_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Your opponent: The equipped monster loses 500 ATK for each of your "Ɐttraction" Traps that are equipped to a monster.
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_EQUIP+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP + EFFECT_TYPE_CONTINUOUS);
    // e3:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e3:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e3, EFFECT_DESTROY_REPLACE);
    // e3:SetTarget(s.desreptg)
    effect::set_target(d, e3, Some(cost_target(desreptg)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.atkfilter(c)
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsTrap() and c:IsSetCard(SET_ATTRACTION) and c:GetEquipTarget()
    card::is_trap(d, c) && card::is_set_card(d, c, &[SET_ATTRACTION as u16]) && card::get_equip_target(d, c).is_some()
}
// function s.atkval(e,c)
fn atkval(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // local et=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, e).nil();
    let et = card::get_equip_target(d, h);
    // local hp=e:GetHandlerPlayer()
    let hp = effect::get_handler_player(d, e);
    // if not et then return 0 end
    let Some(et) = et else {
        return 0;
    };
    // if et:GetControler()==hp then
    if card::get_controler(d, et) == hp {
        // return 500
        500
    } else {
        // return Duel.GetMatchingGroupCount(s.atkfilter,hp,LOCATION_SZONE,0,nil)*-500
        duel::get_matching_group_count(d, Some(&atkfilter), hp, LOCATION_SZONE, 0, NO_EX) as i64 * -500
    }
}
// function s.desreptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=c:GetEquipTarget()
    let tc = card::get_equip_target(d, c);
    // if chk==0 then return tc:IsControler(tp) and not tc:IsReason(REASON_REPLACE) end
    if a.chk == Some(0) {
        let tc = tc.nil();
        return card::is_controler(d, tc, a.tp) && !card::is_reason(d, tc, REASON_REPLACE);
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
