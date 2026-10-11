//! Recette de Personnel (Staff Recipe)
//! Scripted by Raye
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 40551410;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID + 1], listed_series: &[SET_NOUVELLES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Special Summon 1 "Nouvelles Token"
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.tokentg)
    effect::set_target(d, e1, Some(cost_target(tokentg)));
    // e1:SetOperation(s.tokenop)
    effect::set_operation(d, e1, Some(operation(tokenop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local rparams={filter=aux.FilterBoolFunction(Card.IsSetCard,SET_NOUVELLES),lvtype=RITPROC_EQUAL,forcedselection=s.forced}
    let rparams = ritual::RitualProc {
        lvtype: RITPROC_EQUAL,
        filter: Some(Arc::new(|d, c| card::is_set_card(d, c, &[SET_NOUVELLES as u16]))),
        forcedselection: Some(Arc::new(forced)),
        ..Default::default()
    };
    // Ritual Summon 1 "Nouvelles" Ritual Monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_RELEASE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_RELEASE + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(s.rtcost)
    effect::set_cost(d, e2, Some(cost_target(rtcost)));
    // e2:SetTarget(Ritual.Target(rparams))
    let target = ritual::target(d, rparams.clone());
    effect::set_target(d, e2, Some(target));
    // e2:SetOperation(Ritual.Operation(rparams))
    let op = ritual::operation(d, rparams);
    effect::set_operation(d, e2, Some(op));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.token_data`: `Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_NOUVELLES,TYPES_TOKEN,50,50,1,RACE_FIEND,ATTRIBUTE_DARK)`
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_NOUVELLES as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(50),
        defense: Some(50),
        level: Some(1),
        race: Some(RACE_FIEND as u64),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
/// `s.tokentg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tokentg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() and chkc:IsRitualMonster() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc) && card::is_ritual_monster(d, chkc);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_NOUVELLES,TYPES_TOKEN,50,50,1,RACE_FIEND,ATTRIBUTE_DARK)
    //     and Duel.IsExistingTarget(aux.FaceupFilter(Card.IsRitualMonster),tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), None, None, 0)
            && duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c) && card::is_ritual_monster(d, c)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsRitualMonster),tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c) && card::is_ritual_monster(d, c)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, a.tp, 0);
    true
}
/// `s.tokenop(e,tp,eg,ep,ev,re,r,rp)`
fn tokenop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0
    //     or not Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_NOUVELLES,TYPES_TOKEN,50,50,1,RACE_FIEND,ATTRIBUTE_DARK) then return false end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 || !duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), None, None, 0) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local token=Duel.CreateToken(tp,id+1)
    let token = duel::create_token(d, a.tp, ID + 1).nil();
    // if Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP) and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if duel::special_summon_step(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None)
        && card::is_relate_to_effect(d, tc.nil(), a.e)
        && card::is_faceup(d, tc.nil())
    {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_LEVEL)
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
        // e1:SetValue(tc:GetLevel())
        let level = card::get_level(d, tc.nil());
        effect::set_value(d, e1, level as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // token:RegisterEffect(e1)
        card::register_effect(d, token, e1, false);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.rtcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rtcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToGraveAsCost() and c:IsStatus(STATUS_EFFECT_ENABLED) end
    if a.chk == Some(0) {
        return card::is_able_to_grave_as_cost(d, c) && card::is_status(d, c, STATUS_EFFECT_ENABLED);
    }
    // Duel.SendtoGrave(c,REASON_COST)
    duel::send_to_grave(d, CardSet::from_card(c), REASON_COST, None, None);
    true
}
/// `s.forced(e,tp,g,sc)`
fn forced(d: &mut Duel, e: EffectId, _tp: u8, g: &CardSet, _sc: CardId) -> (bool, bool) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // return not g:IsContains(c),g:IsContains(c)
    (!group::is_contains(g, c), group::is_contains(g, c))
}
