//! Mimicking Man-Eater Bug
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 72427512;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Flip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_ATKCHANGE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot be destroyed by battle with monsters of the same type
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Cannot be destroyed by effects of monsters of the same type
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    // e3:SetValue(function(e,re) return re and re:IsMonsterEffect() and re:IsCardRace(e:GetHandler():GetRace()) end)
    effect::set_value_fn(d, e3, lua_fn(e3_value));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e,re) return re and re:IsMonsterEffect() and re:IsCardRace(e:GetHandler():GetRace()) end`
fn e3_value(d: &mut Duel, a: Args) -> bool {
    let re = a.v(1).as_effect();
    let Some(re) = re else { return false };
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    let h = effect::get_handler(d, a.e(0)).nil();
    let race = card::get_race(d, h);
    effect::is_card_race(d, re, race)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not tc or not tc:IsRelateToEffect(e) then return end
    let Some(tc) = tc else { return };
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.Destroy(tc,REASON_EFFECT)>0 and not c:IsStatus(STATUS_BATTLE_DESTROYED)
    //     and c:IsRelateToEffect(e) and c:IsFaceup() then
    if duel::destroy(d, tc, REASON_EFFECT, None, None) > 0
        && !card::is_status(d, c, STATUS_BATTLE_DESTROYED)
        && card::is_relate_to_effect(d, c, a.e)
        && card::is_faceup(d, c)
    {
        // Gain target's original attack
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(tc:GetBaseAttack())
        let base = card::get_base_attack(d, tc) as i64;
        effect::set_value(d, e1, base);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local tc_race=tc:GetOriginalRace()
        let tc_race = card::get_original_race(d, tc);
        // if not c:IsRace(tc_race) and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if !card::is_race(d, c, tc_race) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Change race into target's original race
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_CHANGE_RACE)
            effect::set_code(d, e2, EFFECT_CHANGE_RACE);
            // e2:SetValue(tc_race)
            effect::set_value(d, e2, tc_race as i64);
            // c:RegisterEffect(e2)
            card::register_effect(d, c, e2, false);
        }
    }
}
