//! Nine-Tailed Fox
use super::prelude::*;

pub const ID: u32 = 27198001;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[27198002], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_GRAVE|LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_GRAVE | LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // pierce
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e2, EFFECT_PIERCE);
    // e2:SetCondition(s.pcon)
    effect::set_condition(d, e2, Some(condition(pcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // token
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCondition(s.condition)
    effect::set_condition(d, e3, Some(condition(cond)));
    // e3:SetTarget(s.target)
    effect::set_target(d, e3, Some(cost_target(target)));
    // e3:SetOperation(s.operation)
    effect::set_operation(d, e3, Some(operation(op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,nil,2,false,aux.ReleaseCheckMMZ,nil) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, None, 2, 2, false, Some(&|d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| aux::release_check_mmz(d, sg, tp, exg)), NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,nil,2,2,false,aux.ReleaseCheckMMZ,nil)
    let g = duel::select_release_group_cost(d, a.tp, None, 2, 2, false, Some(&|d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| aux::release_check_mmz(d, sg, tp, exg)), NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler() ... (the Lua reads e:GetHandler() directly)
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.pcon(e)`
fn pcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetPreviousLocation()==LOCATION_GRAVE
    let h = effect::get_handler(d, a.e).nil();
    card::get_previous_location(d, h) == LOCATION_GRAVE
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsReason(REASON_DESTROY) and c:IsReason(REASON_BATTLE|REASON_EFFECT)
    card::is_reason(d, c, REASON_DESTROY) && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
}
/// The `Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,500,500,2,RACE_ZOMBIE,ATTRIBUTE_FIRE)` data
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(500),
        defense: Some(500),
        level: Some(2),
        race: Some(RACE_ZOMBIE),
        attribute: Some(ATTRIBUTE_FIRE),
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>1 and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,500,500,2,RACE_ZOMBIE,ATTRIBUTE_FIRE) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 1
            && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), None, None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 2, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft>1 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if ft > 1 && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // if ft<2 then return end
    if ft < 2 {
        return;
    }
    // if not Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,500,500,2,RACE_ZOMBIE,ATTRIBUTE_FIRE) then return end
    if !duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), None, None, 0) {
        return;
    }
    // for i=1,2 do
    for _i in 1..=2 {
        // local token=Duel.CreateToken(tp,id+1)
        let token = duel::create_token(d, a.tp, ID + 1).nil();
        // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
