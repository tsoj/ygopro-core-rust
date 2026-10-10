//! Rod of Silence - Kay'est
use super::prelude::*;

pub const ID: u32 = 95515060;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // Def
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // e2:SetValue(500)
    effect::set_value(d, e2, 500);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // disable
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e4, EFFECT_DISABLE);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetTargetRange(LOCATION_SZONE,LOCATION_SZONE)
    effect::set_target_range(d, e4, LOCATION_SZONE, LOCATION_SZONE);
    // e4:SetTarget(s.distg)
    effect::set_target(d, e4, Some(effect_card_fn(distg)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // disable effect
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e5:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e5, EVENT_CHAIN_SOLVING);
    // e5:SetRange(LOCATION_SZONE)
    effect::set_range(d, e5, LOCATION_SZONE);
    // e5:SetOperation(s.disop)
    effect::set_operation(d, e5, Some(operation(disop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // self destroy
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD);
    // e6:SetCode(EFFECT_SELF_DESTROY)
    effect::set_code(d, e6, EFFECT_SELF_DESTROY);
    // e6:SetRange(LOCATION_SZONE)
    effect::set_range(d, e6, LOCATION_SZONE);
    // e6:SetTargetRange(LOCATION_SZONE,LOCATION_SZONE)
    effect::set_target_range(d, e6, LOCATION_SZONE, LOCATION_SZONE);
    // e6:SetTarget(s.distg)
    effect::set_target(d, e6, Some(effect_card_fn(distg)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.eqfilter(c,eq,eqc)`: `eq==c and c:GetEquipGroup():IsContains(eqc)`
fn eqfilter(d: &mut Duel, x: CardId, eq: CardId, eqc: CardId) -> bool {
    eq == x && card::get_equip_group(d, x).contains(eqc)
}
/// `s.distg(e,c)`
fn distg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local ec=e:GetHandler()
    let ec = effect::get_handler(d, e).nil();
    // if c==ec or c:GetCardTargetCount()==0 then return false end
    if c == ec || card::get_card_target_count(d, c) == 0 {
        return false;
    }
    // local eq=ec:GetEquipTarget()
    let Some(eq) = card::get_equip_target(d, ec) else {
        // return eq and ...  (nil when there is no equip target)
        return false;
    };
    // return eq and (c:IsHasCardTarget(eq) or c:GetCardTarget():IsExists(s.eqfilter,1,nil,eq,c)) and c:IsSpell()
    let targets = card::get_card_target(d, c);
    (card::is_has_card_target(d, c, eq) || group::is_exists(d, &targets, &|d, x| eqfilter(d, x, eq, c), 1, NO_EX)) && card::is_spell(d, c)
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local ec=e:GetHandler()
    let ec = effect::get_handler(d, a.e).nil();
    // if not ec:GetEquipTarget() or not re:IsSpellEffect() or not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return end
    let Some(eqt) = card::get_equip_target(d, ec) else { return };
    let re = a.re.nil();
    if !effect::is_spell_effect(d, re) || !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return;
    }
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned());
    // if not g or not g:IsContains(ec:GetEquipTarget()) then return end
    let Some(g) = g else { return };
    if !g.contains(eqt) {
        return;
    }
    // if Duel.NegateEffect(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_effect(d, a.ev as u8) {
        let rc = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rc, re) {
            // Duel.Destroy(re:GetHandler(),REASON_EFFECT)
            duel::destroy(d, rc, REASON_EFFECT, None, None);
        }
    }
}
