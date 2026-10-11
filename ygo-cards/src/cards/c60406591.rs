//! Sinister Seeds
use super::prelude::*;

pub const ID: u32 = 60406591;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[60406592], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYED);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, |d| {
        // s[0]=nil
        // s[1]=nil
        // s[2]=nil
        reset_values(d);
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_BATTLE_DAMAGE)
        effect::set_code(d, ge1, EVENT_BATTLE_DAMAGE);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
        // aux.AddValuesReset(function() s[0]=nil s[1]=nil s[2]=nil end)
        aux::add_values_reset(d, reset_values);
    });
}
/// `aux.AddValuesReset(function() s[0]=nil s[1]=nil s[2]=nil end)`
fn reset_values(d: &mut Duel) {
    set_s_var(d, ID, 0, Value::Nil);
    set_s_var(d, ID, 1, Value::Nil);
    set_s_var(d, ID, 2, Value::Nil);
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // s[0]=ep
    set_s_var(d, ID, 0, Value::Int(a.ep as i64));
    // s[1]=math.floor(ev/500)
    set_s_var(d, ID, 1, Value::Int(a.ev as i64 / 500));
    // s[2]=eg:GetFirst():GetBattleTarget()
    let eg = a.eg.clone().nil();
    let first = group::get_first(d, &eg).nil();
    let bt = card::get_battle_target(d, first);
    set_s_var(d, ID, 2, match bt {
        Some(x) => Value::Card(x),
        None => Value::Nil,
    });
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return s[0]==tp and #eg==1 and eg:GetFirst()==s[2] and eg:GetFirst():GetBattlePosition()==POS_FACEUP_ATTACK
    if !matches!(s_var(d, ID, 0), Value::Int(v) if v == a.tp as i64) {
        return false;
    }
    let eg = a.eg.clone().nil();
    if eg.len() != 1 {
        return false;
    }
    let first = group::get_first(d, &eg).nil();
    if !matches!(s_var(d, ID, 2), Value::Card(x) if x == first) {
        return false;
    }
    card::get_battle_position(d, first) == POS_FACEUP_ATTACK as u32
}
/// `s[1]` as the Lua number it holds (a comparison with nil is a Lua error)
fn token_count(d: &Duel) -> i64 {
    match s_var(d, ID, 1) {
        Value::Int(v) => v,
        _ => crate::lua_error("attempt to compare nil with number"),
    }
}
fn token_data() -> duel::MonsterData {
    // TYPES_TOKEN, 100, 100, level 1, RACE_PLANT, ATTRIBUTE_DARK
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(100),
        defense: Some(100),
        level: Some(1),
        race: Some(RACE_PLANT),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let ct = token_count(d);
    let tp = a.tp;
    // if chk==0 then return ct>0 and (ct==1 or not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT))
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>=ct
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,100,100,1,RACE_PLANT,ATTRIBUTE_DARK) end
    if a.chk == Some(0) {
        return ct > 0
            && (ct == 1 || duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none())
            && (duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 as i64) >= ct
            && duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), None, None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,ct,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, ct as u8, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,ct,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, ct as u8, tp, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=s[1]
    let ct = token_count(d);
    // if (ct>1 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT))
    //     or Duel.GetLocationCount(tp,LOCATION_MZONE)<ct
    //     or not Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,100,100,1,RACE_PLANT,ATTRIBUTE_DARK) then return end
    if (ct > 1 && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some())
        || (duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 as i64) < ct
        || !duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), None, None, 0)
    {
        return;
    }
    // for i=1,ct do
    for _ in 0..ct {
        // local token=Duel.CreateToken(tp,id+1)
        let token = duel::create_token(d, tp, ID + 1).nil();
        // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
