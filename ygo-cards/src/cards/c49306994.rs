//! White Veil
use super::prelude::*;

pub const ID: u32 = 49306994;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // Activation limitation
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    effect::set_target_range(d, e1, 0, 1);
    effect::set_value_fn(d, e1, lua_fn(aclimit));
    effect::set_condition(d, e1, Some(condition(actcon)));
    card::register_effect(d, c, e1, false);
    // Disable
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    effect::set_condition(d, e2, Some(condition(discon)));
    effect::set_operation(d, e2, Some(operation(disop)));
    card::register_effect(d, c, e2, false);
    // Destroy Spell/Trap
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_DESTROY);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYING);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_condition(d, e3, Some(condition(descon)));
    effect::set_target(d, e3, Some(cost_target(destg)));
    effect::set_operation(d, e3, Some(operation(desop)));
    card::register_effect(d, c, e3, false);
    // Register before leaving
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e4, EVENT_LEAVE_FIELD_P);
    effect::set_operation(d, e4, Some(operation(checkop)));
    card::register_effect(d, c, e4, false);
    // Damage
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e5, EVENT_LEAVE_FIELD);
    effect::set_label_object(d, e5, Value::Effect(e4));
    effect::set_operation(d, e5, Some(operation(leave)));
    card::register_effect(d, c, e5, false);
    // aux.DoubleSnareValidity(c,LOCATION_SZONE)
    aux::double_snare_validity(d, Some(c), LOCATION_SZONE, None);
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // return re:IsHasType(EFFECT_TYPE_ACTIVATE) and not re:GetHandler():IsImmuneToEffect(e)
    let re = a.e(1);
    if !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
        return false;
    }
    let h = effect::get_handler(d, re).nil();
    !card::is_immune_to_effect(d, h, a.e(0))
}
/// `s.actcon(e)`
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_equip_target(d, h);
    // return Duel.GetAttacker()==tc or Duel.GetAttackTarget()==tc
    duel::get_attacker(d) == tc || duel::get_attack_target(d) == tc
}
/// `s.discon(e,...)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_equip_target(d, h);
    // return Duel.GetAttacker()==tc or Duel.GetAttackTarget()==tc
    duel::get_attacker(d) == tc || duel::get_attack_target(d) == tc
}
/// `s.disop(e,...)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(Card.IsNegatableSpellTrap,tp,0,LOCATION_ONFIELD,c)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_negatable_spell_trap(d, x)), a.tp, 0, LOCATION_ONFIELD, c.into());
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
        // if tc:IsType(TYPE_TRAPMONSTER) then
        if card::is_type(d, tc, TYPE_TRAPMONSTER) {
            // local e3=Effect.CreateEffect(c)
            let e3 = effect::create_effect(d, c);
            // e3:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            // e3:SetCode(EFFECT_DISABLE_TRAPMONSTER)
            effect::set_code(d, e3, EFFECT_DISABLE_TRAPMONSTER);
            // e3:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE)
            effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
            // tc:RegisterEffect(e3)
            card::register_effect(d, tc, e3, false);
        }
    }
}
/// `s.descon(e,...)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=e:GetHandler():GetEquipTarget():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let et = card::get_equip_target(d, h).nil();
    let bc = card::get_battle_target(d, et);
    // return bc and bc:IsPreviousControler(1-tp)
    match bc {
        Some(bc) => card::is_previous_controler(d, bc, 1 - a.tp),
        None => false,
    }
}
/// `s.destg(e,...,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_spell_trap(d, c)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // local sg=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,nil)
    let sg = duel::get_matching_group(d, Some(&|d, c| card::is_spell_trap(d, c)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,sg,#sg,0,0)
    let n = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(sg), n, 0, 0);
    true
}
/// `s.desop(e,...)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local sg=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,nil)
    let sg = duel::get_matching_group(d, Some(&|d, c| card::is_spell_trap(d, c)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // Duel.Destroy(sg,REASON_EFFECT)
    duel::destroy(d, sg, REASON_EFFECT, None, None);
}
/// `s.checkop(e,...)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsDisabled() then e:SetLabel(1) else e:SetLabel(0) end
    let h = effect::get_handler(d, a.e).nil();
    if card::is_disabled(d, h) {
        effect::set_label(d, a.e, &[1]);
    } else {
        effect::set_label(d, a.e, &[0]);
    }
}
/// `s.leave(e,...)`
fn leave(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if e:GetLabelObject():GetLabel()==0 and c:IsPreviousControler(tp)
    //   and c:IsPreviousLocation(LOCATION_SZONE) and c:IsPreviousPosition(POS_FACEUP) then
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    if effect::get_label(d, lo) == 0
        && card::is_previous_controler(d, c, a.tp)
        && card::is_previous_location(d, c, LOCATION_SZONE)
        && card::is_previous_position(d, c, POS_FACEUP)
    {
        // Duel.Damage(tp,3000,REASON_EFFECT)
        duel::damage(d, a.tp, 3000, REASON_EFFECT, false, None);
    }
}
