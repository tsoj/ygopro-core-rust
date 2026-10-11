//! Oyster Meister
use super::prelude::*;

pub const ID: u32 = 83239739;
pub const TOKEN_OYSTER: u32 = ID + 1;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_OYSTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If this card is sent from the field to the GY, except when destroyed by battle: Special Summon 1 "Oyster Token" (Fish/WATER/Level 1/ATK 0/DEF 0)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetCondition(s.tokencon)
    effect::set_condition(d, e1, Some(condition(tokencon)));
    // e1:SetTarget(s.tokentg)
    effect::set_target(d, e1, Some(cost_target(tokentg)));
    // e1:SetOperation(s.tokenop)
    effect::set_operation(d, e1, Some(operation(tokenop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tokencon(e,tp,eg,ep,ev,re,r,rp)`
fn tokencon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_ONFIELD) and not c:IsReason(REASON_BATTLE)
    card::is_previous_location(d, c, LOCATION_ONFIELD) && !card::is_reason(d, c, REASON_BATTLE)
}
/// `s.tokentg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tokentg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    true
}
/// `Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_OYSTER,0,TYPES_TOKEN,0,0,1,RACE_FISH,ATTRIBUTE_WATER)`
fn can_summon(d: &mut Duel, tp: u8) -> bool {
    duel::is_player_can_special_summon_monster(
        d,
        tp,
        TOKEN_OYSTER,
        duel::MonsterData {
            setcodes: Some(vec![0]),
            type_: Some(TYPES_TOKEN),
            attack: Some(0),
            defense: Some(0),
            level: Some(1),
            race: Some(RACE_FISH),
            attribute: Some(ATTRIBUTE_WATER),
        },
        None,
        None,
        0,
    )
}
/// `s.tokenop(e,tp,eg,ep,ev,re,r,rp)`
fn tokenop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsPlayerCanSpecialSummonMonster(...) then
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && can_summon(d, a.tp) {
        // local token=Duel.CreateToken(tp,TOKEN_OYSTER)
        let token = duel::create_token(d, a.tp, TOKEN_OYSTER).nil();
        // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
