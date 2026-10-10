//! The Phantom Knights of Shade Brigandine
//! 幻影騎士団シェード・ブリガンダイン
use super::prelude::*;

pub const ID: u32 = 98827725;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Can be activated the turn it was Set
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_TRAP_ACT_IN_SET_TURN)
    effect::set_code(d, e2, EFFECT_TRAP_ACT_IN_SET_TURN);
    // e2:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e2, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e2:SetCondition(s.actcon)
    effect::set_condition(d, e2, Some(condition(actcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// The `Duel.IsPlayerCanSpecialSummonMonster(tp,id,SET_THE_PHANTOM_KNIGHTS,TYPE_MONSTER+TYPE_NORMAL,0,300,4,RACE_WARRIOR,ATTRIBUTE_DARK)` check.
fn can_summon(d: &mut Duel, tp: u8) -> bool {
    duel::is_player_can_special_summon_monster(
        d,
        tp,
        ID,
        duel::MonsterData {
            setcodes: Some(vec![SET_THE_PHANTOM_KNIGHTS as u16]),
            type_: Some(TYPE_MONSTER + TYPE_NORMAL),
            attack: Some(0),
            defense: Some(300),
            level: Some(4),
            race: Some(RACE_WARRIOR),
            attribute: Some(ATTRIBUTE_DARK),
        },
        None,
        None,
        0,
    )
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and e:IsHasType(EFFECT_TYPE_ACTIVATE)
    //     and Duel.IsPlayerCanSpecialSummonMonster(...) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE)
            && can_summon(d, tp);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, h.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.IsPlayerCanSpecialSummonMonster(...) then
    if card::is_relate_to_effect(d, c, a.e) && can_summon(d, tp) {
        // c:AddMonsterAttribute(TYPE_NORMAL)
        card::add_monster_attribute(d, c, TYPE_NORMAL, 0, 0, 0, 0, 0);
        // Duel.SpecialSummonStep(c,0,tp,tp,true,false,POS_FACEUP_DEFENSE)
        duel::special_summon_step(d, c, 0, tp, tp, true, false, POS_FACEUP_DEFENSE as u8, None);
        // c:AddMonsterAttributeComplete()
        card::add_monster_attribute_complete(d, c);
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
/// `s.actcon(e)`
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(Card.IsTrap,e:GetHandlerPlayer(),LOCATION_GRAVE,0,1,nil)
    let p = effect::get_handler_player(d, a.e);
    !duel::is_existing_matching_card(d, Some(&|d, c| card::is_trap(d, c)), p, LOCATION_GRAVE, 0, 1, NO_EX)
}
