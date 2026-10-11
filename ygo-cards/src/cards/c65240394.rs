//! Big Shield Gardna (Pre-Errata)
//! The negate effect is a continuous effect rather than quick
use super::prelude::*;

pub const ID: u32 = 65240394;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // to attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e1, EVENT_DAMAGE_STEP_END);
    // e1:SetOperation(s.posop)
    effect::set_operation(d, e1, Some(operation(posop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // if not c:IsStatus(STATUS_COPYING_EFFECT) then s[c]=true end
    if !card::is_status(d, c, STATUS_COPYING_EFFECT) {
        set_s_var(d, ID, c.0 as u64, Value::Bool(true));
    }
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, move |d| {
        // local ge=Effect.CreateEffect(c)
        let ge = effect::create_effect(d, c);
        // ge:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge:SetCode(EVENT_CHAIN_ACTIVATING)
        effect::set_code(d, ge, EVENT_CHAIN_ACTIVATING);
        // ge:SetOperation(s.negop)
        effect::set_operation(d, ge, Some(operation(negop)));
        // Duel.RegisterEffect(ge,0)
        duel::register_effect(d, ge, 0);
    });
}
/// `s.posop`
fn posop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c==Duel.GetAttackTarget() and c:IsDefensePos() and c:IsRelateToBattle() then
    if Some(c) == duel::get_attack_target(d) && card::is_defense_pos(d, c) && card::is_relate_to_battle(d, c) {
        // Duel.ChangePosition(c,POS_FACEUP_ATTACK)
        duel::change_position(d, CardSet::from_card(c), POS_FACEUP_ATTACK as u8, None, None, None, false);
    }
}
/// `s.negop`
fn negop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetChainInfo(ev,CHAININFO_TYPE)==TYPE_SPELL and re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then
    let chain_type = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TYPE])[0].int();
    if chain_type == TYPE_SPELL as i64 && effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE) && effect::is_has_property(d, a.re.nil(), EFFECT_FLAG_CARD_TARGET, 0) {
        // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
        let g = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
        // if #g~=1 then return end
        if g.len() != 1 {
            return;
        }
        // local tc=g:GetFirst()
        let Some(tc) = group::get_first(d, &g) else { return };
        // if s[tc] and tc:IsLocation(LOCATION_MZONE) and tc:IsFacedown() then
        if matches!(s_var(d, ID, tc.0 as u64), Value::Bool(true)) && card::is_location(d, tc, LOCATION_MZONE) && card::is_facedown(d, tc) {
            // Duel.ChangePosition(tc,POS_FACEUP_DEFENSE)
            duel::change_position(d, CardSet::from_card(tc), POS_FACEUP_DEFENSE as u8, None, None, None, false);
            // Duel.NegateActivation(ev)
            duel::negate_activation(d, a.ev as u8);
        }
    }
}
