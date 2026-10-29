//! Stronghold the Hidden Fortress
use super::prelude::*;

pub const ID: u32 = 27157727;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_SHINING_SARCOPHAGUS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
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
    // e1:SetHintTiming(0,TIMING_BATTLE_START|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_BATTLE_START | TIMING_END_PHASE));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_EFFECT|TYPE_MONSTER,0,2000,4,RACE_MACHINE,ATTRIBUTE_EARTH)`
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPE_EFFECT | TYPE_MONSTER),
        attack: Some(0),
        defense: Some(2000),
        level: Some(4),
        race: Some(RACE_MACHINE),
        attribute: Some(ATTRIBUTE_EARTH),
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and
    //     Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_EFFECT|TYPE_MONSTER,0,2000,4,RACE_MACHINE,ATTRIBUTE_EARTH) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, tp, ID, token_data(), Some(POS_FACEUP as u8), None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, e);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, h.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) or Duel.GetLocationCount(tp,LOCATION_MZONE)<=0
    //     or not Duel.IsPlayerCanSpecialSummonMonster(...) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    if !duel::is_player_can_special_summon_monster(d, tp, ID, token_data(), Some(POS_FACEUP as u8), None, 0) {
        return;
    }
    // c:AddMonsterAttribute(TYPE_EFFECT|TYPE_TRAP)
    card::add_monster_attribute(d, c, TYPE_EFFECT | TYPE_TRAP, 0, 0, 0, 0, 0);
    // Duel.SpecialSummonStep(c,0,tp,tp,true,false,POS_FACEUP)
    duel::special_summon_step(d, c, 0, tp, tp, true, false, POS_FACEUP as u8, None);
    // c:AddMonsterAttributeComplete()
    card::add_monster_attribute_complete(d, c);
    // Gains 1000 ATK for each card you control that is "Shining Sarcophagus" or a monster that mentions it
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_value(atkval));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e1,true)
    card::register_effect(d, c, e1, true);
    // Destroy an opponent's attacking monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e2,true)
    card::register_effect(d, c, e2, true);
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and (c:IsCode(CARD_SHINING_SARCOPHAGUS) or (c:IsMonster() and c:ListsCode(CARD_SHINING_SARCOPHAGUS)))
    card::is_faceup(d, c)
        && (card::is_code(d, c, &[CARD_SHINING_SARCOPHAGUS]) || (card::is_monster(d, c) && card::lists_code(d, c, &[CARD_SHINING_SARCOPHAGUS])))
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    // return Duel.GetFieldGroup(e:GetHandlerPlayer(),LOCATION_ONFIELD,0):FilterCount(s.atkfilter,nil)*1000
    let p = effect::get_handler_player(d, e);
    let g = duel::get_field_group(d, p, LOCATION_ONFIELD, 0);
    group::filter_count(d, &g, &atkfilter, NO_EX) as i64 * 1000
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return Duel.GetAttacker():IsControler(1-tp)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_SHINING_SARCOPHAGUS),tp,LOCATION_ONFIELD,0,1,nil)
    let at = duel::get_attacker(d).nil();
    card::is_controler(d, at, 1 - tp)
        && duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_SHINING_SARCOPHAGUS])),
            tp,
            LOCATION_ONFIELD,
            0,
            1,
            NO_EX,
        )
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local at=Duel.GetAttacker()
    let at = duel::get_attacker(d);
    // if chk==0 then return at:IsRelateToBattle() end
    if a.chk == Some(0) {
        return card::is_relate_to_battle(d, at.nil());
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,at,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, at.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, _a: &EffArgs) {
    // local at=Duel.GetAttacker()
    let at = duel::get_attacker(d).nil();
    // if at:IsRelateToBattle() then
    if card::is_relate_to_battle(d, at) {
        // Duel.Destroy(at,REASON_EFFECT)
        duel::destroy(d, at, REASON_EFFECT, None, None);
    }
}
