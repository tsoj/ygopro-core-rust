//! Jurrac Spinos
use super::prelude::*;

pub const ID: u32 = 44689688;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[44689689], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYED);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg);
    // return #eg==1 and tc:GetReasonCard()==e:GetHandler() and tc:IsLocation(LOCATION_GRAVE) and tc:IsReason(REASON_BATTLE)
    if eg.len() != 1 {
        return false;
    }
    let tc = tc.nil();
    let h = effect::get_handler(d, a.e);
    card::get_reason_card(d, tc) == h && card::is_location(d, tc, LOCATION_GRAVE) && card::is_reason(d, tc, REASON_BATTLE)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    let tp = a.tp;
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,300,0,1,RACE_DINOSAUR,ATTRIBUTE_FIRE,POS_FACEUP_ATTACK,1-tp) then
    if duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), None, None).0 > 0
        && duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), Some(POS_FACEUP_ATTACK as u8), Some(1 - tp), 0)
    {
        // local token=Duel.CreateToken(tp,id+1)
        let token = duel::create_token(d, tp, ID + 1).nil();
        // Duel.SpecialSummon(token,0,tp,1-tp,false,false,POS_FACEUP_ATTACK)
        duel::special_summon(d, token, 0, tp, 1 - tp, false, false, POS_FACEUP_ATTACK as u8, None);
    }
}
// the token: Duel.IsPlayerCanSpecialSummonMonster(..., TYPES_TOKEN, 300, 0, 1, RACE_DINOSAUR, ATTRIBUTE_FIRE, ...)
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(300),
        defense: Some(0),
        level: Some(1),
        race: Some(RACE_DINOSAUR),
        attribute: Some(ATTRIBUTE_FIRE),
    }
}
