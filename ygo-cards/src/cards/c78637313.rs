//! Call of Darkness
use super::prelude::*;

pub const ID: u32 = 78637313;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_MONSTER_REBORN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Send monsters Special Summoned by "Monster Reborn" to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetCode(EVENT_ADJUST)
    effect::set_code(d, e2, EVENT_ADJUST);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetOperation(s.adjustop)
    effect::set_operation(d, e2, Some(operation(adjustop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Cannot set "Monster Reborn"
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCode(EFFECT_CANNOT_SSET)
    effect::set_code(d, e3, EFFECT_CANNOT_SSET);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetTargetRange(0,1)
    effect::set_target_range(d, e3, 0, 1);
    // e3:SetTarget(function(_,c)return c:IsCode(CARD_MONSTER_REBORN)end)
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_code(d, c, &[CARD_MONSTER_REBORN]))));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Cannot activate "Monster Reborn"
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e4, EFFECT_CANNOT_ACTIVATE);
    // e4:SetTargetRange(0,1)
    effect::set_target_range(d, e4, 0, 1);
    // e4:SetValue(function(_,re)return re:GetHandler():IsCode(CARD_MONSTER_REBORN)end)
    effect::set_value_fn(
        d,
        e4,
        lua_fn(|d, a: Args| {
            let re = a.e(1);
            let rh = effect::get_handler(d, re).nil();
            card::is_code(d, rh, &[CARD_MONSTER_REBORN])
        }),
    );
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.adjustop(e,tp,eg,ep,ev,re,r,rp)`
fn adjustop(d: &mut Duel, _a: &EffArgs) {
    // local phase=Duel.GetCurrentPhase()
    let phase = duel::get_current_phase(d);
    // if (phase==PHASE_DAMAGE and not Duel.IsDamageCalculated()) or phase==PHASE_DAMAGE_CAL then return end
    if (phase == PHASE_DAMAGE && !duel::is_damage_calculated(d)) || phase == PHASE_DAMAGE_CAL {
        return;
    }
    // local g=Duel.GetMatchingGroup(Card.IsSummonType,0,LOCATION_MZONE,LOCATION_MZONE,nil,SUMMON_TYPE_SPECIAL+SUMMON_WITH_MONSTER_REBORN)
    let g = duel::get_matching_group(
        d,
        Some(&|d, c| card::is_summon_type(d, c, &[SUMMON_TYPE_SPECIAL + SUMMON_WITH_MONSTER_REBORN])),
        0,
        LOCATION_MZONE,
        LOCATION_MZONE,
        NO_EX,
    );
    // if Duel.SendtoGrave(g,REASON_EFFECT)>0 then
    if duel::send_to_grave(d, g, REASON_EFFECT, None, None) > 0 {
        // Duel.Readjust()
        duel::readjust(d);
    }
}
