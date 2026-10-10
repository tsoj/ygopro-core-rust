//! Albion the Shrouded Dragon
use super::prelude::*;

pub const ID: u32 = 25451383;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ALBAZ], listed_series: &[SET_BRANDED], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Change name
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    // e1:SetRange(LOCATION_MZONE|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_MZONE | LOCATION_GRAVE);
    // e1:SetValue(CARD_ALBAZ)
    effect::set_value(d, e1, CARD_ALBAZ as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Apply the appropriate effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_HAND | LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(s.spcost)
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.cfilter(c,e,tp)
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if not ((c:IsCode(CARD_ALBAZ) or (c:IsSetCard(SET_BRANDED) and c:IsSpellTrap())) and c:IsAbleToGraveAsCost()) then return false end
    if !((card::is_code(d, c, &[CARD_ALBAZ]) || (card::is_set_card(d, c, &[SET_BRANDED as u16]) && card::is_spell_trap(d, c)))
        && card::is_able_to_grave_as_cost(d, c))
    {
        return false;
    }
    // local hc=e:GetHandler()
    let hc = effect::get_handler(d, e).nil();
    // if c:IsLocation(LOCATION_HAND) then
    if card::is_location(d, c, LOCATION_HAND) {
        // return Duel.GetLocationCount(tp,LOCATION_MZONE,0)>0 and hc:IsCanBeSpecialSummoned(e,0,tp,false,false)
        duel::get_location_count(d, tp, LOCATION_MZONE as u8, Some(0), None, None).0 > 0
            && card::is_can_be_special_summoned(d, hc, e, 0, tp, false, false, None, None, None)
    } else {
        // return hc:IsAbleToDeck() and (hc:IsLocation(LOCATION_GRAVE) or Duel.IsPlayerCanDraw(tp,1))
        card::is_able_to_deck(d, hc) && (card::is_location(d, hc, LOCATION_GRAVE) || duel::is_player_can_draw(d, tp, 1))
    }
}
// function s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&|d, c| cfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel).nil();
    // local label=tc:IsLocation(LOCATION_DECK) and 1 or 0
    let label = if card::is_location(d, tc, LOCATION_DECK) { 1 } else { 0 };
    // e:SetLabel(label)
    effect::set_label(d, a.e, &[label]);
    // Duel.SendtoGrave(tc,REASON_COST)
    duel::send_to_grave(d, CardSet::from_card(tc), REASON_COST, None, None);
    true
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local label=e:GetLabel()
    let label = effect::get_label(d, a.e);
    if label == 0 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    } else {
        // local cat=CATEGORY_TODECK
        let mut cat = CATEGORY_TODECK;
        // if c:IsLocation(LOCATION_HAND) then
        if card::is_location(d, c, LOCATION_HAND) {
            // cat=cat+CATEGORY_DRAW
            cat += CATEGORY_DRAW;
            // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
            duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
        }
        // e:SetCategory(cat)
        effect::set_category(d, a.e, cat);
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,c,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(c)), 1, a.tp, 0);
    }
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local label=e:GetLabel()
    let label = effect::get_label(d, a.e);
    // if label==0 and Duel.GetLocationCount(tp,LOCATION_MZONE,0)>0 then
    if label == 0 && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, Some(0), None, None).0 > 0 {
        // Hand: Special Summon this card
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    } else {
        // Deck: Place this card on the bottom of the Deck
        // local loc=c:GetLocation()
        let loc = card::get_location(d, c);
        // if Duel.SendtoDeck(c,nil,SEQ_DECKBOTTOM,REASON_EFFECT)>0 and c:IsLocation(LOCATION_DECK) and loc==LOCATION_HAND then
        if duel::send_to_deck(d, c, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None) > 0
            && card::is_location(d, c, LOCATION_DECK)
            && loc == LOCATION_HAND
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Draw(tp,1,REASON_EFFECT)
            duel::draw(d, a.tp, 1, REASON_EFFECT);
        }
    }
}
