//! Nitro Warrior
use super::prelude::*;

pub const ID: u32 = 18013090;
pub const INFO: ScriptInfo = ScriptInfo {
    material: &[96182448],
    listed_names: &[96182448],
    material_setcode: &[SET_SYNCHRON],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: "Nitro Synchron" + 1 or more non-Tuner monsters
    // Synchro.AddProcedure(c,s.tunerfilter,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, Some(mat_filter(tunerfilter)), 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Register Spell card activations
    // local e0a=Effect.CreateEffect(c)
    let e0a = effect::create_effect(d, c);
    // e0a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e0a, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e0a:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e0a, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e0a:SetCode(EVENT_CHAINING)
    effect::set_code(d, e0a, EVENT_CHAINING);
    // e0a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e0a, LOCATION_MZONE);
    // e0a:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return ep==tp and Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e0a, Some(condition(e0a_cond)));
    // e0a:SetOperation(aux.chainreg)
    effect::set_operation(d, e0a, Some(aux::chainreg()));
    // c:RegisterEffect(e0a)
    card::register_effect(d, c, e0a, false);
    // local e0b=Effect.CreateEffect(c)
    let e0b = effect::create_effect(d, c);
    // e0b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e0b, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e0b:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e0b, EVENT_CHAIN_SOLVED);
    // e0b:SetRange(LOCATION_MZONE)
    effect::set_range(d, e0b, LOCATION_MZONE);
    // e0b:SetOperation(s.regop)
    effect::set_operation(d, e0b, Some(operation(regop)));
    // c:RegisterEffect(e0b)
    card::register_effect(d, c, e0b, false);
    // Once during each of your turns, if you activate a Spell Card, this card gains 1000 ATK during the next attack involving this card, during damage calculation only
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE);
    // e1a:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1a, EFFECT_UPDATE_ATTACK);
    // e1a:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1a, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1a, LOCATION_MZONE);
    // e1a:SetCondition(s.atkcon)
    effect::set_condition(d, e1a, Some(condition(atkcon)));
    // e1a:SetValue(1000)
    effect::set_value(d, e1a, 1000);
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // Reset the flag effect above if this card attacks again
    // local e1b=Effect.CreateEffect(c)
    let e1b = effect::create_effect(d, c);
    // e1b:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1b, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1b:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1b, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1b:SetCode(EVENT_BATTLED)
    effect::set_code(d, e1b, EVENT_BATTLED);
    // e1b:SetCondition(function(e) return e:GetHandler():HasFlagEffect(id) end)
    effect::set_condition(d, e1b, Some(condition(e1b_cond)));
    // e1b:SetOperation(function(e) e:GetHandler():ResetFlagEffect(id) end)
    effect::set_operation(d, e1b, Some(operation(e1b_op)));
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // Change 1 Defense Position monster your opponent controls to Attack Position, then this card can make a second attack against that monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e2, CATEGORY_POSITION);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_BATTLED)
    effect::set_code(d, e2, EVENT_BATTLED);
    // e2:SetCondition(s.chainattackposcon)
    effect::set_condition(d, e2, Some(condition(chainattackposcon)));
    // e2:SetTarget(s.chainattackpostg)
    effect::set_target(d, e2, Some(cost_target(chainattackpostg)));
    // e2:SetOperation(s.chainattackposop)
    effect::set_operation(d, e2, Some(operation(chainattackposop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.tunerfilter(c,lc,stype,tp)`
fn tunerfilter(d: &mut Duel, c: CardId, sc: Option<CardId>, st: u64, tp: u8) -> bool {
    // return c:IsSummonCode(lc,stype,tp,96182448) or c:IsHasEffect(20932152)
    card::is_summon_code(d, c, sc, st, tp, &[96182448]) || !card::is_has_effect(d, c, 20932152, None).is_empty()
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return ep==tp and Duel.IsTurnPlayer(tp) end` (e0a)
fn e0a_cond(d: &mut Duel, a: &EffArgs) -> bool {
    a.ep == a.tp && duel::is_turn_player(d, a.tp)
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // if re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsSpellEffect() and e:GetHandler():GetFlagEffect(1)>0 then
    let re = a.re.nil();
    if effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && effect::is_spell_effect(d, re) {
        let h = effect::get_handler(d, a.e).nil();
        if card::get_flag_effect(d, h, 1) > 0 {
            // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
            let h = effect::get_handler(d, a.e).nil();
            card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        }
    }
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return Duel.GetAttacker()==c and c:HasFlagEffect(id) and Duel.IsPhase(PHASE_DAMAGE_CAL)
    duel::get_attacker(d) == Some(c) && card::has_flag_effect(d, c, ID, None) && duel::is_phase(d, PHASE_DAMAGE_CAL)
}
/// `function(e) return e:GetHandler():HasFlagEffect(id) end` (e1b)
fn e1b_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::has_flag_effect(d, h, ID, None)
}
/// `function(e) e:GetHandler():ResetFlagEffect(id) end` (e1b)
fn e1b_op(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    card::reset_flag_effect(d, h, ID);
}
/// `s.chainattackposcon(e,tp,eg,ep,ev,re,r,rp)`
fn chainattackposcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // return bc and bc:IsStatus(STATUS_BATTLE_DESTROYED) and c:CanChainAttack()
    match bc {
        None => false,
        Some(bc) => card::is_status(d, bc, STATUS_BATTLE_DESTROYED) && card::can_chain_attack(d, c, None, false),
    }
}
/// `s.chainattackposfilter(c)`
fn chainattackposfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsDefensePos() and not c:IsStatus(STATUS_BATTLE_DESTROYED) and c:IsCanChangePosition()
    card::is_faceup(d, c) && card::is_defense_pos(d, c) && !card::is_status(d, c, STATUS_BATTLE_DESTROYED) && card::is_can_change_position(d, c)
}
/// `s.chainattackpostg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn chainattackpostg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        // return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and s.chainattackposfilter(chkc)
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && chainattackposfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.chainattackposfilter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&chainattackposfilter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_POSCHANGE as u64);
    // local g=Duel.SelectTarget(tp,s.chainattackposfilter,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&chainattackposfilter), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g), 1, 0, 0);
    true
}
/// `s.chainattackposop(e,tp,eg,ep,ev,re,r,rp)`
fn chainattackposop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
            // Duel.ChangePosition(tc,POS_FACEUP_ATTACK)
            duel::change_position(d, tc, POS_FACEUP_ATTACK as u8, None, None, None, false);
            // Duel.ChainAttack(tc)
            duel::chain_attack(d, Some(tc));
        }
    }
}
