//! Vendread Reorigin
use super::prelude::*;

pub const ID: u32 = 30650147;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[30650148], listed_series: &[SET_VENDREAD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
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
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.filter(c,tp)
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:GetOriginalLevel()>0 and c:IsReleasableByEffect()
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_VENDREAD,TYPES_TOKEN,0,0,c:GetLevel(),RACE_ZOMBIE,ATTRIBUTE_DARK)
    card::is_faceup(d, c)
        && card::get_original_level(d, c) > 0
        && card::is_releasable_by_effect(d, c, None, None)
        && {
            let lv = card::get_level(d, c);
            duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(lv), None, None, 0)
        }
}
/// `TYPES_TOKEN` token with Set Code "Vendread", Zombie/DARK, ATK/DEF 0
fn token_data(level: u32) -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_VENDREAD as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(level),
        race: Some(RACE_ZOMBIE),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.filter,tp,0,LOCATION_MZONE,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, tp)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
    // Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_MZONE,1,1,nil,tp)
    let _ = duel::select_target(d, tp, Some(&|d, c| filter(d, c, tp)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, 0);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<1 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.Release(tc,REASON_EFFECT)>0 then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, e) && duel::release(d, tc, REASON_EFFECT, None) > 0 {
            // local token=Duel.CreateToken(tp,id+1)
            let token = duel::create_token(d, tp, ID + 1).nil();
            // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
            effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
            // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
            effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
            // e1:SetRange(LOCATION_MZONE)
            effect::set_range(d, e1, LOCATION_MZONE);
            // e1:SetTargetRange(1,0)
            effect::set_target_range(d, e1, 1, 0);
            // e1:SetTarget(aux.NOT(aux.TargetBoolFunction(Card.IsSetCard,SET_VENDREAD)))
            effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_set_card(d, c, &[SET_VENDREAD as u16]))));
            // token:RegisterEffect(e1,true)
            card::register_effect(d, token, e1, true);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_CANNOT_SUMMON)
            effect::set_code(d, e2, EFFECT_CANNOT_SUMMON);
            // token:RegisterEffect(e2,true)
            card::register_effect(d, token, e2, true);
            // local e3=Effect.CreateEffect(e:GetHandler())
            let e3 = effect::create_effect(d, h);
            // e3:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            // e3:SetCode(EFFECT_CHANGE_LEVEL)
            effect::set_code(d, e3, EFFECT_CHANGE_LEVEL);
            // e3:SetValue(tc:GetOriginalLevel())
            effect::set_value(d, e3, card::get_original_level(d, tc) as i64);
            // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
            // token:RegisterEffect(e3,true)
            card::register_effect(d, token, e3, true);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
