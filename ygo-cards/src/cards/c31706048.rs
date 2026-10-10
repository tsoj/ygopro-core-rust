//! World Legacy Survivor
use super::prelude::*;

pub const ID: u32 = 31706048;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_KRAWLER, SET_WORLD_LEGACY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_DECKDES)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_DECKDES);
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
/// `s.target(e,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)<5 then return false end
        if duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) < 5 {
            return false;
        }
        // local g=Duel.GetDecktopGroup(tp,5)
        let g = duel::get_decktop_group(d, a.tp, 5);
        // local result=g:FilterCount(Card.IsAbleToHand,nil)>0
        let result = group::filter_count(d, &g, &|d, c| card::is_able_to_hand(d, c, None), NO_EX) > 0;
        // return result and Duel.IsPlayerCanDiscardDeck(tp,5)
        return result && duel::is_player_can_discard_deck(d, a.tp, 5);
    }
    true
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsMonster() and c:IsSetCard(SET_KRAWLER)) or c:IsSetCard(SET_WORLD_LEGACY)
    (card::is_type(d, c, TYPE_MONSTER) && card::is_set_card(d, c, &[SET_KRAWLER as u16])) || card::is_set_card(d, c, &[SET_WORLD_LEGACY as u16])
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if not Duel.IsPlayerCanDiscardDeck(tp,5) then return end
    if !duel::is_player_can_discard_deck(d, a.tp, 5) {
        return;
    }
    // Duel.ConfirmDecktop(tp,5)
    duel::confirm_decktop(d, a.tp, 5);
    // local g=Duel.GetDecktopGroup(tp,5)
    let mut g = duel::get_decktop_group(d, a.tp, 5);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // if g:IsExists(s.filter,1,nil) then
        if group::is_exists(d, &g, &filter, 1, NO_EX) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
            // local sg=g:FilterSelect(tp,s.filter,1,1,nil)
            let sg = group::filter_select(d, &g, a.tp, &filter, 1, 1, false, NO_EX).nil();
            // if sg:GetFirst():IsAbleToHand() then
            let first = group::get_first(d, &sg).nil();
            if card::is_able_to_hand(d, first, None) {
                // Duel.SendtoHand(sg,nil,REASON_EFFECT)
                duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
                // Duel.ConfirmCards(1-tp,sg)
                duel::confirm_cards(d, 1 - a.tp, sg.clone());
                // Duel.ShuffleHand(tp)
                duel::shuffle_hand(d, a.tp);
                // g:Sub(sg)
                group::remove_card(&mut g, &sg);
                // Duel.SendtoGrave(g,REASON_EFFECT|REASON_EXCAVATE)
                duel::send_to_grave(d, g.clone(), REASON_EFFECT | REASON_EXCAVATE, None, None);
            } else {
                // Duel.SendtoGrave(g,REASON_EFFECT|REASON_EXCAVATE)
                duel::send_to_grave(d, g.clone(), REASON_EFFECT | REASON_EXCAVATE, None, None);
            }
        } else {
            // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT|REASON_EXCAVATE)
            duel::send_to_deck(d, g.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT | REASON_EXCAVATE, None);
            // Duel.ShuffleDeck(tp)
            duel::shuffle_deck(d, a.tp);
        }
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(effect_card_fn(splimit)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // aux.addTempLizardCheck(e:GetHandler(),tp,s.lizfilter)
    let h = effect::get_handler(d, a.e).nil();
    aux::add_temp_lizard_check(d, h, a.tp, Some(effect_card_fn(lizfilter)), None, None, None, None);
}
/// `s.splimit(e,c)`
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsLinkMonster() and c:IsLocation(LOCATION_EXTRA)
    !card::is_link_monster(d, c) && card::is_location(d, c, LOCATION_EXTRA)
}
/// `s.lizfilter(e,c)`
fn lizfilter(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsOriginalType(TYPE_LINK)
    !card::is_original_type(d, c, TYPE_LINK)
}
