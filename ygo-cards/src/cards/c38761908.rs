//! Paleozoic Dinomischus
use super::prelude::*;

pub const ID: u32 = 38761908;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish 1 card on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_HANDES+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_HANDES + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special summon itself from GY as a monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter1(c)`
fn filter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsDiscardable(REASON_EFFECT)
    card::is_discardable(d, c, Some(REASON_EFFECT), None)
}
/// `s.filter2(c)`
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAbleToRemove()
    card::is_faceup(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // e:GetHandler() is read in every branch below (read once here: it has no side effects)
    let h = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsOnField() and s.filter2(chkc) and chkc~=e:GetHandler() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && filter2(d, chkc) && chkc != h;
    }
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter1,tp,LOCATION_HAND,0,1,e:GetHandler())
    //     and Duel.IsExistingTarget(s.filter2,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter1), tp, LOCATION_HAND, 0, 1, h.into())
            && duel::is_existing_target(d, Some(&filter2), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.filter2,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,e:GetHandler())
    let g = duel::select_target(d, tp, Some(&filter2), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, h.into());
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, tp, 1);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if Duel.DiscardHand(tp,s.filter1,1,1,REASON_EFFECT|REASON_DISCARD,nil)>0 and tc:IsRelateToEffect(e) then
    if duel::discard_hand(d, tp, Some(&filter1), 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX) > 0 && card::is_relate_to_effect(d, tc.nil(), a.e) {
        // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, tc.nil(), POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    // return re:IsTrapEffect() and re:IsHasType(EFFECT_TYPE_ACTIVATE)
    effect::is_trap_effect(d, re) && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:GetFlagEffect(id)==0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id,SET_PALEOZOIC,TYPE_MONSTER|TYPE_NORMAL,1200,0,2,RACE_AQUA,ATTRIBUTE_WATER) end
    if a.chk == Some(0) {
        return card::get_flag_effect(d, c, ID) == 0
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, tp, ID, monster_data(), None, None, 0);
    }
    // c:RegisterFlagEffect(id,RESET_CHAIN,0,1)
    card::register_flag_effect(d, c, ID, RESET_CHAIN, 0, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// The monster `Duel.IsPlayerCanSpecialSummonMonster(tp,id,SET_PALEOZOIC,TYPE_MONSTER|TYPE_NORMAL,1200,0,2,RACE_AQUA,ATTRIBUTE_WATER)` describes.
fn monster_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_PALEOZOIC as u16]),
        type_: Some(TYPE_MONSTER | TYPE_NORMAL),
        attack: Some(1200),
        defense: Some(0),
        level: Some(2),
        race: Some(RACE_AQUA),
        attribute: Some(ATTRIBUTE_WATER),
    }
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.IsPlayerCanSpecialSummonMonster(tp,id,SET_PALEOZOIC,TYPE_MONSTER|TYPE_NORMAL,1200,0,2,RACE_AQUA,ATTRIBUTE_WATER) then
    if !(card::is_relate_to_effect(d, c, a.e) && duel::is_player_can_special_summon_monster(d, tp, ID, monster_data(), None, None, 0)) {
        return;
    }
    // c:AddMonsterAttribute(TYPE_NORMAL)
    card::add_monster_attribute(d, c, TYPE_NORMAL, 0, 0, 0, 0, 0);
    // c:AssumeProperty(ASSUME_RACE,RACE_AQUA)
    card::assume_property(d, c, ASSUME_RACE, RACE_AQUA);
    // Duel.SpecialSummonStep(c,0,tp,tp,true,false,POS_FACEUP)
    duel::special_summon_step(d, c, 0, tp, tp, true, false, POS_FACEUP as u8, None);
    // c:AddMonsterAttributeComplete()
    card::add_monster_attribute_complete(d, c);
    // Unaffected by monster effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(3101)
    effect::set_description(d, e1, 3101);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(s.efilter)
    effect::set_value_fn(d, e1, lua_fn(efilter));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Banish it if it leaves the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(3300)
    effect::set_description(d, e2, 3300);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
    effect::set_code(d, e2, EFFECT_LEAVE_FIELD_REDIRECT);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
    // e2:SetReset(RESET_EVENT|RESETS_REDIRECT)
    effect::set_reset(d, e2, RESET_EVENT | RESETS_REDIRECT, 1);
    // e2:SetValue(LOCATION_REMOVED)
    effect::set_value(d, e2, LOCATION_REMOVED as i64);
    // c:RegisterEffect(e2,true)
    card::register_effect(d, c, e2, true);
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.efilter(e,re)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    // return re:IsMonsterEffect()
    effect::is_monster_effect(d, re)
}
