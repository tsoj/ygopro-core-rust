//! Blockman
use super::prelude::*;

pub const ID: u32 = 48115277;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[48115278], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Register when it was placed on the field
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_NO_TURN_RESET)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_NO_TURN_RESET, 0);
    // e0:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e0:SetRange(LOCATION_MZONE)
    effect::set_range(d, e0, LOCATION_MZONE);
    // e0:SetCode(EVENT_ADJUST)
    effect::set_code(d, e0, EVENT_ADJUST);
    // e0:SetCountLimit(1)
    effect::set_count_limit(d, e0, 1, 0, 0, 0);
    // e0:SetOperation(s.op)
    effect::set_operation(d, e0, Some(operation(op)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Check for how long it has been on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EVENT_TURN_END)
    effect::set_code(d, e1, EVENT_TURN_END);
    // e1:SetCondition(s.regcon)
    effect::set_condition(d, e1, Some(condition(regcon)));
    // e1:SetOperation(s.regop)
    effect::set_operation(d, e1, Some(operation(regop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special summon tokens
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCost(s.spcost)
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(1000),
        defense: Some(1500),
        level: Some(4),
        race: Some(RACE_ROCK),
        attribute: Some(ATTRIBUTE_EARTH),
    }
}
fn op(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsTurnPlayer(tp) then
    if duel::is_turn_player(d, a.tp) {
        // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,0)
        let c = effect::get_handler(d, a.e).nil();
        card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
    }
}
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsTurnPlayer(tp)
    !duel::is_turn_player(d, a.tp)
}
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=c:GetFlagEffectLabel(id)
    let ct = card::get_flag_effect_label(d, c, ID).first().copied();
    match ct {
        // if not ct then c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,0)
        None => {
            card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
        }
        // else c:SetFlagEffectLabel(id,ct+1) end
        Some(ct) => {
            card::set_flag_effect_label(d, c, ID, (ct + 1) as u32);
        }
    }
}
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsReleasable() end
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_releasable(d, c, None, None);
    }
    // local ct=e:GetHandler():GetFlagEffectLabel(id)
    // if not ct then ct=0 end
    let ct = card::get_flag_effect_label(d, c, ID).first().copied().unwrap_or(0);
    // e:SetLabel(ct)
    effect::set_label(d, a.e, &[ct]);
    // Duel.Release(e:GetHandler(),REASON_COST)
    duel::release(d, c, REASON_COST, None);
    true
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 as i64;
    let c = effect::get_handler(d, a.e).nil();
    // if e:GetHandler():GetSequence()<5 then ft=ft+1 end
    if card::get_sequence(d, c) < 5 {
        ft += 1;
    }
    if a.chk == Some(0) {
        // local ct=e:GetHandler():GetFlagEffectLabel(id)
        // if not ct then ct=0 end
        let ct = card::get_flag_effect_label(d, c, ID).first().copied().unwrap_or(0);
        // return (ct==0 or not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)) and ft>ct-1
        //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,1000,1500,4,RACE_ROCK,ATTRIBUTE_EARTH)
        return (ct == 0 || duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none())
            && ft > ct - 1
            && duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), None, None, 0);
    }
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,ct+1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, (ct + 1) as u8, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,ct+1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, (ct + 1) as u8, a.tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // if ct>0 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if ct > 0 && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>ct
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,1000,1500,4,RACE_ROCK,ATTRIBUTE_EARTH) then
    if (duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 as i64) > ct
        && duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), None, None, 0)
    {
        // for i=1,ct+1 do
        for _i in 1..=(ct + 1) {
            // local token=Duel.CreateToken(tp,id+1)
            let token = duel::create_token(d, a.tp, ID + 1).nil();
            // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
            duel::special_summon_step(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
            // local e1=Effect.CreateEffect(e:GetHandler())
            let c = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CANNOT_ATTACK)
            effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // token:RegisterEffect(e1,true)
            card::register_effect(d, token, e1, true);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
