//! Kuribon the Fairy Spirit
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 34873741;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ANCIENT_FAIRY_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand, and if you do, it is treated as a Tuner
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If another "Ancient Fairy Dragon", or monster(s) that mentions it, that you control would be destroyed by card effect, you can return this card from the field to the hand instead
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.reptg)
    effect::set_target(d, e2, Some(cost_target(reptg)));
    // e2:SetValue(function(e,c) return s.repfilter(c,e:GetHandlerPlayer()) and c~=e:GetHandler() end)
    effect::set_value_fn(d, e2, effect_card_fn(|d, e, c| {
        let hp = effect::get_handler_player(d, e);
        repfilter(d, c, hp) && Some(c) != effect::get_handler(d, e)
    }));
    // e2:SetOperation(function(e) Duel.SendtoHand(e:GetHandler(),nil,REASON_EFFECT|REASON_REPLACE) end)
    effect::set_operation(d, e2, Some(operation(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        duel::send_to_hand(d, h, None, REASON_EFFECT | REASON_REPLACE, None);
    })));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.spconfilter(c)
fn spconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and (c:IsCode(CARD_ANCIENT_FAIRY_DRAGON)
    //     or (c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_BEAST|RACE_PLANT|RACE_FAIRY) and c:IsMonster()))
    card::is_faceup(d, c)
        && (card::is_code(d, c, &[CARD_ANCIENT_FAIRY_DRAGON])
            || (card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_race(d, c, (RACE_BEAST | RACE_PLANT | RACE_FAIRY) as u64) && card::is_monster(d, c)))
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.spconfilter,tp,LOCATION_ONFIELD,0,1,nil)
    duel::is_existing_matching_card(d, Some(&spconfilter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummonStep(c,0,tp,tp,false,false,POS_FACEUP) then
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon_step(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) {
        // It is treated as a Tuner
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_ADD_TYPE)
        effect::set_code(d, e1, EFFECT_ADD_TYPE);
        // e1:SetValue(TYPE_TUNER)
        effect::set_value(d, e1, TYPE_TUNER as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
// function s.repfilter(c,tp)
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return (c:IsCode(CARD_ANCIENT_FAIRY_DRAGON) or (c:ListsCode(CARD_ANCIENT_FAIRY_DRAGON) and c:IsMonster()))
    //     and c:IsControler(tp) and c:IsFaceup() and c:IsOnField()
    //     and c:IsReason(REASON_EFFECT) and not c:IsReason(REASON_REPLACE)
    (card::is_code(d, c, &[CARD_ANCIENT_FAIRY_DRAGON]) || (card::lists_code(d, c, &[CARD_ANCIENT_FAIRY_DRAGON]) && card::is_monster(d, c)))
        && card::is_controler(d, c, tp)
        && card::is_faceup(d, c)
        && card::is_on_field(d, c)
        && card::is_reason(d, c, REASON_EFFECT)
        && !card::is_reason(d, c, REASON_REPLACE)
}
// function s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() and not eg:IsContains(c) and eg:IsExists(s.repfilter,1,c,tp) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return card::is_able_to_hand(d, c, None)
            && !group::is_contains(&eg, c)
            && group::is_exists(d, &eg, &|d, x| repfilter(d, x, a.tp), 1, c.into());
    }
    // return Duel.SelectEffectYesNo(tp,c,96)
    duel::select_effect_yes_no(d, a.tp, c, Some(96))
}
