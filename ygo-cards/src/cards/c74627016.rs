//! T.G. Tank Grub
use super::prelude::*;

pub const ID: u32 = 74627016;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[74627017],
    listed_series: &[SET_TG],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Can be treated as a non-Tuner for the Synchro Summon of a "T.G." monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_NONTUNER)
    effect::set_code(d, e1, EFFECT_NONTUNER);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(function(e,sc) return sc:IsSetCard(SET_TG) end)
    effect::set_value_fn(d, e1, effect_card_fn(|d, _e, sc| card::is_set_card(d, sc, &[SET_TG as u16])));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "T.G. Token"
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
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
// function s.tkcon(e,tp,eg,ep,ev,re,r,rp)
fn tkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsLocation(LOCATION_GRAVE) and r==REASON_SYNCHRO and c:GetReasonCard():IsSetCard(SET_TG)
    card::is_location(d, c, LOCATION_GRAVE)
        && a.r as i64 == REASON_SYNCHRO as i64
        && card::is_set_card(d, card::get_reason_card(d, c).nil(), &[SET_TG as u16])
}
// function s.tktg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_TG,TYPES_TOKEN,0,0,1,RACE_MACHINE,ATTRIBUTE_EARTH,POS_FACEUP_ATTACK) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && token_summonable(d, a.tp);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 0, 0);
    true
}
// Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_TG,TYPES_TOKEN,0,0,1,RACE_MACHINE,ATTRIBUTE_EARTH,POS_FACEUP_ATTACK)
fn token_summonable(d: &mut Duel, tp: u8) -> bool {
    duel::is_player_can_special_summon_monster(
        d,
        tp,
        ID + 1,
        duel::MonsterData {
            setcodes: Some(vec![SET_TG as u16]),
            type_: Some(TYPES_TOKEN),
            attack: Some(0),
            defense: Some(0),
            level: Some(1),
            race: Some(RACE_MACHINE),
            attribute: Some(ATTRIBUTE_EARTH),
        },
        Some(POS_FACEUP_ATTACK as u8),
        None,
        0,
    )
}
// function s.tkop(e,tp,eg,ep,ev,re,r,rp)
fn tkop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // if Duel.IsPlayerCanSpecialSummonMonster(...) then
    if token_summonable(d, a.tp) {
        // local token=Duel.CreateToken(tp,id+1)
        let token = duel::create_token(d, a.tp, ID + 1).nil();
        // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP_ATTACK)
        duel::special_summon(d, CardSet::from_card(token), 0, a.tp, a.tp, false, false, POS_FACEUP_ATTACK as u8, None);
    }
}
