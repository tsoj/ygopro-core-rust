//! Battle Waltz
use super::prelude::*;

pub const ID: u32 = 15629801;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[15629802], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,c:GetAttack(),c:GetDefense(),c:GetLevel(),c:GetRace(),c:GetAttribute())`
fn can_summon_copy(d: &mut Duel, tp: u8, c: CardId) -> bool {
    let attack = card::get_attack(d, c);
    let defense = card::get_defense(d, c);
    let level = card::get_level(d, c);
    let race = card::get_race(d, c);
    let attribute = card::get_attribute(d, c);
    duel::is_player_can_special_summon_monster(
        d,
        tp,
        ID + 1,
        duel::MonsterData {
            setcodes: Some(vec![0]),
            type_: Some(TYPES_TOKEN),
            attack: Some(attack),
            defense: Some(defense),
            level: Some(level),
            race: Some(race),
            attribute: Some(attribute),
        },
        None,
        None,
        0,
    )
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_SYNCHRO) and Duel.IsPlayerCanSpecialSummonMonster(...)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_SYNCHRO) && can_summon_copy(d, tp, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
    duel::select_target(d, tp, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsFaceup() and tc:IsRelateToEffect(e)
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,tc:GetAttack(),...) then
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && can_summon_copy(d, tp, tc) {
            // local token=Duel.CreateToken(tp,id+1)
            let token = duel::create_token(d, tp, ID + 1).nil();
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_SET_BASE_ATTACK)
            effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
            // e1:SetValue(tc:GetAttack())
            let atk = card::get_attack(d, tc);
            effect::set_value(d, e1, atk as i64);
            // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
            effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
            // token:RegisterEffect(e1)
            card::register_effect(d, token, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_SET_BASE_DEFENSE)
            effect::set_code(d, e2, EFFECT_SET_BASE_DEFENSE);
            // e2:SetValue(tc:GetDefense())
            let def = card::get_defense(d, tc);
            effect::set_value(d, e2, def as i64);
            // token:RegisterEffect(e2)
            card::register_effect(d, token, e2, false);
            // local e3=e1:Clone()
            let e3 = effect::clone(d, e1);
            // e3:SetCode(EFFECT_CHANGE_LEVEL)
            effect::set_code(d, e3, EFFECT_CHANGE_LEVEL);
            // e3:SetValue(tc:GetLevel())
            let lv = card::get_level(d, tc);
            effect::set_value(d, e3, lv as i64);
            // token:RegisterEffect(e3)
            card::register_effect(d, token, e3, false);
            // local e4=e1:Clone()
            let e4 = effect::clone(d, e1);
            // e4:SetCode(EFFECT_CHANGE_RACE)
            effect::set_code(d, e4, EFFECT_CHANGE_RACE);
            // e4:SetValue(tc:GetRace())
            let race = card::get_race(d, tc);
            effect::set_value(d, e4, race as i64);
            // token:RegisterEffect(e4)
            card::register_effect(d, token, e4, false);
            // local e5=e1:Clone()
            let e5 = effect::clone(d, e1);
            // e5:SetCode(EFFECT_CHANGE_ATTRIBUTE)
            effect::set_code(d, e5, EFFECT_CHANGE_ATTRIBUTE);
            // e5:SetValue(tc:GetAttribute())
            let attr = card::get_attribute(d, tc);
            effect::set_value(d, e5, attr as i64);
            // token:RegisterEffect(e5)
            card::register_effect(d, token, e5, false);
            // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            // local e6=Effect.CreateEffect(e:GetHandler())
            let e6 = effect::create_effect(d, h);
            // e6:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e6, EFFECT_TYPE_SINGLE);
            // e6:SetCode(EFFECT_NO_BATTLE_DAMAGE)
            effect::set_code(d, e6, EFFECT_NO_BATTLE_DAMAGE);
            // e6:SetValue(1)
            effect::set_value(d, e6, 1);
            // e6:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e6, RESET_EVENT | RESETS_STANDARD, 1);
            // token:RegisterEffect(e6)
            card::register_effect(d, token, e6, false);
            // local e7=e6:Clone()
            let e7 = effect::clone(d, e6);
            // e7:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
            effect::set_code(d, e7, EFFECT_AVOID_BATTLE_DAMAGE);
            // token:RegisterEffect(e7)
            card::register_effect(d, token, e7, false);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
