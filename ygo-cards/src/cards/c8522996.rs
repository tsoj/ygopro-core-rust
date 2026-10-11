//! The First Monarch
use super::prelude::*;

pub const ID: u32 = 8522996;

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
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // change attribute
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCondition(s.chcon)
    effect::set_condition(d, e2, Some(condition(chcon)));
    // e2:SetCost(s.chcost)
    effect::set_cost(d, e2, Some(cost_target(chcost)));
    // e2:SetTarget(s.chtg)
    effect::set_target(d, e2, Some(cost_target(chtg)));
    // e2:SetOperation(s.chop)
    effect::set_operation(d, e2, Some(operation(chop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // sum limit
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e3, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetTargetRange(1,0)
    effect::set_target_range(d, e3, 1, 0);
    // e3:SetCondition(s.chcon)
    effect::set_condition(d, e3, Some(condition(chcon)));
    // e3:SetTarget(s.splimit)
    effect::set_target(d, e3, Some(lua_fn(splimit)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_EFFECT,1000,2400,6,RACE_FIEND,ATTRIBUTE_DARK) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, a.tp, ID, monster_data(), None, None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
// The monster data the Lua call passes to IsPlayerCanSpecialSummonMonster
fn monster_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPE_MONSTER | TYPE_EFFECT),
        attack: Some(1000),
        defense: Some(2400),
        level: Some(6),
        race: Some(RACE_FIEND),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0
    //   or not Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_EFFECT,1000,2400,6,RACE_FIEND,ATTRIBUTE_DARK) then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0
        || !duel::is_player_can_special_summon_monster(d, a.tp, ID, monster_data(), None, None, 0)
    {
        return;
    }
    // c:AddMonsterAttribute(TYPE_EFFECT+TYPE_TRAP)
    card::add_monster_attribute(d, c, TYPE_EFFECT + TYPE_TRAP, 0, 0, 0, 0, 0);
    // Duel.SpecialSummonStep(c,1,tp,tp,true,false,POS_FACEUP)
    duel::special_summon_step(d, c, 1, a.tp, a.tp, true, false, POS_FACEUP as u8, None);
    // c:AddMonsterAttributeComplete()
    card::add_monster_attribute_complete(d, c);
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
// function s.chcon(e,tp,eg,ep,ev,re,r,rp)
fn chcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetSummonType()==SUMMON_TYPE_SPECIAL+1
    let h = effect::get_handler(d, a.e).nil();
    card::get_summon_type(d, h) == SUMMON_TYPE_SPECIAL + 1
}
// function s.chcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn chcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, a.tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
// function s.chtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn chtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,0)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, 0);
    // local aat=Duel.AnnounceAttribute(tp,1,ATTRIBUTE_ALL)
    let aat = duel::announce_attribute(d, a.tp, 1, ATTRIBUTE_ALL);
    // e:SetLabel(aat)
    effect::set_label(d, a.e, &[aat as i64]);
    true
}
// function s.chop(e,tp,eg,ep,ev,re,r,rp)
fn chop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local att=e:GetLabel()
    let att = effect::get_label(d, a.e);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_COPY_INHERIT)
    effect::set_property(d, e1, EFFECT_FLAG_COPY_INHERIT, 0);
    // e1:SetCode(EFFECT_CHANGE_ATTRIBUTE)
    effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
    // e1:SetValue(att)
    effect::set_value(d, e1, att);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // double tribute
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_DOUBLE_TRIBUTE)
    effect::set_code(d, e2, EFFECT_DOUBLE_TRIBUTE);
    // e2:SetValue(s.condition)
    effect::set_value_fn(d, e2, effect_card_fn(double_tribute_value));
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.condition(e,c)
fn double_tribute_value(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsAttribute(e:GetHandler():GetAttribute())
    let h = effect::get_handler(d, e).nil();
    let att = card::get_attribute(d, h);
    card::is_attribute(d, c, att)
}
// function s.splimit(e,c,tp,sumtp,sumpos)
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return c:IsAttribute(ATTRIBUTE_ALL&~(e:GetHandler():GetAttribute()))
    let c = a.c(1);
    let h = effect::get_handler(d, a.e(0)).nil();
    let att = card::get_attribute(d, h);
    card::is_attribute(d, c, ATTRIBUTE_ALL & !att)
}
