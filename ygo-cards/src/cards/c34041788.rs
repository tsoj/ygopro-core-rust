//! Endymion Empire
use super::prelude::*;

pub const ID: u32 = 34041788;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_REGULUS_THE_PRINCE_OF_ENDYMION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When this card is activated: Add 1 "Regulus, the Prince of Endymion" or 1 monster that mentions it from your Deck to your hand, ...
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,{id,0},EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If a card(s) you control would be destroyed by battle or card effect, you can destroy 1 "Regulus, the Prince of Endymion" in your hand or face-up Monster Zone instead
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.desreptg)
    effect::set_target(d, e2, Some(cost_target(desreptg)));
    // e2:SetOperation(s.desrepop)
    effect::set_operation(d, e2, Some(operation(desrepop)));
    // e2:SetValue(function(e,c) return s.repfilter(c,e:GetHandlerPlayer()) end)
    effect::set_value_fn(d, e2, effect_card_fn(|d, e, cc| {
        let p = effect::get_handler_player(d, e);
        repfilter(d, cc, p)
    }));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsCode(CARD_REGULUS_THE_PRINCE_OF_ENDYMION) or (c:IsMonster() and c:ListsCode(CARD_REGULUS_THE_PRINCE_OF_ENDYMION)))
    //     and c:IsAbleToHand()
    (card::is_code(d, c, &[CARD_REGULUS_THE_PRINCE_OF_ENDYMION]) || (card::is_monster(d, c) && card::lists_code(d, c, &[CARD_REGULUS_THE_PRINCE_OF_ENDYMION])))
        && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_SPELLCASTER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_SPELLCASTER as u64) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SendtoHand(g,nil,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // if Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) > 0
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, cc| spfilter(d, cc, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local sg=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
            let sg = duel::select_matching_card(d, tp, Some(&|d, cc| spfilter(d, cc, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
            // if #sg>0 then
            if !sg.is_empty() {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsOnField() and c:IsReason(REASON_BATTLE|REASON_EFFECT)
    //     and not c:IsReason(REASON_REPLACE)
    card::is_controler(d, c, tp) && card::is_on_field(d, c) && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT) && !card::is_reason(d, c, REASON_REPLACE)
}
/// `s.desfilter(c,e)`
fn desfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsCode(CARD_REGULUS_THE_PRINCE_OF_ENDYMION) and (c:IsLocation(LOCATION_HAND) or c:IsFaceup())
    //     and c:IsDestructable(e) and not c:IsStatus(STATUS_DESTROY_CONFIRMED|STATUS_BATTLE_DESTROYED)
    card::is_code(d, c, &[CARD_REGULUS_THE_PRINCE_OF_ENDYMION])
        && (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
        && card::is_destructable(d, c, Some(e))
        && !card::is_status(d, c, STATUS_DESTROY_CONFIRMED | STATUS_BATTLE_DESTROYED)
}
/// `s.desreptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then return eg:IsExists(s.repfilter,1,nil,tp)
    //     and Duel.IsExistingMatchingCard(s.desfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil,e) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return group::is_exists(d, &eg, &|d, cc| repfilter(d, cc, tp), 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, cc| desfilter(d, cc, e)), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    // if Duel.SelectEffectYesNo(tp,e:GetHandler(),96) then
    let h = effect::get_handler(d, e).nil();
    if duel::select_effect_yes_no(d, tp, h, Some(96)) {
        // Duel.Hint(HINT_CARD,0,id)
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESREPLACE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESREPLACE as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.desfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil,e):GetFirst()
        let sg = duel::select_matching_card(d, tp, Some(&|d, cc| desfilter(d, cc, e)), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sg);
        // e:SetLabelObject(sc)
        effect::set_label_object(d, e, Value::card(sc));
        let sc = sc.nil();
        // sc:SetStatus(STATUS_DESTROY_CONFIRMED,true)
        card::set_status(d, sc, STATUS_DESTROY_CONFIRMED, true);
        // if sc:IsLocation(LOCATION_HAND) then
        if card::is_location(d, sc, LOCATION_HAND) {
            // Duel.ConfirmCards(1-tp,sc)
            duel::confirm_cards(d, 1 - tp, CardSet::from_card(sc));
        } else {
            // Duel.HintSelection(sc)
            duel::hint_selection(d, CardSet::from_card(sc), true);
        }
        // return true
        true
    } else {
        // return false
        false
    }
}
/// `s.desrepop(e,tp,...)`
fn desrepop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local sc=e:GetLabelObject()
    let sc = effect::get_label_object(d, e).as_card().nil();
    // sc:SetStatus(STATUS_DESTROY_CONFIRMED,false)
    card::set_status(d, sc, STATUS_DESTROY_CONFIRMED, false);
    // Duel.Destroy(sc,REASON_EFFECT|REASON_REPLACE)
    duel::destroy(d, CardSet::from_card(sc), REASON_EFFECT | REASON_REPLACE, None, None);
}
