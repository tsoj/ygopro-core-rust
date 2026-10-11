//! Gladiator Beast Gistel
use super::prelude::*;

pub const ID: u32 = 26582143;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_GLADIATOR_BEAST, SET_GLADIATOR],
    listed_names: &[ID],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Gladiator Beast" monster from your Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_PHASE|PHASE_BATTLE)
    effect::set_code(d, e1, EVENT_PHASE | PHASE_BATTLE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(function(e) return e:GetHandler():GetBattledGroupCount()>0 end)
    effect::set_condition(d, e1, Some(condition(e1_cond)));
    // e1:SetCost(Cost.SelfToDeck)
    effect::set_cost(d, e1, Some(cost::self_to_deck()));
    // e1:SetTarget(s.decksptg)
    effect::set_target(d, e1, Some(cost_target(decksptg)));
    // e1:SetOperation(s.deckspop)
    effect::set_operation(d, e1, Some(operation(deckspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon both this card and 1 other "Gladiator Beast" monster from your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(s.handspcost)
    effect::set_cost(d, e2, Some(cost_target(handspcost)));
    // e2:SetTarget(s.handsptg)
    effect::set_target(d, e2, Some(cost_target(handsptg)));
    // e2:SetOperation(s.handspop)
    effect::set_operation(d, e2, Some(operation(handspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Add 1 "Gladiator" Spell/Trap from your Deck to your hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(aux.gbspcon)
    effect::set_condition(d, e3, Some(aux::gbspcon()));
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.deckspfilter(c,e,tp)
fn deckspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GLADIATOR_BEAST) and c:IsCanBeSpecialSummoned(e,100,tp,false,false) and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_GLADIATOR_BEAST as u16])
        && card::is_can_be_special_summoned(d, c, e, 100, tp, false, false, None, None, None)
        && !card::is_code(d, c, &[ID])
}
// function s.decksptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn decksptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetMZoneCount(tp,e:GetHandler())>0
    //     and Duel.IsExistingMatchingCard(s.deckspfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::get_m_zone_count(d, a.tp, h.into(), None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| deckspfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.deckspop(e,tp,eg,ep,ev,re,r,rp)
fn deckspop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.deckspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| deckspfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc and Duel.SpecialSummon(sc,100,tp,tp,false,false,POS_FACEUP)>0 then
    if let Some(sc) = sc {
        if duel::special_summon(d, CardSet::from_card(sc), 100, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
            // sc:RegisterFlagEffect(sc:GetOriginalCode(),RESET_EVENT|RESETS_STANDARD_DISABLE,0,1)
            let code = card::get_original_code(d, sc);
            card::register_flag_effect(d, sc, code, RESET_EVENT | RESETS_STANDARD_DISABLE, 0, 1, 0, 0);
        }
    }
}
// function s.handspcostfilter(c,e,tp)
fn handspcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GLADIATOR_BEAST) and not c:IsPublic() and c:IsCanBeSpecialSummoned(e,100,tp,false,false)
    card::is_set_card(d, c, &[SET_GLADIATOR_BEAST as u16])
        && !card::is_public(d, c)
        && card::is_can_be_special_summoned(d, c, e, 100, tp, false, false, None, None, None)
}
// function s.handspcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn handspcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:IsPublic() and Duel.IsExistingMatchingCard(s.handspcostfilter,tp,LOCATION_HAND,0,1,c,e,tp) end
    if a.chk == Some(0) {
        return !card::is_public(d, c)
            && duel::is_existing_matching_card(d, Some(&|d, x| handspcostfilter(d, x, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
    // local rc=Duel.SelectMatchingCard(tp,s.handspcostfilter,tp,LOCATION_HAND,0,1,1,c,e,tp):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| handspcostfilter(d, x, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, 1, false, c.into()).nil();
    let rc = group::get_first(d, &g);
    // Duel.ConfirmCards(1-tp,Group.FromCards(c,rc))
    let mut shown = CardSet::from_card(c);
    if let Some(rc) = rc {
        group::add_card(&mut shown, &CardSet::from_card(rc));
    }
    duel::confirm_cards(d, 1 - a.tp, shown);
    // e:SetLabelObject(rc)
    effect::set_label_object(d, a.e, rc.map_or(Value::Nil, Value::Card));
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, a.tp);
    true
}
// function s.handsptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn handsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>=2
    //     and c:IsCanBeSpecialSummoned(e,100,tp,false,false) end
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 >= 2
            && card::is_can_be_special_summoned(d, c, a.e, 100, a.tp, false, false, None, None, None);
    }
    // local rc=e:GetLabelObject()
    let rc = effect::get_label_object(d, a.e).as_card();
    // local g=Group.FromCards(c,rc)
    let mut g = CardSet::from_card(c);
    if let Some(rc) = rc {
        group::add_card(&mut g, &CardSet::from_card(rc));
    }
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 2, a.tp, 0);
    true
}
// function s.handspop(e,tp,eg,ep,ev,re,r,rp)
fn handspop(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) or Duel.GetLocationCount(tp,LOCATION_MZONE)<2 then return end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some()
        || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 2
    {
        return;
    }
    // local sg=Duel.GetTargetCards(e)
    let sg = duel::get_target_cards(d, a.e).nil();
    // if #sg==2 and sg:FilterCount(Card.IsCanBeSpecialSummoned,nil,e,100,tp,false,false)==2
    //     and Duel.SpecialSummon(sg,100,tp,tp,false,false,POS_FACEUP)==2 then
    let e = a.e;
    let tp = a.tp;
    if sg.len() == 2
        && group::filter_count(d, &sg, &|d, x| card::is_can_be_special_summoned(d, x, e, 100, tp, false, false, None, None, None), NO_EX) == 2
        && duel::special_summon(d, sg.clone(), 100, tp, tp, false, false, POS_FACEUP as u8, None) == 2
    {
        // for sc in sg:Iter() do
        for sc in group::iter(d, &sg) {
            // sc:RegisterFlagEffect(sc:GetOriginalCode(),RESET_EVENT|RESETS_STANDARD_DISABLE,0,1)
            let code = card::get_original_code(d, sc);
            card::register_flag_effect(d, sc, code, RESET_EVENT | RESETS_STANDARD_DISABLE, 0, 1, 0, 0);
        }
    }
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GLADIATOR) and c:IsSpellTrap() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_GLADIATOR as u16]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
// e1 condition: function(e) return e:GetHandler():GetBattledGroupCount()>0 end
fn e1_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::get_battled_group_count(d, c) > 0
}
