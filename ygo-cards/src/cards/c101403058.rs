//! Reindsurm Practice
//! Scripted by Naim
use super::prelude::*;

pub const ID: u32 = 101403058;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_REINDSURM], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 2 "Reindsurm" monsters from your Deck to your hand, then send 1 card from your hand to the GY, then if your opponent has a Dragon monster in their field or GY, you can add 1 Insect monster from your GY to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If you Ritual Summon an Insect monster(s) while this card is in your GY: You can banish this card; place 1 "Reindsurm" Field Spell from your Deck face-up on your field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.plcon)
    effect::set_condition(d, e2, Some(condition(plcon)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.pltg)
    effect::set_target(d, e2, Some(cost_target(pltg)));
    // e2:SetOperation(s.plop)
    effect::set_operation(d, e2, Some(operation(plop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_REINDSURM) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_REINDSURM as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,2,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 2, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 2, a.tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_HAND as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.insectfilter(c)`
fn insectfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_INSECT) and c:IsAbleToHand()
    card::is_race(d, c, RACE_INSECT) && card::is_able_to_hand(d, c, None)
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,2,2,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 2, 2, false, NO_EX).nil();
    // if #g==2 and Duel.SendtoHand(g,nil,REASON_EFFECT)>0 then
    if g.len() == 2 && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g.clone());
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, a.tp);
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, a.tp);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
        // local tgc=Duel.SelectMatchingCard(tp,Card.IsAbleToGrave,tp,LOCATION_HAND,0,1,1,nil):GetFirst()
        let tg = duel::select_matching_card(d, a.tp, Some(&|d, c| card::is_able_to_grave(d, c, None)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        let tgc = group::get_first(d, &tg);
        // if tgc then
        if let Some(tgc) = tgc {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // if Duel.SendtoGrave(tgc,REASON_EFFECT)>0 and tgc:IsLocation(LOCATION_GRAVE)
            //   and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_DRAGON),tp,0,LOCATION_MZONE|LOCATION_GRAVE,1,nil)
            //   and Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.insectfilter),tp,LOCATION_GRAVE,0,1,nil)
            //   and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
            if duel::send_to_grave(d, tgc, REASON_EFFECT, None, None) > 0
                && card::is_location(d, tgc, LOCATION_GRAVE)
                && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_DRAGON)), a.tp, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX)
                && duel::is_existing_matching_card(d, Some(&aux::necro_valley_filter(|d, c| insectfilter(d, c))), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
                && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
            {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
                // local thc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.insectfilter),tp,LOCATION_GRAVE,0,1,1,nil):GetFirst()
                let thg = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(|d, c| insectfilter(d, c))), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
                // if thc then
                if let Some(thc) = group::get_first(d, &thg) {
                    // Duel.HintSelection(thc)
                    duel::hint_selection(d, thc, true);
                    // Duel.BreakEffect()
                    duel::break_effect(d);
                    // Duel.SendtoHand(thc,nil,REASON_EFFECT)
                    duel::send_to_hand(d, thc, None, REASON_EFFECT, None);
                }
            }
        }
    }
}
/// `s.plconfilter(c,tp)`
fn plconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRitualSummoned() and c:IsRace(RACE_INSECT) and c:IsSummonPlayer(tp) and c:IsFaceup()
    card::is_ritual_summoned(d, c) && card::is_race(d, c, RACE_INSECT) && card::is_summon_player(d, c, tp) && card::is_faceup(d, c)
}
/// `s.plcon(e,tp,eg,ep,ev,re,r,rp)`
fn plcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.plconfilter,1,nil,tp)
    group::is_exists(d, &a.eg.clone().nil(), &|d, c| plconfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.plfilter(c,tp)`
fn plfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_REINDSURM) and c:IsFieldSpell() and not c:IsForbidden() and c:CheckUniqueOnField(tp)
    card::is_set_card(d, c, &[SET_REINDSURM as u16]) && card::is_field_spell(d, c) && !card::is_forbidden(d, c) && card::check_unique_on_field(d, c, tp, None, None)
}
/// `s.pltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_DECK,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| plfilter(d, c, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.plop(e,tp,eg,ep,ev,re,r,rp)`
fn plop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.plfilter,tp,LOCATION_DECK,0,1,1,nil,tp):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| plfilter(d, c, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if sc then
    if let Some(sc) = group::get_first(d, &g) {
        // Duel.MoveToField(sc,tp,tp,LOCATION_FZONE,POS_FACEUP,true)
        duel::move_to_field(d, sc, a.tp, a.tp, LOCATION_FZONE as u16, POS_FACEUP as u8, true, None);
    }
}
