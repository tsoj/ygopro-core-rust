//! Silk Bomb Moth
use super::prelude::*;

pub const ID: u32 = 18300894;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card, and if you do, shuffle 1 WIND monster from your GY into the Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp) return Duel.IsExistingMatchingCard(Card.IsAttribute,tp,LOCATION_GRAVE,0,2,nil,ATTRIBUTE_WIND) end)
    effect::set_condition(d, e1, Some(condition(cond1)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Reveal the top card of your opponent's Deck, and apply the appropriate effect based on its card type
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    // e2a:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2a, CATEGORY_REMOVE);
    // e2a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2a:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2a, EFFECT_FLAG_DELAY, 0);
    // e2a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2a, EVENT_SUMMON_SUCCESS);
    // e2a:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2a, 1, ID, 1, 0);
    // e2a:SetTarget(s.rvtg)
    effect::set_target(d, e2a, Some(cost_target(rvtg)));
    // e2a:SetOperation(s.rvop)
    effect::set_operation(d, e2a, Some(operation(rvop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WIND) and c:IsAbleToDeck()
    card::is_attribute(d, c, ATTRIBUTE_WIND) && card::is_able_to_deck(d, c)
}
/// `function(e,tp) return Duel.IsExistingMatchingCard(Card.IsAttribute,tp,LOCATION_GRAVE,0,2,nil,ATTRIBUTE_WIND) end`
fn cond1(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_attribute(d, c, ATTRIBUTE_WIND)), a.tp, LOCATION_GRAVE, 0, 2, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingMatchingCard(s.tdfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)
            && duel::is_existing_matching_card(d, Some(&tdfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
        // local g=Duel.SelectMatchingCard(tp,s.tdfilter,tp,LOCATION_GRAVE,0,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&tdfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_COST)
            duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_COST, None);
        }
    }
}
/// `s.rvtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_DECK) > 0;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - a.tp, LOCATION_DECK as i32);
    true
}
/// `s.rvop(e,tp,eg,ep,ev,re,r,rp)`
fn rvop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)==0 then return end
    if duel::get_field_group_count(d, a.tp, 0, LOCATION_DECK) == 0 {
        return;
    }
    // Duel.DisableShuffleCheck()
    duel::disable_shuffle_check(d, true);
    // Duel.ConfirmDecktop(1-tp,1)
    duel::confirm_decktop(d, 1 - a.tp, 1);
    // local rvc=Duel.GetDecktopGroup(1-tp,1):GetFirst()
    let dt = duel::get_decktop_group(d, 1 - a.tp, 1);
    let rvc = group::get_first(d, &dt).nil();
    // if rvc:IsMonster() then
    if card::is_monster(d, rvc) {
        // Place it in your opponent's Spell & Trap Zone as a face-up Continuous Spell
        // if Duel.MoveToField(rvc,tp,1-tp,LOCATION_SZONE,POS_FACEUP,true) then
        if duel::move_to_field(d, rvc, a.tp, 1 - a.tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_CHANGE_TYPE)
            effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
            // e1:SetValue(TYPE_SPELL|TYPE_CONTINUOUS)
            effect::set_value(d, e1, (TYPE_SPELL | TYPE_CONTINUOUS) as i64);
            // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TURN_SET))
            effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
            // rvc:RegisterEffect(e1)
            card::register_effect(d, rvc, e1, false);
        }
    } else {
        // Banish it
        // Duel.Remove(rvc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, rvc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
