//! Photon Sanctuary
use super::prelude::*;

pub const ID: u32 = 17418744;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[17418745], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SUMMON,s.counterfilter)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SUMMON, lua_fn(counterfilter));
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,s.counterfilter)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(counterfilter));
    // Duel.AddCustomActivityCounter(id,ACTIVITY_FLIPSUMMON,s.counterfilter)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_FLIPSUMMON, lua_fn(counterfilter));
}
/// `s.counterfilter(c)`
fn counterfilter(d: &mut Duel, a: Args) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT)
    card::is_attribute(d, a.c(0), ATTRIBUTE_LIGHT)
}
/// `s.sumlimit(e,c,sump,sumtype,sumpos,targetp,se)`
fn sumlimit(d: &mut Duel, a: Args) -> bool {
    // return c:IsAttribute(ATTRIBUTE_ALL&~ATTRIBUTE_LIGHT)
    card::is_attribute(d, a.c(1), ATTRIBUTE_ALL & !ATTRIBUTE_LIGHT)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetCustomActivityCount(id,tp,ACTIVITY_SUMMON)==0
    //     and Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)==0
    //     and Duel.GetCustomActivityCount(id,tp,ACTIVITY_FLIPSUMMON)==0 end
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, tp, ACTIVITY_SUMMON) == 0
            && duel::get_custom_activity_count(d, ID, tp, ACTIVITY_SPSUMMON) == 0
            && duel::get_custom_activity_count(d, ID, tp, ACTIVITY_FLIPSUMMON) == 0;
    }
    let h = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.sumlimit)
    effect::set_target(d, e1, Some(lua_fn(sumlimit)));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_CANNOT_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SUMMON);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EFFECT_CANNOT_FLIP_SUMMON)
    effect::set_code(d, e3, EFFECT_CANNOT_FLIP_SUMMON);
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, tp);
    // local e4=Effect.CreateEffect(e:GetHandler())
    let e4 = effect::create_effect(d, h);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e4, RESET_PHASE | PHASE_END, 1);
    // e4:SetTargetRange(1,0)
    effect::set_target_range(d, e4, 1, 0);
    // Duel.RegisterEffect(e4,tp)
    duel::register_effect(d, e4, tp);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>1
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_PHOTON,TYPES_TOKEN,2000,0,4,RACE_THUNDER,ATTRIBUTE_LIGHT) end
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 1
            && can_summon(d, tp);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 2, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, 0);
    true
}
/// `Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_PHOTON,TYPES_TOKEN,2000,0,4,RACE_THUNDER,ATTRIBUTE_LIGHT)`
fn can_summon(d: &mut Duel, tp: u8) -> bool {
    duel::is_player_can_special_summon_monster(
        d,
        tp,
        ID + 1,
        duel::MonsterData {
            setcodes: Some(vec![SET_PHOTON as u16]),
            type_: Some(TYPES_TOKEN),
            attack: Some(2000),
            defense: Some(0),
            level: Some(4),
            race: Some(RACE_THUNDER),
            attribute: Some(ATTRIBUTE_LIGHT),
        },
        None,
        None,
        0,
    )
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and Duel.GetLocationCount(tp,LOCATION_MZONE)>1
    //     and Duel.IsPlayerCanSpecialSummonMonster(...) then
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 1
        && can_summon(d, tp)
    {
        // for i=1,2 do
        for _ in 0..2 {
            // local token=Duel.CreateToken(tp,id+1)
            let token = duel::create_token(d, tp, ID + 1).nil();
            // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
            duel::special_summon_step(d, token, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
            let h = effect::get_handler(d, a.e).nil();
            // local e1=Effect.CreateEffect(e:GetHandler())
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CANNOT_ATTACK)
            effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // token:RegisterEffect(e1,true)
            card::register_effect(d, token, e1, true);
            // local e2=Effect.CreateEffect(e:GetHandler())
            let e2 = effect::create_effect(d, h);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_CANNOT_BE_SYNCHRO_MATERIAL)
            effect::set_code(d, e2, EFFECT_CANNOT_BE_SYNCHRO_MATERIAL);
            // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
            effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
            // e2:SetValue(1)
            effect::set_value(d, e2, 1);
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            // token:RegisterEffect(e2,true)
            card::register_effect(d, token, e2, true);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
