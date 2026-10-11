//! Follow Wing
use super::prelude::*;

pub const ID: u32 = 42776855;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddPersistentProcedure(c,0,s.filter,nil,nil,nil,TIMINGS_CHECK_MONSTER_E)
    aux::add_persistent_procedure(
        d,
        c,
        Some(0),
        Some(aux::equip_filter_fn(|d, c, _e, _tp| filter(d, c))),
        None,
        None,
        None,
        Some(TIMINGS_CHECK_MONSTER_E),
        None,
        None,
        None,
        None,
        false,
    );
    // indes
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_BATTLE);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    // e3:SetTarget(aux.PersistentTargetFilter)
    effect::set_target(d, e3, Some(aux::persistent_target_filter()));
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_EFFECT);
    // e4:SetValue(s.efilter)
    effect::set_value_fn(d, e4, lua_fn(|d, a: Args| efilter(d, a.e(0), a.e(1))));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // destroy
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e5, aux::stringid(ID, 0));
    // e5:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e5, CATEGORY_DESTROY);
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e5:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e5, EVENT_BATTLE_START);
    // e5:SetRange(LOCATION_SZONE)
    effect::set_range(d, e5, LOCATION_SZONE);
    // e5:SetCondition(s.atkcon)
    effect::set_condition(d, e5, Some(condition(atkcon)));
    // e5:SetOperation(s.atkop)
    effect::set_operation(d, e5, Some(operation(atkop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // Destroy2
    // local e7=Effect.CreateEffect(c)
    let e7 = effect::create_effect(d, c);
    // e7:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e7, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e7:SetRange(LOCATION_SZONE)
    effect::set_range(d, e7, LOCATION_SZONE);
    // e7:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e7, EVENT_LEAVE_FIELD);
    // e7:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e7, EFFECT_FLAG_DELAY, 0);
    // e7:SetCondition(s.descon2)
    effect::set_condition(d, e7, Some(condition(descon2)));
    // e7:SetOperation(s.desop2)
    effect::set_operation(d, e7, Some(operation(desop2)));
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_SYNCHRO)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_SYNCHRO)
}
/// `s.efilter(e,re)`
fn efilter(d: &mut Duel, e: EffectId, re: EffectId) -> bool {
    // return e:GetOwnerPlayer()~=re:GetOwnerPlayer()
    effect::get_owner_player(d, e) != effect::get_owner_player(d, re)
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=c:GetFirstCardTarget()
    // if not tc then return false end
    let Some(tc) = card::get_first_card_target(d, c) else { return false };
    // local bc=tc:GetBattleTarget()
    let bc = card::get_battle_target(d, tc);
    // return tc:IsLocation(LOCATION_MZONE) and bc and bc:IsFaceup() and bc:IsLocation(LOCATION_MZONE) and bc:IsLevelAbove(5)
    if !card::is_location(d, tc, LOCATION_MZONE) {
        return false;
    }
    let Some(bc) = bc else { return false };
    card::is_faceup(d, bc) && card::is_location(d, bc, LOCATION_MZONE) && card::is_level_above(d, bc, 5)
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // local tc=c:GetFirstCardTarget()
    // if not tc then return false end
    let Some(tc) = card::get_first_card_target(d, c) else { return };
    // local bc=tc:GetBattleTarget()
    let bc = card::get_battle_target(d, tc).nil();
    // local atk=bc:GetBaseAttack()
    let atk = card::get_base_attack(d, bc);
    // if bc:IsRelateToBattle() and Duel.Destroy(bc,REASON_EFFECT)~=0 and bc:IsMonster() then
    if card::is_relate_to_battle(d, bc) && duel::destroy(d, bc, REASON_EFFECT, None, None) != 0 && card::is_monster(d, bc) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(atk)
        effect::set_value(d, e1, atk as i64);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.descon2(e,tp,eg,ep,ev,re,r,rp)`
fn descon2(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetHandler():GetFirstCardTarget()
    let h = effect::get_handler(d, a.e).nil();
    // return tc and eg:IsContains(tc)
    let Some(tc) = card::get_first_card_target(d, h) else { return false };
    let eg = a.eg.clone().nil();
    group::is_contains(&eg, tc)
}
/// `s.desop2(e,tp,eg,ep,ev,re,r,rp)`
fn desop2(d: &mut Duel, a: &EffArgs) {
    // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::destroy(d, h, REASON_EFFECT, None, None);
}
