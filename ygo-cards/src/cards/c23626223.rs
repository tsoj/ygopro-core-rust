//! Statue of Anguish Pattern
use super::prelude::*;

pub const ID: u32 = 23626223;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card as an Effect Monster (Rock-Type/EARTH/Level 7/ATK 0/DEF 2500) (This card is also still a Trap Card.)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END,TIMINGS_CHECK_MONSTER_E)
    // (the Lua call's third argument is ignored by the core: only two timings are read)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If Summoned this way, this card cannot be targeted by an opponent's card effects while either player controls another Trap Card that is a monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(function(e) local c=e:GetHandler() return c:IsSummonType(SUMMON_TYPE_SPECIAL+1) and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsTrapCard),0,LOCATION_MZONE,LOCATION_MZONE,1,c) end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let c = effect::get_handler(d, a.e).nil();
        card::is_summon_type(d, c, &[SUMMON_TYPE_SPECIAL + 1])
            && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, x| card::is_trap_card(d, x))), 0, LOCATION_MZONE, LOCATION_MZONE, 1, c.into())
    })));
    // e2:SetValue(aux.tgoval)
    effect::set_value_fn(d, e2, aux::tgoval());
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy 1 card on the field
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.descon)
    effect::set_condition(d, e3, Some(condition(descon)));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// The monster this card becomes when it is Special Summoned (`Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_EFFECT,0,2500,7,RACE_ROCK,ATTRIBUTE_EARTH)`)
fn statue_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPE_MONSTER | TYPE_EFFECT),
        attack: Some(0),
        defense: Some(2500),
        level: Some(7),
        race: Some(RACE_ROCK),
        attribute: Some(ATTRIBUTE_EARTH),
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_EFFECT,0,2500,7,RACE_ROCK,ATTRIBUTE_EARTH) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, a.tp, ID, statue_data(), None, None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, h.map(CardSet::from_card), 1, a.tp, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_EFFECT,0,2500,7,RACE_ROCK,ATTRIBUTE_EARTH) then
    if card::is_relate_to_effect(d, c, a.e) && duel::is_player_can_special_summon_monster(d, a.tp, ID, statue_data(), None, None, 0) {
        // c:AddMonsterAttribute(TYPE_EFFECT|TYPE_TRAP)
        card::add_monster_attribute(d, c, TYPE_EFFECT | TYPE_TRAP, 0, 0, 0, 0, 0);
        // Duel.SpecialSummonStep(c,1,tp,tp,true,false,POS_FACEUP)
        duel::special_summon_step(d, c, 1, a.tp, a.tp, true, false, POS_FACEUP as u8, None);
        // c:AddMonsterAttributeComplete()
        card::add_monster_attribute_complete(d, c);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.desconfilter(c,tp)`
fn desconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSummonLocation(LOCATION_STZONE) and c:IsPreviousControler(tp)
    card::is_summon_location(d, c, LOCATION_STZONE) && card::is_previous_controler(d, c, tp)
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsSummonType(SUMMON_TYPE_SPECIAL+1) and not eg:IsContains(c) and eg:IsExists(s.desconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    card::is_summon_type(d, c, &[SUMMON_TYPE_SPECIAL + 1])
        && !group::is_contains(&eg, c)
        && group::is_exists(d, &eg, &|d, x| desconfilter(d, x, tp), 1, NO_EX)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
    }
}
