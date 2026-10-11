//! Live☆Twin Lil-la Treat
use super::prelude::*;

pub const ID: u32 = 81078880;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_KI_SIKIL as u32, SET_EVIL_TWIN as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself from hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Banish itself to reduce ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e2, EVENT_BATTLE_DAMAGE);
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,c)` of the Special Summon procedure
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // return Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_KI_SIKIL),e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    let p = card::get_controler(d, c);
    let hp = effect::get_handler_player(d, a.e(0));
    duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x) && card::is_set_card(d, x, &[SET_KI_SIKIL as u16])), hp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=Duel.GetAttacker()
    let tc = duel::get_attacker(d);
    // local at=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    // return (Duel.GetBattleDamage(tp)>0 or Duel.GetBattleDamage(1-tp))
    //     and (tc:IsSetCard(SET_EVIL_TWIN) or (at and at:IsSetCard(SET_EVIL_TWIN)))
    // The second operand of the Lua `or` is a number, which is always true in a condition.
    let _ = duel::get_battle_damage(d, a.tp) > 0;
    let _ = duel::get_battle_damage(d, 1 - a.tp);
    let tc = tc.nil();
    if card::is_set_card(d, tc, &[SET_EVIL_TWIN as u16]) {
        return true;
    }
    match at {
        Some(at) => card::is_set_card(d, at, &[SET_EVIL_TWIN as u16]),
        None => false,
    }
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // (the Lua `chkc` is not a parameter of this function: it is the nil global, so no chkc branch)
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| card::is_faceup(d, x)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,Card.IsFaceup,tp,0,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, a.tp, Some(&|d, x| card::is_faceup(d, x)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // if Duel.GetBattleDamage(tp)>0 then
            let dmg = duel::get_battle_damage(d, a.tp);
            if dmg > 0 {
                // e1:SetValue(-Duel.GetBattleDamage(tp))
                effect::set_value(d, e1, -(dmg as i64));
            } else {
                // e1:SetValue(-Duel.GetBattleDamage(1-tp))
                let other = duel::get_battle_damage(d, 1 - a.tp);
                effect::set_value(d, e1, -(other as i64));
            }
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
