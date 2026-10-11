//! Driangle, Dragon of the Dark Deep
use super::prelude::*;

pub const ID: u32 = 98248208;

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
    // e1:SetCondition(function(e,tp) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsLevel,10),tp,LOCATION_MZONE|LOCATION_GRAVE|LOCATION_REMOVED,0,3,nil) end)
    effect::set_condition(d, e1, Some(condition(cond1)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Make this card gain 500 ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(s.atkcost)
    effect::set_cost(d, e2, Some(cost_target(atkcost)));
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(cost_target(atktg)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,tp) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsLevel,10),tp,LOCATION_MZONE|LOCATION_GRAVE|LOCATION_REMOVED,0,3,nil) end`
fn cond1(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_level(d, c, &[10]))),
        a.tp,
        LOCATION_MZONE | LOCATION_GRAVE | LOCATION_REMOVED,
        0,
        3,
        NO_EX,
    )
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDiscardDeckAsCost(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck_as_cost(d, tp, 1);
    }
    // Duel.DiscardDeck(tp,1,REASON_COST)
    duel::discard_deck(d, tp, 1, REASON_COST);
    // local label=Duel.GetOperatedGroup():GetFirst():IsMonster() and 1 or 0
    let og = duel::get_operated_group(d);
    let tc = group::get_first(d, &og).nil();
    let label = if card::is_monster(d, tc) { 1 } else { 0 };
    // e:SetLabel(label)
    effect::set_label(d, a.e, &[label]);
    true
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    let h = effect::get_handler(d, a.e).nil();
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,e:GetHandler(),1,tp,500)
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(CardSet::from_card(h)), 1, tp, 500);
    // if e:GetLabel()==1 then
    if effect::get_label(d, a.e) == 1 {
        // e:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DAMAGE)
        effect::set_category(d, a.e, CATEGORY_ATKCHANGE + CATEGORY_DAMAGE);
        // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,1,1-tp,1000)
        duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 1, 1 - tp, 1000);
    } else {
        // e:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DECKDES)
        effect::set_category(d, a.e, CATEGORY_ATKCHANGE + CATEGORY_DECKDES);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,1)
        duel::set_possible_operation_info(d, 0, CATEGORY_DECKDES, None, 0, tp, 1);
    }
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() and c:UpdateAttack(500)>0 then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) && card::update_attack(d, c, 500, None, None, None) > 0 {
        // if e:GetLabel()==1 then
        if effect::get_label(d, a.e) == 1 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Damage(1-tp,1000,REASON_EFFECT)
            duel::damage(d, 1 - tp, 1000, REASON_EFFECT, false, None);
        // elseif Duel.IsPlayerCanDiscardDeck(tp,1) and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        } else if duel::is_player_can_discard_deck(d, tp, 1) && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.DiscardDeck(tp,1,REASON_EFFECT)
            duel::discard_deck(d, tp, 1, REASON_EFFECT);
        }
    }
}
