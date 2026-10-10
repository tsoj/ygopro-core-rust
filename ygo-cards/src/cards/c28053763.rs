//! Dragonoid Generator
use super::prelude::*;

pub const ID: u32 = 28053763;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[28053764], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(Cost.PayLP(1000))
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(2,id)
    effect::set_count_limit(d, e2, 2, ID, 0, 0);
    // e2:SetTarget(s.tktg)
    effect::set_target(d, e2, Some(cost_target(tktg)));
    // e2:SetOperation(s.tkop)
    effect::set_operation(d, e2, Some(operation(tkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,300,300,1,RACE_MACHINE,ATTRIBUTE_EARTH,POS_FACEUP_ATTACK)`
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(300),
        defense: Some(300),
        level: Some(1),
        race: Some(RACE_MACHINE),
        attribute: Some(ATTRIBUTE_EARTH),
    }
}
/// `s.tktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(...) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), Some(POS_FACEUP_ATTACK as u8), None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    true
}
/// `s.tkop(e,tp,eg,ep,ev,re,r,rp)`
fn tkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsLocation,LOCATION_EXTRA))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // aux.addTempLizardCheck(e:GetHandler(),tp)
    aux::add_temp_lizard_check(d, c, a.tp, None, None, None, None, None);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0
    //     or not Duel.IsPlayerCanSpecialSummonMonster(...) then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0
        || !duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), Some(POS_FACEUP_ATTACK as u8), None, 0)
    {
        return;
    }
    // local token=Duel.CreateToken(tp,id+1)
    let token = duel::create_token(d, a.tp, ID + 1).nil();
    // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP_ATTACK)
    duel::special_summon(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP_ATTACK as u8, None);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetOperation(s.tkop2)
    effect::set_operation(d, e2, Some(operation(tkop2)));
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
}
/// `s.tkop2(e,tp,eg,ep,ev,re,r,rp)`
fn tkop2(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(1-tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // if not Duel.IsPlayerCanSpecialSummonMonster(1-tp,...) then return end
    if !duel::is_player_can_special_summon_monster(d, 1 - a.tp, ID + 1, token_data(), Some(POS_FACEUP_ATTACK as u8), None, 0) {
        return;
    }
    // Duel.Hint(HINT_CARD,PLAYER_ALL,id)
    duel::hint(d, HINT_CARD as u8, PLAYER_ALL as u8, ID as u64);
    // local token=Duel.CreateToken(1-tp,id+1)
    let token = duel::create_token(d, 1 - a.tp, ID + 1).nil();
    // Duel.SpecialSummon(token,0,1-tp,1-tp,false,false,POS_FACEUP_ATTACK)
    duel::special_summon(d, token, 0, 1 - a.tp, 1 - a.tp, false, false, POS_FACEUP_ATTACK as u8, None);
}
