//! Infernoble Knight Captain Roland
use super::prelude::*;

pub const ID: u32 = 40251688;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Send 1 Equip from the Deck to the GY and search 1 Warrior during the End Phase
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsSynchroSummoned() end)
    effect::set_condition(d, e1, Some(lua_fn(e1_condition)));
    // e1:SetOperation(s.regop)
    effect::set_operation(d, e1, Some(operation(regop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Equip itself from GY to 1 warrior monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e2, CATEGORY_EQUIP);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_MAIN_END)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e2, Some(lua_fn(e2_condition)));
    // e2:SetTarget(s.equiptg)
    effect::set_target(d, e2, Some(cost_target(equiptg)));
    // e2:SetOperation(s.equipop)
    effect::set_operation(d, e2, Some(operation(equipop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e) return e:GetHandler():IsSynchroSummoned() end`
fn e1_condition(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_synchro_summoned(d, h)
}
/// `function() return Duel.IsMainPhase() end`
fn e2_condition(d: &mut Duel, _a: Args) -> bool {
    duel::is_main_phase(d, None)
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.tgcon)
    effect::set_condition(d, e1, Some(condition(tgcon)));
    // e1:SetOperation(s.tgop)
    effect::set_operation(d, e1, Some(operation(tgop)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_EQUIP) and c:IsSpell() and c:IsAbleToGrave()
    card::is_type(d, c, TYPE_EQUIP) && card::is_spell(d, c) && card::is_able_to_grave(d, c, None)
}
/// `s.tgcon(e,tp,eg,ep,ev,re,r,rp)`
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil)
    duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_WARRIOR) and c:IsAbleToHand()
    card::is_race(d, c, RACE_WARRIOR as u64) && card::is_able_to_hand(d, c, None)
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local gc=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let gc = group::get_first(d, &sel);
    // if gc and Duel.SendtoGrave(gc,REASON_EFFECT)>0 and gc:IsLocation(LOCATION_GRAVE) then
    if let Some(gc) = gc {
        if duel::send_to_grave(d, gc, REASON_EFFECT, None, None) > 0 && card::is_location(d, gc, LOCATION_GRAVE) {
            // local sg=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
            let sg = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
            // if #sg==0 then return end
            if sg.is_empty() {
                return;
            }
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
            // local hg=sg:Select(tp,1,1,nil)
            let hg = group::select(d, &sg, tp, 1, 1, false, NO_EX).nil();
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(hg,nil,REASON_EFFECT)
            duel::send_to_hand(d, hg.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,hg)
            duel::confirm_cards(d, 1 - tp, hg);
        }
    }
}
/// `s.equiptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn equiptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and chkc:IsFaceup() and chkc:IsRace(RACE_WARRIOR) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && card::is_faceup(d, chkc) && card::is_race(d, chkc, RACE_WARRIOR as u64);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingTarget(aux.FaceupFilter(Card.IsRace,RACE_WARRIOR),tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_WARRIOR as u64)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsRace,RACE_WARRIOR),tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_WARRIOR as u64)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.equipop(e,tp,eg,ep,ev,re,r,rp)`
fn equipop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or Duel.GetLocationCount(tp,LOCATION_SZONE)==0 then return end
    if !card::is_relate_to_effect(d, c, a.e) || duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 == 0 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) or tc:IsFacedown() or tc:IsControler(1-tp) then return end
    if !card::is_relate_to_effect(d, tc, a.e) || card::is_facedown(d, tc) || card::is_controler(d, tc, 1 - tp) {
        return;
    }
    // if Duel.Equip(tp,c,tc) then
    if duel::equip(d, tp, c, tc, true, false) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_EQUIP_LIMIT)
        effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // e1:SetValue(function(e,c) return c==e:GetLabelObject() end)
        effect::set_value_fn(d, e1, lua_fn(equip_limit));
        // e1:SetLabelObject(tc)
        effect::set_label_object(d, e1, Value::Card(tc));
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_EQUIP)
        effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
        // e2:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
        // e2:SetValue(500)
        effect::set_value(d, e2, 500);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
}
/// `function(e,c) return c==e:GetLabelObject() end`
fn equip_limit(d: &mut Duel, a: Args) -> bool {
    a.card(1) == effect::get_label_object(d, a.e(0)).as_card()
}
