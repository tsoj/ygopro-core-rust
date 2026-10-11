//! Damage Translation
//! ダメージ・トランスレーション
use super::prelude::*;

pub const ID: u32 = 35268887;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[35268888],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, move |d| {
        // s[0]=0
        // s[1]=0
        set_s_var(d, ID, 0, Value::Int(0));
        set_s_var(d, ID, 1, Value::Int(0));
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_DAMAGE)
        effect::set_code(d, ge1, EVENT_DAMAGE);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
        // aux.AddValuesReset(function()
        //     s[0]=0
        //     s[1]=0
        // end)
        aux::add_values_reset(d, reset_values);
    });
    // end)
}
/// The reset function of `aux.AddValuesReset`: `s[0]=0 s[1]=0`
fn reset_values(d: &mut Duel) {
    set_s_var(d, ID, 0, Value::Int(0));
    set_s_var(d, ID, 1, Value::Int(0));
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // if r&REASON_EFFECT~=0 then
    if a.r & REASON_EFFECT != 0 {
        // s[ep]=s[ep]+1
        let k = a.ep as u64;
        let v = s_var(d, ID, k).int();
        set_s_var(d, ID, k, Value::Int(v + 1));
    }
}
/// `s.val(e,re,dam,r,rp,rc)`
fn val(_d: &mut Duel, a: Args) -> Value {
    // if r&REASON_EFFECT~=0 then return dam/2
    // else return dam end
    let dam = a.int(2);
    let r = a.int(3) as u32;
    if r & REASON_EFFECT != 0 {
        Value::Int(lua_int(dam as f64 / 2.0))
    } else {
        Value::Int(dam)
    }
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CHANGE_DAMAGE)
    effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(s.val)
    effect::set_value_fn(d, e1, lua_fn(val));
    // e1:SetReset(RESET_PHASE|PHASE_END,1)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local e2=Effect.CreateEffect(e:GetHandler())
    let e2 = effect::create_effect(d, h);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.tokentg)
    effect::set_target(d, e2, Some(cost_target(tokentg)));
    // e2:SetOperation(s.tokenop)
    effect::set_operation(d, e2, Some(operation(tokenop)));
    // e2:SetReset(RESET_PHASE|PHASE_END,1)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
}
/// `Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,0,0,1,RACE_FIEND,ATTRIBUTE_DARK)`
fn can_summon_token(d: &mut Duel, tp: u8) -> bool {
    let md = duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(1),
        race: Some(RACE_FIEND),
        attribute: Some(ATTRIBUTE_DARK),
    };
    duel::is_player_can_special_summon_monster(d, tp, ID + 1, md, None, None, 0)
}
/// `s.tokentg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tokentg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ct=s[tp]
    let ct = s_var(d, ID, tp as u64).int();
    // if chk==0 then return ct>0 and (ct==1 or not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT))
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>=ct
    //     and Duel.IsPlayerCanSpecialSummonMonster(...) end
    if a.chk == Some(0) {
        return ct > 0
            && (ct == 1 || duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none())
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 as i64 >= ct
            && can_summon_token(d, tp);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,ct,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, ct as u8, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,ct,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, ct as u8, tp, 0);
    true
}
/// `s.tokenop(e,tp,eg,ep,ev,re,r,rp)`
fn tokenop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=s[tp]
    let ct = s_var(d, ID, tp as u64).int();
    // if (ct>1 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT))
    //     or Duel.GetLocationCount(tp,LOCATION_MZONE)<ct
    //     or not Duel.IsPlayerCanSpecialSummonMonster(...) then return end
    if (ct > 1 && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some())
        || (duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 as i64) < ct
        || !can_summon_token(d, tp)
    {
        return;
    }
    // for i=1,ct do
    for _ in 0..ct {
        // local token=Duel.CreateToken(tp,id+1)
        let token = duel::create_token(d, tp, ID + 1).nil();
        // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon_step(d, token, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
