//! Graveyard of Wandering Souls
use super::prelude::*;

pub const ID: u32 = 98596596;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_FIREBALL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // token
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.tkcon1)
    effect::set_condition(d, e2, Some(condition(tkcon1)));
    // e2:SetTarget(s.tktg)
    effect::set_target(d, e2, Some(cost_target(tktg)));
    // e2:SetOperation(s.tkop1)
    effect::set_operation(d, e2, Some(operation(tkop1)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // tokens
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCondition(s.tkcon2)
    effect::set_condition(d, e3, Some(condition(tkcon2)));
    // e3:SetTarget(s.tktg)
    effect::set_target(d, e3, Some(cost_target(tktg)));
    // e3:SetOperation(s.tkop2)
    effect::set_operation(d, e3, Some(operation(tkop2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_FIREBALL,0,TYPES_TOKEN,100,100,1,RACE_PYRO,ATTRIBUTE_FIRE)`
fn can_summon_fireball(d: &mut Duel, tp: u8) -> bool {
    duel::is_player_can_special_summon_monster(
        d,
        tp,
        TOKEN_FIREBALL,
        duel::MonsterData {
            setcodes: Some(vec![0]),
            type_: Some(TYPES_TOKEN),
            attack: Some(100),
            defense: Some(100),
            level: Some(1),
            race: Some(RACE_PYRO),
            attribute: Some(ATTRIBUTE_FIRE),
        },
        None,
        None,
        0,
    )
}
// function s.cfilter(c,tp)
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsReason(REASON_BATTLE) and c:IsLocation(LOCATION_GRAVE) and c:IsPreviousControler(tp)
    card::is_reason(d, c, REASON_BATTLE) && card::is_location(d, c, LOCATION_GRAVE) && card::is_previous_controler(d, c, tp)
}
// function s.tkcon1(e,tp,eg,ep,ev,re,r,rp)
fn tkcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, tp), 1, NO_EX)
}
// function s.tktg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_FIREBALL,0,TYPES_TOKEN,100,100,1,RACE_PYRO,ATTRIBUTE_FIRE) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && can_summon_fireball(d, tp);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    true
}
// function s.tkop1(e,tp,eg,ep,ev,re,r,rp)
fn tkop1(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_FIREBALL,...) then
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && can_summon_fireball(d, tp) {
        // local token=Duel.CreateToken(tp,TOKEN_FIREBALL)
        let token = duel::create_token(d, tp, TOKEN_FIREBALL).nil();
        // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
// function s.tkcfilter(c,tp)
fn tkcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsMonster() and c:IsPreviousLocation(LOCATION_HAND|LOCATION_ONFIELD) and c:IsPreviousControler(tp)
    //     and c:GetReasonPlayer()~=tp and c:IsReason(REASON_EFFECT)
    card::is_monster(d, c)
        && card::is_previous_location(d, c, LOCATION_HAND | LOCATION_ONFIELD)
        && card::is_previous_controler(d, c, tp)
        && card::get_reason_player(d, c) != tp
        && card::is_reason(d, c, REASON_EFFECT)
}
// function s.tkcon2(e,tp,eg,ep,ev,re,r,rp)
fn tkcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.tkcfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| tkcfilter(d, c, tp), 1, NO_EX)
}
// function s.tkop2(e,tp,eg,ep,ev,re,r,rp)
fn tkop2(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=eg:FilterCount(s.tkcfilter,nil,tp)
    let eg = a.eg.clone().nil();
    let ct = group::filter_count(d, &eg, &|d, c| tkcfilter(d, c, tp), NO_EX) as i32;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // ft=math.min(ct,ft)
    let mut ft = ct.min(ft);
    // if ft<1 or not Duel.IsPlayerCanSpecialSummonMonster(tp,23116809,0,TYPES_TOKEN,100,100,1,RACE_PYRO,ATTRIBUTE_FIRE) then return end
    if ft < 1
        || !duel::is_player_can_special_summon_monster(
            d,
            tp,
            23116809,
            duel::MonsterData {
                setcodes: Some(vec![0]),
                type_: Some(TYPES_TOKEN),
                attack: Some(100),
                defense: Some(100),
                level: Some(1),
                race: Some(RACE_PYRO),
                attribute: Some(ATTRIBUTE_FIRE),
            },
            None,
            None,
            0,
        )
    {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // for i=1,ft do
    for _ in 0..ft {
        // local token=Duel.CreateToken(tp,23116809)
        let token = duel::create_token(d, tp, 23116809).nil();
        // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
