//! Speedroid Block-n-Roll
use super::prelude::*;

pub const ID: u32 = 69550259;

// s.listed_names={69550260}
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[69550260], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e1, EVENT_BE_MATERIAL);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.tkcon)
    effect::set_condition(d, e1, Some(condition(tkcon)));
    // e1:SetTarget(s.tktg)
    effect::set_target(d, e1, Some(cost_target(tktg)));
    // e1:SetOperation(s.tkop)
    effect::set_operation(d, e1, Some(operation(tkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tkcon(e,tp,eg,ep,ev,re,r,rp)`
fn tkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLocation(LOCATION_GRAVE) and r==REASON_SYNCHRO
    let h = effect::get_handler(d, a.e).nil();
    card::is_location(d, h, LOCATION_GRAVE) && a.r == REASON_SYNCHRO
}
/// `sync:GetLevel()` where `sync` is the reason card (nil raises an error)
fn sync_level(d: &mut Duel, sync: Option<CardId>) -> u32 {
    let sync = sync.nil();
    card::get_level(d, sync)
}
/// The token data of `Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_SPEEDROID,TYPES_TOKEN,0,0,lv,RACE_MACHINE,ATTRIBUTE_WIND)`
fn token_data(lv: u32) -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_SPEEDROID as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(lv),
        race: Some(RACE_MACHINE),
        attribute: Some(ATTRIBUTE_WIND),
    }
}
/// `s.tktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,...,sync:GetLevel(),...) end
    if a.chk == Some(0) {
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
            let h = effect::get_handler(d, a.e).nil();
            let sync = card::get_reason_card(d, h);
            let lv = sync_level(d, sync);
            return duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(lv), None, None, 0);
        }
        return false;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, tp, 0);
    true
}
/// `s.tkop(e,tp,eg,ep,ev,re,r,rp)`
fn tkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local sync=c:GetReasonCard()
    let sync = card::get_reason_card(d, c);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsPlayerCanSpecialSummonMonster(...) then
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        let lv = sync_level(d, sync);
        if duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(lv), None, None, 0) {
            // local tk=Duel.CreateToken(tp,id+1)
            let tk = duel::create_token(d, tp, ID + 1).nil();
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CHANGE_LEVEL)
            effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
            // e1:SetValue(sync:GetLevel())
            effect::set_value(d, e1, lv as i64);
            // tk:RegisterEffect(e1)
            card::register_effect(d, tk, e1, false);
            // Duel.SpecialSummon(tk,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, tk, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
