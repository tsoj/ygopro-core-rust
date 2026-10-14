//! GMX Suppression Squad
use super::prelude::*;

pub const ID: u32 = 39706423;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_GMX],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If this card is in your hand and you control a "GMX" monster or a Dinosaur monster: You can Special Summon this card
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
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You can target 1 face-up monster on the field; excavate the top cards of your Deck until you excavate a Dinosaur
    // monster, send that Dinosaur monster to the GY, and if you do, the targeted monster becomes a Dinosaur monster
    // (until the end of this turn), also shuffle the rest into the Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.excavtg)
    effect::set_target(d, e2, Some(cost_target(excavtg)));
    // e2:SetOperation(s.excavop)
    effect::set_operation(d, e2, Some(operation(excavop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spconfilter(c)`
fn spconfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsSetCard(SET_GMX) or c:IsRace(RACE_DINOSAUR)) and c:IsFaceup()
    (card::is_set_card(d, c, &[SET_GMX as u16]) || card::is_race(d, c, RACE_DINOSAUR as u64)) && card::is_faceup(d, c)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.spconfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&spconfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DINOSAUR) and c:IsAbleToGrave()
    card::is_race(d, c, RACE_DINOSAUR as u64) && card::is_able_to_grave(d, c, None)
}
/// `aux.FaceupFilter(aux.NOT(Card.IsRace),RACE_DINOSAUR)`
fn not_dino_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && !card::is_race(d, c, RACE_DINOSAUR as u64)
}
/// `s.excavtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn excavtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and not chkc:IsRace(RACE_DINOSAUR) and chkc:IsFaceup() end
    // (ygoai override lua_fixes/c39706423.lua: the original indexes the undefined global c, a Lua error;
    // the fix uses chkc)
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && !card::is_race(d, chkc, RACE_DINOSAUR as u64) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(aux.NOT(Card.IsRace),RACE_DINOSAUR),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    //     and Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&not_dino_faceup), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,aux.FaceupFilter(aux.NOT(Card.IsRace),RACE_DINOSAUR),tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, tp, Some(&not_dino_faceup), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `g:GetMaxGroup(Card.GetSequence):GetFirst()`
fn max_seq_card(d: &mut Duel, g: &CardSet) -> CardId {
    let max = group::get_max_group(d, g, &|d, c| card::get_sequence(d, c) as i64).nil().0;
    group::get_first(d, &max).nil()
}
/// `s.excavop(e,tp,eg,ep,ev,re,r,rp)`
fn excavop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local deck_count=Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)
    let deck_count = duel::get_field_group_count(d, tp, LOCATION_DECK, 0);
    // if deck_count>0 then
    if deck_count > 0 {
        // local g=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&tgfilter), tp, LOCATION_DECK, 0, NO_EX);
        // if #g==0 then
        if g.is_empty() {
            // Duel.ConfirmDecktop(tp,deck_count)
            duel::confirm_decktop(d, tp, deck_count as u32);
            // local excav_g=Duel.GetDecktopGroup(tp,deck_count)
            let excav_g = duel::get_decktop_group(d, tp, deck_count as u32);
            // Duel.RaiseEvent(excav_g,EVENT_CUSTOM+1595137,e,REASON_EFFECT,tp,tp,deck_count)
            duel::raise_event(d, excav_g, EVENT_CUSTOM + 1595137, Some(a.e), REASON_EFFECT, tp, tp, deck_count as u32);
        } else {
            // local sc=g:GetMaxGroup(Card.GetSequence):GetFirst()
            let sc = max_seq_card(d, &g);
            // local sc_seq=sc:GetSequence()
            let sc_seq = card::get_sequence(d, sc) as i32;
            // local excav_count=deck_count-sc_seq
            let excav_count = deck_count - sc_seq;
            // Duel.ConfirmDecktop(tp,excav_count)
            duel::confirm_decktop(d, tp, excav_count as u32);
            // local excav_g=Duel.GetDecktopGroup(tp,excav_count)
            let excav_g = duel::get_decktop_group(d, tp, excav_count as u32);
            // Duel.RaiseEvent(excav_g,EVENT_CUSTOM+1595137,e,REASON_EFFECT,tp,tp,excav_count)
            duel::raise_event(d, excav_g, EVENT_CUSTOM + 1595137, Some(a.e), REASON_EFFECT, tp, tp, excav_count as u32);
            // local tc=Duel.GetFirstTarget()
            let tc = duel::first_target(d);
            // if Duel.SendtoGrave(sc,REASON_EFFECT|REASON_EXCAVATE) and sc:IsLocation(LOCATION_GRAVE) and tc:IsRelateToEffect(e)
            //     and tc:IsFaceup() and not tc:IsRace(RACE_DINOSAUR) then
            // (a Lua number is always true, so only the checks after it decide)
            duel::send_to_grave(d, sc, REASON_EFFECT | REASON_EXCAVATE, None, None);
            if card::is_location(d, sc, LOCATION_GRAVE) {
                let tc = tc.nil();
                if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && !card::is_race(d, tc, RACE_DINOSAUR as u64) {
                    // The targeted monster becomes a Dinosaur monster (until the end of this turn)
                    // local e1=Effect.CreateEffect(e:GetHandler())
                    let h = effect::get_handler(d, a.e).nil();
                    let e1 = effect::create_effect(d, h);
                    // e1:SetType(EFFECT_TYPE_SINGLE)
                    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_COPY_INHERIT)
                    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_COPY_INHERIT, 0);
                    // e1:SetCode(EFFECT_CHANGE_RACE)
                    effect::set_code(d, e1, EFFECT_CHANGE_RACE);
                    // e1:SetValue(RACE_DINOSAUR)
                    effect::set_value(d, e1, RACE_DINOSAUR as i64);
                    // e1:SetReset(RESETS_STANDARD_PHASE_END)
                    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
                    // tc:RegisterEffect(e1)
                    card::register_effect(d, tc, e1, false);
                }
            }
        }
    }
    // Duel.ShuffleDeck(tp)
    duel::shuffle_deck(d, tp);
}
