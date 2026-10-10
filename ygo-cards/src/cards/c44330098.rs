//! Gorz the Emissary of Darkness
use super::prelude::*;

pub const ID: u32 = 44330098;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[44330099], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCode(EVENT_DAMAGE)
    effect::set_code(d, e1, EVENT_DAMAGE);
    // e1:SetCondition(s.sumcon)
    effect::set_condition(d, e1, Some(condition(sumcon)));
    // e1:SetTarget(s.sumtg)
    effect::set_target(d, e1, Some(cost_target(sumtg)));
    // e1:SetOperation(s.sumop)
    effect::set_operation(d, e1, Some(operation(sumop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon success
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCondition(s.sumcon2)
    effect::set_condition(d, e2, Some(condition(sumcon2)));
    // e2:SetTarget(s.sumtg2)
    effect::set_target(d, e2, Some(cost_target(sumtg2)));
    // e2:SetOperation(s.sumop2)
    effect::set_operation(d, e2, Some(operation(sumop2)));
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetCondition(s.sumcon3)
    effect::set_condition(d, e3, Some(condition(sumcon3)));
    // e3:SetTarget(s.sumtg3)
    effect::set_target(d, e3, Some(cost_target(sumtg3)));
    // e3:SetOperation(s.sumop3)
    effect::set_operation(d, e3, Some(operation(sumop3)));
    // e3:SetLabelObject(e1)
    effect::set_label_object(d, e3, Value::Effect(e1));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsStatus(STATUS_LEAVE_CONFIRMED)
    !card::is_status(d, c, STATUS_LEAVE_CONFIRMED)
}
/// `s.sumcon(e,tp,eg,ep,ev,re,r,rp)`
fn sumcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==tp and tp~=rp and not Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_ONFIELD,0,1,nil)
    a.ep == a.tp && a.tp != a.rp && !duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `s.sumtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sumtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.sumop(e,tp,eg,ep,ev,re,r,rp)`
fn sumop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local sumtype=1
    let mut sumtype: u32 = 1;
    // if (r&REASON_BATTLE)~=0 then sumtype=2 end
    if (a.r & REASON_BATTLE) != 0 {
        sumtype = 2;
    }
    // if Duel.SpecialSummon(c,sumtype,tp,tp,false,false,POS_FACEUP)~=0 then
    if duel::special_summon(d, c, sumtype, a.tp, a.tp, false, false, POS_FACEUP as u8, None) != 0 {
        // e:SetLabel(ev)
        effect::set_label(d, a.e, &[a.ev as i64]);
    }
}
/// `s.sumcon2(e,tp,eg,ep,ev,re,r,rp)`
fn sumcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetSummonType()==SUMMON_TYPE_SPECIAL+2
    let c = effect::get_handler(d, a.e).nil();
    card::get_summon_type(d, c) == SUMMON_TYPE_SPECIAL + 2
}
/// `s.sumtg2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sumtg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    true
}
/// `s.sumop2(e,tp,eg,ep,ev,re,r,rp)`
fn sumop2(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local val=e:GetLabelObject():GetLabel()
    let lbl = effect::get_label_object(d, a.e).as_effect().nil();
    let val = effect::get_label(d, lbl);
    // if not Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,-2,-2,7,RACE_FAIRY,ATTRIBUTE_LIGHT) then return end
    if !duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), None, None, 0) {
        return;
    }
    // local token=Duel.CreateToken(tp,id+1)
    let token = duel::create_token(d, a.tp, ID + 1).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SET_ATTACK)
    effect::set_code(d, e1, EFFECT_SET_ATTACK);
    // e1:SetValue(val)
    effect::set_value(d, e1, val);
    // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
    // token:RegisterEffect(e1)
    card::register_effect(d, token, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_SET_DEFENSE)
    effect::set_code(d, e2, EFFECT_SET_DEFENSE);
    // e2:SetValue(val)
    effect::set_value(d, e2, val);
    // e2:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
    effect::set_reset(d, e2, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
    // token:RegisterEffect(e2)
    card::register_effect(d, token, e2, false);
    // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
/// The token of `s.sumop2`: a Level 7 LIGHT Fairy token with ATK and DEF set by the Lua call
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(-2),
        defense: Some(-2),
        level: Some(7),
        race: Some(RACE_FAIRY),
        attribute: Some(ATTRIBUTE_LIGHT),
    }
}
/// `s.sumcon3(e,tp,eg,ep,ev,re,r,rp)`
fn sumcon3(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetSummonType()==SUMMON_TYPE_SPECIAL+1
    let c = effect::get_handler(d, a.e).nil();
    card::get_summon_type(d, c) == SUMMON_TYPE_SPECIAL + 1
}
/// `s.sumtg3(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sumtg3(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local d=e:GetLabelObject():GetLabel()
    let e1 = effect::get_label_object(d, a.e).as_effect().nil();
    let dmg = effect::get_label(d, e1);
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(d)
    duel::set_target_param(d, dmg as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,d)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, dmg as i32);
    true
}
/// `s.sumop3(e,tp,eg,ep,ev,re,r,rp)`
fn sumop3(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
