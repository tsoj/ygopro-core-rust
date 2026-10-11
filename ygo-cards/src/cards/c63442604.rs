//! Physical Double
use super::prelude::*;

pub const ID: u32 = 63442604;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[63442605], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER));
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsTurnPlayer(tp)
    !duel::is_turn_player(d, a.tp)
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:HasLevel()
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,c:GetAttack(),c:GetDefense(),c:GetLevel(),c:GetRace(),c:GetAttribute())
    if !card::is_faceup(d, c) || !card::has_level(d, c) {
        return false;
    }
    let md = token_data(d, c);
    duel::is_player_can_special_summon_monster(d, tp, ID + 1, md, None, None, 0)
}
/// The token's stats: a copy of `c`'s printed stats (the Lua passes them as the summon's arguments).
fn token_data(d: &mut Duel, c: CardId) -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(card::get_attack(d, c)),
        defense: Some(card::get_defense(d, c)),
        level: Some(card::get_level(d, c)),
        race: Some(card::get_race(d, c)),
        attribute: Some(card::get_attribute(d, c)),
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.cfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && cfilter(d, chkc, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and
    //     Duel.IsExistingTarget(s.cfilter,tp,0,LOCATION_MZONE,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| cfilter(d, c, tp)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.cfilter,tp,0,LOCATION_MZONE,1,1,nil,tp)
    let _g = duel::select_target(d, tp, Some(&|d, c| cfilter(d, c, tp)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not tc or not tc:IsRelateToEffect(e) or tc:IsFacedown() then return end
    let Some(tc) = tc else { return };
    if !card::is_relate_to_effect(d, tc, e) || card::is_facedown(d, tc) {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0
    //     or not Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,tc:GetAttack(),tc:GetDefense(),
    //         tc:GetLevel(),tc:GetRace(),tc:GetAttribute()) then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    let md = token_data(d, tc);
    if !duel::is_player_can_special_summon_monster(d, tp, ID + 1, md, None, None, 0) {
        return;
    }
    // local token=Duel.CreateToken(tp,id+1)
    let token = duel::create_token(d, tp, ID + 1).nil();
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SET_BASE_ATTACK)
    effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
    // e1:SetValue(tc:GetAttack())
    let atk = card::get_attack(d, tc) as i64;
    effect::set_value(d, e1, atk);
    // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
    // token:RegisterEffect(e1)
    card::register_effect(d, token, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_SET_BASE_DEFENSE)
    effect::set_code(d, e2, EFFECT_SET_BASE_DEFENSE);
    // e2:SetValue(tc:GetDefense())
    let def = card::get_defense(d, tc) as i64;
    effect::set_value(d, e2, def);
    // token:RegisterEffect(e2)
    card::register_effect(d, token, e2, false);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EFFECT_CHANGE_LEVEL)
    effect::set_code(d, e3, EFFECT_CHANGE_LEVEL);
    // e3:SetValue(tc:GetLevel())
    let lv = card::get_level(d, tc) as i64;
    effect::set_value(d, e3, lv);
    // token:RegisterEffect(e3)
    card::register_effect(d, token, e3, false);
    // local e4=e1:Clone()
    let e4 = effect::clone(d, e1);
    // e4:SetCode(EFFECT_CHANGE_RACE)
    effect::set_code(d, e4, EFFECT_CHANGE_RACE);
    // e4:SetValue(tc:GetRace())
    let race = card::get_race(d, tc) as i64;
    effect::set_value(d, e4, race);
    // token:RegisterEffect(e4)
    card::register_effect(d, token, e4, false);
    // local e5=e1:Clone()
    let e5 = effect::clone(d, e1);
    // e5:SetCode(EFFECT_CHANGE_ATTRIBUTE)
    effect::set_code(d, e5, EFFECT_CHANGE_ATTRIBUTE);
    // e5:SetValue(tc:GetAttribute())
    let attr = card::get_attribute(d, tc) as i64;
    effect::set_value(d, e5, attr);
    // token:RegisterEffect(e5)
    card::register_effect(d, token, e5, false);
    // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    // local de=Effect.CreateEffect(e:GetHandler())
    let de = effect::create_effect(d, h);
    // de:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, de, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // de:SetRange(LOCATION_MZONE)
    effect::set_range(d, de, LOCATION_MZONE);
    // de:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, de, EVENT_PHASE + PHASE_END);
    // de:SetCountLimit(1)
    effect::set_count_limit(d, de, 1, 0, 0, 0);
    // de:SetOperation(s.desop)
    effect::set_operation(d, de, Some(operation(desop)));
    // de:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, de, RESET_EVENT | RESETS_STANDARD, 1);
    // token:RegisterEffect(de)
    card::register_effect(d, token, de, false);
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::destroy(d, h, REASON_EFFECT, None, None);
}
