//! Vahram, the Magistus Divinity Dragon
use super::prelude::*;

pub const ID: u32 = 61272280;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MAGISTUS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Prevent destruction by opponent's Spell/Trap
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetValue(s.indesvalue)
    effect::set_value_fn(d, e1, lua_fn(indesvalue));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy all opponent's face-up cards
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.descon1)
    effect::set_condition(d, e2, Some(condition(descon1)));
    // e2:SetTarget(s.destg1)
    effect::set_target(d, e2, Some(cost_target(destg1)));
    // e2:SetOperation(s.desop1)
    effect::set_operation(d, e2, Some(operation(desop1)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy battling monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e3, EVENT_BATTLE_START);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.descon2)
    effect::set_condition(d, e3, Some(condition(descon2)));
    // e3:SetTarget(s.destg2)
    effect::set_target(d, e3, Some(cost_target(destg2)));
    // e3:SetOperation(s.desop2)
    effect::set_operation(d, e3, Some(operation(desop2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.indesvalue(e,re,rp)`
fn indesvalue(d: &mut Duel, a: Args) -> bool {
    // return re:IsSpellTrapEffect() and rp==1-e:GetHandlerPlayer()
    let re = a.e(1);
    let hp = effect::get_handler_player(d, a.e(0));
    effect::is_spell_trap_effect(d, re) && a.int(2) == 1 - hp as i64
}
/// `s.descon1(e,tp,eg,ep,ev,re,r,rp)`
fn descon1(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsSynchroSummoned()
    card::is_previous_location(d, c, LOCATION_MZONE) && card::is_synchro_summoned(d, c)
}
/// `s.destg1(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg1(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop1(e,tp,eg,ep,ev,re,r,rp)`
fn desop1(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
/// `s.descon2(e,tp,eg,ep,ev,re,r,rp)`
fn descon2(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=e:GetHandler():GetEquipTarget()
    let c = effect::get_handler(d, a.e).nil();
    // return ec and ec:GetBattleTarget()
    match card::get_equip_target(d, c) {
        Some(ec) => card::get_battle_target(d, ec).is_some(),
        None => false,
    }
}
/// `s.destg2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local ec=e:GetHandler():GetEquipTarget()
    let c = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, c).nil();
    // local bc=ec:GetBattleTarget()
    let bc = card::get_battle_target(d, ec);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,bc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, bc.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.desop2(e,tp,eg,ep,ev,re,r,rp)`
fn desop2(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // if not e:GetHandler():IsRelateToEffect(e) then return false end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local ec=e:GetHandler():GetEquipTarget()
    let ec = card::get_equip_target(d, c).nil();
    // local bc=ec:GetBattleTarget()
    let bc = card::get_battle_target(d, ec);
    // if ec and bc and bc:IsRelateToBattle() then
    if let Some(bc) = bc {
        if card::is_relate_to_battle(d, bc) {
            // Duel.Destroy(bc,REASON_EFFECT)
            duel::destroy(d, bc, REASON_EFFECT, None, None);
        }
    }
}
