//! Ice Doll
//! アイス・ドール
//! Scripted by The Razgriz
use super::prelude::*;

pub const ID: u32 = 97476032;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[65569724],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand
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
    // e1:SetCost(Cost.AND(s.wateronlycost,Cost.Discard(aux.FilterBoolFunction(Card.IsAttribute,ATTRIBUTE_WATER),true)))
    effect::set_cost(
        d,
        e1,
        Some(cost::and(vec![
            cost_target(wateronlycost),
            cost::discard(
                Some(std::sync::Arc::new(|d, c, _e, _tp| {
                    card::is_attribute(d, c, ATTRIBUTE_WATER)
                })),
                true,
                None,
                None,
                None,
            ),
        ])),
    );
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 WATER Spellcaster Monster from your Deck to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(s.wateronlycost)
    effect::set_cost(d, e2, Some(cost_target(wateronlycost)));
    // e2:SetTarget(s.thtg(s.waterthfilter))
    effect::set_target(d, e2, Some(cost_target(|d, a| thtg(d, a, waterthfilter))));
    // e2:SetOperation(s.thop(s.waterthfilter))
    effect::set_operation(d, e2, Some(operation(|d, a| thop(d, a, waterthfilter))));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Add 1 "Ice Doll Mirror" from your Deck to your hand
    // local e4=e2:Clone()
    let e4 = effect::clone(d, e2);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e4, EVENT_TO_GRAVE);
    // e4:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e4, 1, ID, 2, 0);
    // e4:SetCondition(function(e) return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD) end)
    effect::set_condition(
        d,
        e4,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::is_previous_location(d, h, LOCATION_ONFIELD)
        })),
    );
    // e4:SetTarget(s.thtg(s.mirrorthfilter))
    effect::set_target(d, e4, Some(cost_target(|d, a| thtg(d, a, mirrorthfilter))));
    // e4:SetOperation(s.thop(s.mirrorthfilter))
    effect::set_operation(d, e4, Some(operation(|d, a| thop(d, a, mirrorthfilter))));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,function(c) return c:IsAttribute(ATTRIBUTE_WATER) end)
    duel::add_custom_activity_counter(
        d,
        ID,
        ACTIVITY_SPSUMMON,
        lua_fn(|d, a| card::is_attribute(d, a.c(0), ATTRIBUTE_WATER)),
    );
}
// function s.wateronlycost(e,tp,eg,ep,ev,re,r,rp,chk)
fn wateronlycost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)==0 end
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_SPSUMMON) == 0;
    }
    // Cannot Special Summon the turn you activate this effect, except WATER monsters
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e1, aux::stringid(ID, 3));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(
        d,
        e1,
        EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT,
        0,
    );
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return c:IsAttributeExcept(ATTRIBUTE_WATER) end)
    effect::set_target(
        d,
        e1,
        Some(effect_card_fn(|d, _e, c| {
            card::is_attribute_except(d, c, ATTRIBUTE_WATER, None, 0, None)
        })),
    );
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    true
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(
                d, c, a.e, 0, a.tp, false, false, None, None, None,
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(
        d,
        0,
        CATEGORY_SPECIAL_SUMMON,
        Some(CardSet::from_card(c)),
        1,
        a.tp,
        0,
    );
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(
            d,
            CardSet::from_card(c),
            0,
            a.tp,
            a.tp,
            false,
            false,
            POS_FACEUP as u8,
            None,
        );
    }
}
// function s.waterthfilter(c)
fn waterthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WATER) and c:IsRace(RACE_SPELLCASTER) and c:IsAbleToHand()
    card::is_attribute(d, c, ATTRIBUTE_WATER)
        && card::is_race(d, c, RACE_SPELLCASTER as u64)
        && card::is_able_to_hand(d, c, None)
}
// function s.mirrorthfilter(c)
fn mirrorthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(65569724) and c:IsAbleToHand()
    card::is_code(d, c, &[65569724]) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(thfilter)
// return function(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs, thfilter: fn(&mut Duel, CardId) -> bool) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&thfilter),
            a.tp,
            LOCATION_DECK,
            0,
            1,
            NO_EX,
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.thop(thfilter)
// return function(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs, thfilter: fn(&mut Duel, CardId) -> bool) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&thfilter),
        a.tp,
        LOCATION_DECK,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, &g);
    }
}
