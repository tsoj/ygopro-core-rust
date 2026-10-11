//! Performage Trapeze High Magician
use super::prelude::*;

pub const ID: u32 = 6659193;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[17016362], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 5 Spellcaster monsters
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_SPELLCASTER),5,2)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_SPELLCASTER, sc, st, Some(tp)))),
        Some(5),
        2,
        None,
        None,
        None,
        None,
        false,
        None,
    );
    // Cannot be destroyed by battle or card effects while it has material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(function(e) return e:GetHandler():GetOverlayCount()>0 end)
    effect::set_condition(d, e1, Some(condition(has_overlay_cond)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Your opponent takes any effect damage you would have taken instead
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EFFECT_REFLECT_DAMAGE)
    effect::set_code(d, e3, EFFECT_REFLECT_DAMAGE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(1,0)
    effect::set_target_range(d, e3, 1, 0);
    // e3:SetCondition(s.reflectcond)
    effect::set_condition(d, e3, Some(condition(reflectcond)));
    // e3:SetValue(s.reflectvalue)
    effect::set_value_fn(d, e3, lua_fn(reflectvalue));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Can make up to 3 attacks this turn
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCondition(s.atkcon)
    effect::set_condition(d, e4, Some(condition(atkcon)));
    // e4:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e4, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e4:SetOperation(s.atkop)
    effect::set_operation(d, e4, Some(operation(atkop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `function(e) return e:GetHandler():GetOverlayCount()>0 end` (the condition of e1)
fn has_overlay_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_overlay_count(d, h) > 0
}
/// `s.reflectcond(e)`
fn reflectcond(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    // return c:GetOverlayCount()>c:GetFlagEffect(id)
    let c = effect::get_handler(d, a.e).nil();
    card::get_overlay_count(d, c) > card::get_flag_effect(d, c, ID)
}
/// `s.reflectvalue(e,re,val,r,rp,rc)`
fn reflectvalue(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let r = a.int(3) as u32;
    let rp = a.p(4);
    // if (r&REASON_EFFECT)>0 then
    if (r & REASON_EFFECT) > 0 {
        // Duel.Hint(HINT_CARD,rp,id)
        duel::hint(d, HINT_CARD as u8, rp, ID as u64);
        // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        let h = effect::get_handler(d, e).nil();
        card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        // return true
        return true;
    }
    // return false
    false
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return Duel.IsAbleToEnterBP() and c:CanAttack() and c:GetEffectCount(EFFECT_EXTRA_ATTACK)<2
    //     and c:GetOverlayGroup():IsExists(Card.IsCode,1,nil,17016362)
    if !duel::is_able_to_enter_bp(d) || !card::can_attack(d, c) || card::get_effect_count(d, c, EFFECT_EXTRA_ATTACK) >= 2 {
        return false;
    }
    let mats = card::get_overlay_group(d, c);
    group::is_exists(d, &mats, &|d, c| card::is_code(d, c, &[17016362]), 1, NO_EX)
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Can make up to 3 attacks during each Battle Phase this turn
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_EXTRA_ATTACK)
        effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
        // e1:SetValue(2)
        effect::set_value(d, e1, 2);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
