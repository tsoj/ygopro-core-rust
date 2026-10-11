//! Otoshidamashi
use super::prelude::*;

pub const ID: u32 = 14957440;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[14957441], ..ScriptInfo::DEFAULT };
/// `local COUNTER_OTOSHIDAMASHI=0x59`
const COUNTER_OTOSHIDAMASHI: u16 = 0x59;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_OTOSHIDAMASHI,LOCATION_MZONE)
    card::enable_counter_permit(d, c, COUNTER_OTOSHIDAMASHI, Some(LOCATION_MZONE), None);
    // Your opponent's monster cannot target this card for attacks while you control a non-Tuner monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_BATTLE_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_BATTLE_TARGET);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.atklm)
    effect::set_condition(d, e1, Some(lua_fn(|d, a: Args| atklm(d, a.e(0)))));
    // e1:SetValue(aux.imval2)
    effect::set_value_fn(d, e1, aux::imval2());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon "Otoshidamashi Tokens" equal to the number of "Otoshidamashi Counters" on this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_COUNTER+CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e2, CATEGORY_COUNTER + CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.tkcon)
    effect::set_condition(d, e2, Some(condition(tkcon)));
    // e2:SetTarget(s.tktg)
    effect::set_target(d, e2, Some(cost_target(tktg)));
    // e2:SetOperation(s.tkop)
    effect::set_operation(d, e2, Some(operation(tkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() or (c:IsFaceup() and not c:IsType(TYPE_TUNER))
    card::is_facedown(d, c) || (card::is_faceup(d, c) && !card::is_type(d, c, TYPE_TUNER))
}
/// `s.atklm(e)`
fn atklm(d: &mut Duel, e: EffectId) -> bool {
    // return Duel.IsExistingMatchingCard(s.atkfilter,e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    let p = effect::get_handler_player(d, e);
    duel::is_existing_matching_card(d, Some(&atkfilter), p, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(1-tp) and c:IsMonster() and c:IsPreviousLocation(LOCATION_MZONE)
    card::is_controler(d, c, 1 - tp) && card::is_monster(d, c) && card::is_previous_location(d, c, LOCATION_MZONE)
}
/// `s.tkcon(e,tp,eg,ep,ev,re,r,rp)`
fn tkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, tp), 1, NO_EX)
}
/// `s.tktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsCanAddCounter(COUNTER_OTOSHIDAMASHI,1) end
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_can_add_counter(d, h, COUNTER_OTOSHIDAMASHI, 1, false, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, 1, 0, 0);
    true
}
/// `s.tkop(e,tp,eg,ep,ev,re,r,rp)`
fn tkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if c:AddCounter(COUNTER_OTOSHIDAMASHI,1) and ft>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,0,0,1,RACE_BEAST,ATTRIBUTE_EARTH) then
    if card::add_counter(d, c, COUNTER_OTOSHIDAMASHI, 1, false)
        && ft > 0
        && duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), None, None, 0)
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
        if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
            ft = 1;
        }
        // local ct=c:GetCounter(COUNTER_OTOSHIDAMASHI)
        let mut ct = card::get_counter(d, c, COUNTER_OTOSHIDAMASHI);
        // if ct>ft then ct=ft end
        if ct > ft {
            ct = ft;
        }
        // if ct>1 then
        if ct > 1 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NUMBER)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NUMBER as u64);
            // ct=Duel.AnnounceNumberRange(tp,1,ct)
            ct = duel::announce_number_range(d, tp, Some(1), Some(ct as u32), &[]).nil().0 as i32;
        }
        // for i=1,ct do
        for _ in 1..=ct {
            // local token=Duel.CreateToken(tp,id+1)
            let token = duel::create_token(d, tp, ID + 1).nil();
            // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_LEVEL)
            effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
            // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
            effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
            // e1:SetValue(c:GetCounter(COUNTER_OTOSHIDAMASHI))
            let v = card::get_counter(d, c, COUNTER_OTOSHIDAMASHI) as i64;
            effect::set_value(d, e1, v);
            // token:RegisterEffect(e1,true)
            card::register_effect(d, token, e1, true);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_SET_ATTACK)
            effect::set_code(d, e2, EFFECT_SET_ATTACK);
            // e2:SetValue((c:GetCounter(COUNTER_OTOSHIDAMASHI)+1)*500)
            let v = (card::get_counter(d, c, COUNTER_OTOSHIDAMASHI) as i64 + 1) * 500;
            effect::set_value(d, e2, v);
            // e2:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
            effect::set_reset(d, e2, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
            // token:RegisterEffect(e2,true)
            card::register_effect(d, token, e2, true);
            // local e3=Effect.CreateEffect(c)
            let e3 = effect::create_effect(d, c);
            // e3:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            // e3:SetCode(EFFECT_SET_DEFENSE)
            effect::set_code(d, e3, EFFECT_SET_DEFENSE);
            // e3:SetValue((c:GetCounter(COUNTER_OTOSHIDAMASHI)+1)*500)
            let v = (card::get_counter(d, c, COUNTER_OTOSHIDAMASHI) as i64 + 1) * 500;
            effect::set_value(d, e3, v);
            // e3:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
            effect::set_reset(d, e3, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
            // token:RegisterEffect(e3,true)
            card::register_effect(d, token, e3, true);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
/// The token data of "Otoshidamashi Token" (`Duel.IsPlayerCanSpecialSummonMonster` with the Lua arguments).
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(1),
        race: Some(RACE_BEAST),
        attribute: Some(ATTRIBUTE_EARTH),
    }
}
