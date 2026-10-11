//! Corrupted Keys
use super::prelude::*;

pub const ID: u32 = 67949763;

// s.listed_names={67949764}
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[67949764], ..ScriptInfo::DEFAULT };

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
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ)
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_UMBRAL_HORROR,TYPES_TOKEN,-2,0,1,RACE_FIEND,ATTRIBUTE_DARK)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ) && duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), None, None, 0)
}
/// The token data of `Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_UMBRAL_HORROR,TYPES_TOKEN,-2,0,1,RACE_FIEND,ATTRIBUTE_DARK)`.
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_UMBRAL_HORROR as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(-2),
        defense: Some(0),
        level: Some(1),
        race: Some(RACE_FIEND as u64),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc, tp);
    }
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and Duel.GetLocationCount(tp,LOCATION_MZONE)>2
    //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 2
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    duel::select_target(d, tp, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,3,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 3, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,3,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 3, tp, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc_opt = duel::first_target(d);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>2 then
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 2 {
        // local rfid=tc:GetRealFieldID()
        let tc = tc_opt.nil();
        let rfid = card::get_real_field_id(d, tc);
        // local atk=0
        let mut atk: i32 = 0;
        // local cr=false
        let mut cr = false;
        // if tc and tc:IsFaceup() and tc:IsRelateToEffect(e) then
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, e) {
            // atk=tc:GetAttack()
            atk = card::get_attack(d, tc);
            // cr=true
            cr = true;
        }
        // if not Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_UMBRAL_HORROR,TYPES_TOKEN,-2,0,1,RACE_FIEND,ATTRIBUTE_DARK) then return end
        if !duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), None, None, 0) {
            return;
        }
        // if cr then
        if cr {
            // local de=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, e).nil();
            let de = effect::create_effect(d, h);
            // de:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, de, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
            // de:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, de, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // de:SetCode(EVENT_LEAVE_FIELD)
            effect::set_code(d, de, EVENT_LEAVE_FIELD);
            // de:SetOperation(s.desop)
            effect::set_operation(d, de, Some(operation(desop)));
            // de:SetLabel(rfid)
            effect::set_label(d, de, &[rfid as i64]);
            // tc:RegisterEffect(de,true)
            card::register_effect(d, tc, de, true);
        }
        // for i=1,3 do
        for _ in 1..=3 {
            // local token=Duel.CreateToken(tp,id+1)
            let token = duel::create_token(d, tp, ID + 1).nil();
            // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
            effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // token:RegisterEffect(e1,true)
            card::register_effect(d, token, e1, true);
            // local e2=Effect.CreateEffect(e:GetHandler())
            let e2 = effect::create_effect(d, h);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_UNRELEASABLE_NONSUM)
            effect::set_code(d, e2, EFFECT_UNRELEASABLE_NONSUM);
            // e2:SetValue(1)
            effect::set_value(d, e2, 1);
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            // token:RegisterEffect(e2,true)
            card::register_effect(d, token, e2, true);
            // local e3=Effect.CreateEffect(e:GetHandler())
            let e3 = effect::create_effect(d, h);
            // e3:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            // e3:SetCode(EFFECT_SET_ATTACK)
            effect::set_code(d, e3, EFFECT_SET_ATTACK);
            // e3:SetValue(atk)
            effect::set_value(d, e3, atk as i64);
            // e3:SetLabelObject(tc)
            effect::set_label_object(d, e3, Value::Card(tc));
            // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
            // token:RegisterEffect(e3,true)
            card::register_effect(d, token, e3, true);
            // if cr then
            if cr {
                // token:RegisterFlagEffect(id+1,RESET_EVENT|RESETS_STANDARD,0,0,rfid)
                card::register_flag_effect(d, token, ID + 1, RESET_EVENT | RESETS_STANDARD, 0, 0, rfid as i64, 0);
                // tc:CreateRelation(token,RESET_EVENT|RESETS_STANDARD&~RESET_TURN_SET)
                card::create_relation(d, tc, token, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET));
            }
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
/// `s.desfilter(c,rfid)`
fn desfilter(d: &mut Duel, c: CardId, rfid: i64) -> bool {
    // return c:GetFlagEffectLabel(id+1)==rfid
    card::get_flag_effect_label(d, c, ID + 1).first().copied() == Some(rfid)
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_MZONE,0,nil,e:GetLabel())
    let label = effect::get_label(d, a.e);
    let g = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c, label)), tp, LOCATION_MZONE, 0, NO_EX);
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
    // e:Reset()
    effect::reset(d, a.e);
}
