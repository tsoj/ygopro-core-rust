//! Traptrix Holeutea
use super::prelude::*;

pub const ID: u32 = 74577599;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TRAPTRIX], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card as a Normal Monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(aux.Stringid(id,0))
    effect::set_category(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.selfspcost)
    effect::set_cost(d, e1, Some(cost_target(selfspcost)));
    // e1:SetTarget(s.selfsptg)
    effect::set_target(d, e1, Some(cost_target(selfsptg)));
    // e1:SetOperation(s.selfspop)
    effect::set_operation(d, e1, Some(operation(selfspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Can be activated the turn it was Set by discarding 1 Normal Trap
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e2, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e2:SetCode(EFFECT_TRAP_ACT_IN_SET_TURN)
    effect::set_code(d, e2, EFFECT_TRAP_ACT_IN_SET_TURN);
    // e2:SetValue(function(e,c) e:SetLabel(1) end)
    effect::set_value_fn(d, e2, lua_fn(e2_value));
    // e2:SetCondition(function(e) return Duel.IsExistingMatchingCard(s.selfspcostfilter,e:GetHandlerPlayer(),LOCATION_HAND,0,1,nil) end)
    effect::set_condition(d, e2, Some(lua_fn(e2_cond)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // e1:SetLabelObject(e2)
    effect::set_label_object(d, e1, Value::Effect(e2));
    // Special Summon 1 "Traptrix" monster from your GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e3, Some(cost::self_banish()));
    // e3:SetTarget(s.gysptg)
    effect::set_target(d, e3, Some(cost_target(gysptg)));
    // e3:SetOperation(s.gyspop)
    effect::set_operation(d, e3, Some(operation(gyspop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.selfspcostfilter(c)`
fn selfspcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsNormalTrap() and c:IsDiscardable()
    card::is_normal_trap(d, c) && card::is_discardable(d, c, None, None)
}
/// `e2:SetValue(function(e,c) e:SetLabel(1) end)`
fn e2_value(d: &mut Duel, a: Args) {
    // e:SetLabel(1)
    effect::set_label(d, a.e(0), &[1]);
}
/// `e2:SetCondition(function(e) return Duel.IsExistingMatchingCard(...) end)`
fn e2_cond(d: &mut Duel, a: Args) -> bool {
    // e:GetHandlerPlayer()
    let p = effect::get_handler_player(d, a.e(0));
    // Duel.IsExistingMatchingCard(s.selfspcostfilter,p,LOCATION_HAND,0,1,nil)
    duel::is_existing_matching_card(d, Some(&selfspcostfilter), p, LOCATION_HAND, 0, 1, NO_EX)
}
/// `s.selfspcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfspcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local label_obj=e:GetLabelObject()
    let label_obj = effect::get_label_object(d, a.e).as_effect().nil();
    // if chk==0 then label_obj:SetLabel(0) return true end
    if a.chk == Some(0) {
        effect::set_label(d, label_obj, &[0]);
        return true;
    }
    // if label_obj:GetLabel()>0 then
    if effect::get_label(d, label_obj) > 0 {
        // label_obj:SetLabel(0)
        effect::set_label(d, label_obj, &[0]);
        // Duel.DiscardHand(tp,s.selfspcostfilter,1,1,REASON_COST|REASON_DISCARD)
        duel::discard_hand(d, a.tp, Some(&selfspcostfilter), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    }
    true
}
/// `Duel.IsPlayerCanSpecialSummonMonster(tp,id,SET_TRAPTRIX,TYPE_MONSTER|TYPE_NORMAL,400,2400,4,RACE_PLANT,ATTRIBUTE_EARTH,POS_FACEUP_DEFENSE)`
fn can_summon(d: &mut Duel, tp: u8) -> bool {
    duel::is_player_can_special_summon_monster(
        d,
        tp,
        ID,
        duel::MonsterData {
            setcodes: Some(vec![SET_TRAPTRIX as u16]),
            type_: Some(TYPE_MONSTER | TYPE_NORMAL),
            attack: Some(400),
            defense: Some(2400),
            level: Some(4),
            race: Some(RACE_PLANT),
            attribute: Some(ATTRIBUTE_EARTH),
        },
        Some(POS_FACEUP_DEFENSE as u8),
        None,
        0,
    )
}
/// `s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and e:IsHasType(EFFECT_TYPE_ACTIVATE) and Duel.IsPlayerCanSpecialSummonMonster(...) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE)
            && can_summon(d, tp);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, tp, 0);
    true
}
/// `s.selfspop(e,tp,eg,ep,ev,re,r,rp)`
fn selfspop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.IsPlayerCanSpecialSummonMonster(...) then
    if card::is_relate_to_effect(d, c, a.e) && can_summon(d, a.tp) {
        // c:AddMonsterAttribute(TYPE_NORMAL)
        card::add_monster_attribute(d, c, TYPE_NORMAL, 0, 0, 0, 0, 0);
        // Duel.SpecialSummonStep(c,0,tp,tp,true,false,POS_FACEUP_DEFENSE)
        duel::special_summon_step(d, c, 0, a.tp, a.tp, true, false, POS_FACEUP_DEFENSE as u8, None);
        // c:AddMonsterAttributeComplete()
        card::add_monster_attribute_complete(d, c);
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
/// `s.gyspfilter(c,e,tp)`
fn gyspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_TRAPTRIX) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_TRAPTRIX as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.gysptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn gysptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.gyspfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && gyspfilter(d, chkc, a.e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.gyspfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| gyspfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.gyspfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| gyspfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, 0);
    true
}
/// `s.gyspop(e,tp,eg,ep,ev,re,r,rp)`
fn gyspop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
