//! Ni-Ni the Mirror Mikanko
//! 鏡の御巫ニニ
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 54862960;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Take no battle damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e1, EFFECT_AVOID_BATTLE_DAMAGE);
    // e1:SetCondition(aux.NOT(s.eqcon))
    effect::set_condition(d, e1, Some(condition(not_eqcon)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Cannot be destroyed by battle
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetCondition(s.eqcon)
    effect::set_condition(d, e2, Some(condition(eqcon_cond)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --Reflect battle damage
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_REFLECT_BATTLE_DAMAGE)
    effect::set_code(d, e3, EFFECT_REFLECT_BATTLE_DAMAGE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // --Take control of 1 face-up monster
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e4, CATEGORY_CONTROL);
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e4, EVENT_FREE_CHAIN);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_MAIN_END)
    effect::set_hint_timing(d, e4, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    // e4:SetCondition(function(e,tp) return Duel.IsTurnPlayer(1-tp) and s.eqcon(e) end)
    effect::set_condition(d, e4, Some(lua_fn(cond4)));
    // e4:SetTarget(s.cttg)
    effect::set_target(d, e4, Some(cost_target(cttg)));
    // e4:SetOperation(s.ctop)
    effect::set_operation(d, e4, Some(operation(ctop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.eqcon(e)`
fn eqcon(d: &mut Duel, e: EffectId) -> bool {
    // return e:GetHandler():GetEquipCount()>0
    let h = effect::get_handler(d, e).nil();
    card::get_equip_count(d, h) > 0
}
/// `s.eqcon` as a condition
fn eqcon_cond(d: &mut Duel, a: &EffArgs) -> bool {
    eqcon(d, a.e)
}
/// `aux.NOT(s.eqcon)` as a condition
fn not_eqcon(d: &mut Duel, a: &EffArgs) -> bool {
    !eqcon(d, a.e)
}
/// The condition of e4: `function(e,tp) return Duel.IsTurnPlayer(1-tp) and s.eqcon(e) end`
fn cond4(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    duel::is_turn_player(d, 1 - tp) && eqcon(d, a.e(0))
}
/// `s.cttg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsFaceup() and chkc:IsControlerCanBeChanged() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, 1 - tp)
            && card::is_faceup(d, chkc)
            && card::is_controler_can_be_changed(d, chkc, false, Some(0xff));
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsControlerCanBeChanged),tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(
            d,
            Some(&aux::faceup_filter(|d, x| card::is_controler_can_be_changed(d, x, false, Some(0xff)))),
            tp,
            0,
            LOCATION_MZONE,
            1,
            NO_EX,
        );
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsControlerCanBeChanged),tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(
        d,
        tp,
        Some(&aux::faceup_filter(|d, x| card::is_controler_can_be_changed(d, x, false, Some(0xff)))),
        tp,
        0,
        LOCATION_MZONE,
        1,
        1,
        false,
        NO_EX,
    );
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 1, 0, 0);
    true
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.GetControl(tc,tp,PHASE_END,1)
        duel::get_control(d, tc, a.tp, PHASE_END as u16, 1, Some(0xff), None);
    }
}
