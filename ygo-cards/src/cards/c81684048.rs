//! Pumpking the King of Grave Ghosts
use super::prelude::*;

pub const ID: u32 = 81684048;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_CALL_OF_THE_HAUNTED, ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can reveal this card in your hand; Set 1 "Call of the Haunted" from your Deck or GY, and if you do, discard 1 card. That Set card can be activated this turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SET+CATEGORY_TOGRAVE+CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_SET | CATEGORY_TOGRAVE | CATEGORY_HANDES);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.SelfReveal)
    effect::set_cost(d, e1, Some(cost::self_reveal()));
    // e1:SetTarget(s.settg)
    effect::set_target(d, e1, Some(cost_target(settg)));
    // e1:SetOperation(s.setop)
    effect::set_operation(d, e1, Some(operation(setop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card is Special Summoned: You can Special Summon 1 Level 6 Zombie monster from your Deck, except with 1950 ATK, also you cannot Special Summon from the hand or GY for the rest of this turn, except Zombie monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_CALL_OF_THE_HAUNTED) and c:IsSSetable()
    card::is_code(d, c, &[CARD_CALL_OF_THE_HAUNTED]) && card::is_s_setable(d, c, false, None)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil,REASON_EFFECT) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(
                d,
                Some(&|d, c| card::is_discardable(d, c, Some(REASON_EFFECT), None)),
                a.tp,
                LOCATION_HAND,
                0,
                1,
                NO_EX,
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SET,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SET, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
    // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.setfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::necro_valley_filter(|d, c| setfilter(d, c))),
        a.tp,
        LOCATION_DECK | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let sc = group::get_first(d, &g);
    // if sc and Duel.SSet(tp,sc)>0 then
    if let Some(sc) = sc {
        if duel::s_set(d, a.tp, sc, None, true) > 0 {
            // --That Set card can be activated this turn
            // local e1=Effect.CreateEffect(e:GetHandler())
            let c = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, c);
            // e1:SetDescription(aux.Stringid(id,2))
            effect::set_description(d, e1, aux::stringid(ID, 2));
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE | EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(EFFECT_TRAP_ACT_IN_SET_TURN)
            effect::set_code(d, e1, EFFECT_TRAP_ACT_IN_SET_TURN);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // sc:RegisterEffect(e1)
            card::register_effect(d, sc, e1, false);
            // Duel.DiscardHand(tp,nil,1,1,REASON_EFFECT,nil)
            duel::discard_hand(d, a.tp, None, 1, 1, REASON_EFFECT, NO_EX);
        }
    }
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevel(6) and c:IsRace(RACE_ZOMBIE) and not c:IsAttack(1950) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level(d, c, &[6])
        && card::is_race(d, c, RACE_ZOMBIE as u64)
        && !card::is_attack(d, c, &[1950])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
    // --You cannot Special Summon from the hand or GY for the rest of this turn, except Zombie monsters
    // local e1=Effect.CreateEffect(e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e1, aux::stringid(ID, 3));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_HAND|LOCATION_GRAVE) and c:IsRaceExcept(RACE_ZOMBIE) end)
    effect::set_target(
        d,
        e1,
        Some(effect_card_fn(|d, _e, c| {
            card::is_location(d, c, LOCATION_HAND | LOCATION_GRAVE) && card::is_race_except(d, c, RACE_ZOMBIE as u64, None, 0, None)
        })),
    );
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
