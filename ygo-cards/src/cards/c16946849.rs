//! Synthetic Seraphim
use super::prelude::*;

pub const ID: u32 = 16946849;

pub const TOKEN_SERAPHIM: u32 = ID + 1;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
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
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e2, EVENT_CHAIN_SOLVED);
    // e2:SetCondition(s.tokencon)
    effect::set_condition(d, e2, Some(condition(tokencon)));
    // e2:SetOperation(s.tokenop)
    effect::set_operation(d, e2, Some(operation(tokenop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.tokencon(e,tp,eg,ep,ev,re,r,rp)`
fn tokencon(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsActiveType(TYPE_COUNTER) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,300,300,1,RACE_FAIRY,ATTRIBUTE_LIGHT)
    effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE)
        && effect::is_active_type(d, a.re.nil(), TYPE_COUNTER)
        && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_player_can_special_summon_monster(
            d,
            a.tp,
            TOKEN_SERAPHIM,
            duel::MonsterData {
                setcodes: Some(vec![0]),
                type_: Some(TYPES_TOKEN),
                attack: Some(300),
                defense: Some(300),
                level: Some(1),
                race: Some(RACE_FAIRY),
                attribute: Some(ATTRIBUTE_LIGHT),
            },
            None,
            None,
            0,
        )
}
/// `s.tokenop(e,tp,eg,ep,ev,re,r,rp)`
fn tokenop(d: &mut Duel, a: &EffArgs) {
    // local token=Duel.CreateToken(tp,id+1)
    let token = duel::create_token(d, a.tp, TOKEN_SERAPHIM).nil();
    // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
