//! A.I. Connect
use super::prelude::*;

pub const ID: u32 = 79015062;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_IGNISTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_TODECK);
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
}
// function s.revfilter(c,e,tp,mmz_chk)
fn revfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mmz_chk: bool) -> bool {
    // if not c:IsRace(RACE_CYBERSE) or c:IsPublic() then return false end
    if !card::is_race(d, c, RACE_CYBERSE) || card::is_public(d, c) {
        return false;
    }
    // if c:IsAttribute(ATTRIBUTE_DARK) then
    if card::is_attribute(d, c, ATTRIBUTE_DARK) {
        // return mmz_chk and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
        //     and Duel.IsExistingMatchingCard(s.thfilter1,tp,LOCATION_DECK,0,1,nil)
        mmz_chk
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
            && duel::is_existing_matching_card(d, Some(&thfilter1), tp, LOCATION_DECK, 0, 1, NO_EX)
    } else {
        // return c:IsAbleToDeck() and Duel.IsExistingMatchingCard(s.thfilter2,tp,LOCATION_DECK,0,1,nil,c:GetAttribute())
        if !card::is_able_to_deck(d, c) {
            return false;
        }
        let attr = card::get_attribute(d, c);
        duel::is_existing_matching_card(d, Some(&|d, x| thfilter2(d, x, attr)), tp, LOCATION_DECK, 0, 1, NO_EX)
    }
}
// function s.thfilter1(c)
fn thfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttributeExcept(ATTRIBUTE_DARK) and c:IsLevelBelow(4) and c:IsRace(RACE_CYBERSE) and c:IsAbleToHand()
    card::is_attribute_except(d, c, ATTRIBUTE_DARK, None, 0, None)
        && card::is_level_below(d, c, 4)
        && card::is_race(d, c, RACE_CYBERSE)
        && card::is_able_to_hand(d, c, None)
}
// function s.thfilter2(c,attr)
fn thfilter2(d: &mut Duel, c: CardId, attr: u32) -> bool {
    // return c:IsSetCard(SET_IGNISTER) and c:IsMonster() and c:IsAttributeExcept(attr) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_IGNISTER as u16])
        && card::is_monster(d, c)
        && card::is_attribute_except(d, c, attr, None, 0, None)
        && card::is_able_to_hand(d, c, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local mmz_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let mmz_chk = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.revfilter,tp,LOCATION_HAND,0,1,nil,e,tp,mmz_chk) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| revfilter(d, x, a.e, a.tp, mmz_chk)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
    // local rc=Duel.SelectMatchingCard(tp,s.revfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp,mmz_chk):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&|d, x| revfilter(d, x, a.e, a.tp, mmz_chk)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let rc = group::get_first(d, &sel).nil();
    // Duel.ConfirmCards(1-tp,rc)
    duel::confirm_cards(d, 1 - a.tp, CardSet::from_card(rc));
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, a.tp);
    // e:SetLabelObject(rc)
    effect::set_label_object(d, a.e, Value::Card(rc));
    // Duel.SetTargetCard(rc)
    duel::set_target_card(d, CardSet::from_card(rc));
    // local category=rc:IsAttribute(ATTRIBUTE_DARK) and CATEGORY_SPECIAL_SUMMON or CATEGORY_TODECK
    let category = if card::is_attribute(d, rc, ATTRIBUTE_DARK) { CATEGORY_SPECIAL_SUMMON } else { CATEGORY_TODECK };
    // Duel.SetOperationInfo(0,category,rc,1,tp,0)
    duel::set_operation_info(d, 0, category, Some(CardSet::from_card(rc)), 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local rc=e:GetLabelObject()
    let rc = effect::get_label_object(d, a.e).as_card().nil();
    // if rc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, rc, a.e) {
        // local success=nil
        // local thfilter=nil
        let success: bool;
        let is_dark: bool;
        // if rc:IsAttribute(ATTRIBUTE_DARK) then
        if card::is_attribute(d, rc, ATTRIBUTE_DARK) {
            // Special Summon the revealed monster, and if you do, add 1 non-DARK Level 4 or lower Cyberse monster from your Deck to your hand
            // success=Duel.SpecialSummon(rc,0,tp,tp,false,false,POS_FACEUP)>0
            success = duel::special_summon(d, CardSet::from_card(rc), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0;
            // thfilter=s.thfilter1
            is_dark = true;
        } else {
            // Shuffle the revealed monster into the Deck, and if you do, add 1 "@Ignister" monster with a different Attribute from your Deck to your hand
            // Duel.ConfirmCards(1-tp,rc)
            duel::confirm_cards(d, 1 - a.tp, CardSet::from_card(rc));
            // success=Duel.SendtoDeck(rc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0
            success = duel::send_to_deck(d, CardSet::from_card(rc), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0;
            // thfilter=s.thfilter2
            is_dark = false;
        }
        // if success then
        if success {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
            // local g=Duel.SelectMatchingCard(tp,thfilter,tp,LOCATION_DECK,0,1,1,nil,rc:GetAttribute())
            let g = if is_dark {
                duel::select_matching_card(d, a.tp, Some(&thfilter1), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil()
            } else {
                let attr = card::get_attribute(d, rc);
                duel::select_matching_card(d, a.tp, Some(&|d, x| thfilter2(d, x, attr)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil()
            };
            // if #g>0 then
            if !g.is_empty() {
                // Duel.SendtoHand(g,nil,REASON_EFFECT)
                duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
                // Duel.ConfirmCards(1-tp,g)
                duel::confirm_cards(d, 1 - a.tp, &g);
            }
        }
    }
    // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // You cannot Special Summon for the rest of this turn after this card resolves, except Cyberse monsters
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return not c:IsRace(RACE_CYBERSE) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_race(d, c, RACE_CYBERSE))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
