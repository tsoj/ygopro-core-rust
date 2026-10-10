//! Dragon's Light and Darkness
use super::prelude::*;

pub const ID: u32 = 63295720;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP,TIMING_DAMAGE_STEP|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, Some(TIMING_DAMAGE_STEP | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tdfilter(c,tp)`
fn tdfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsLevel(8) and c:IsRace(RACE_DRAGON) and c:IsAbleToDeck()
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,c:GetAttribute())
    card::is_level(d, c, &[8]) && card::is_race(d, c, RACE_DRAGON as u64) && card::is_able_to_deck(d, c) && {
        let attr = card::get_attribute(d, c);
        duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x, attr)), tp, LOCATION_DECK, 0, 1, NO_EX)
    }
}
/// `s.thfilter(c,attr)`
fn thfilter(d: &mut Duel, c: CardId, attr: u32) -> bool {
    // return c:IsLevel(8) and c:IsRace(RACE_DRAGON) and not c:IsAttribute(attr) and c:IsAbleToHand()
    card::is_level(d, c, &[8]) && card::is_race(d, c, RACE_DRAGON as u64) && !card::is_attribute(d, c, attr) && card::is_able_to_hand(d, c, None)
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DRAGON) and c:IsFaceup() and c:HasLevel()
    card::is_race(d, c, RACE_DRAGON as u64) && card::is_faceup(d, c) && card::has_level(d, c)
}
/// `s.disfilter(c,tp)`
fn disfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRace(RACE_DRAGON) and c:IsFaceup() and c:IsLocation(LOCATION_MZONE) and c:IsControler(tp)
    card::is_race(d, c, RACE_DRAGON as u64) && card::is_faceup(d, c) && card::is_location(d, c, LOCATION_MZONE) && card::is_controler(d, c, tp)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return e:GetLabel()==2 and chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.atkfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return effect::get_label(d, a.e) == 2
            && card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, tp)
            && atkfilter(d, chkc);
    }
    // local dmg_step=Duel.IsPhase(PHASE_DAMAGE)
    let dmg_step = duel::is_phase(d, PHASE_DAMAGE);
    // local ex,_,_,te_ev,te_re=Duel.CheckEvent(EVENT_CHAINING,true)
    let pe = duel::check_event(d, EVENT_CHAINING);
    let ex = pe.is_some();
    let te_ev = pe.as_ref().map(|p| p.event_value);
    let te_re = pe.as_ref().and_then(|p| p.reason_effect);
    // local te_tg=te_ev and Duel.GetChainInfo(te_ev,CHAININFO_TARGET_CARDS) or nil
    let te_tg = te_ev.and_then(|ev| duel::get_chain_info(d, ev as u8, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned());
    // Shuffle 1 Level 8 Dragon monster from your hand into the Deck, and if you do, add 1 Level 8 Dragon monster with a different Attribute from your Deck to your hand
    // local b1=not Duel.HasFlagEffect(tp,id) and not dmg_step
    //     and Duel.IsExistingMatchingCard(s.tdfilter,tp,LOCATION_HAND,0,1,nil,tp)
    let b1 = !duel::has_flag_effect(d, tp, ID, None) && !dmg_step
        && duel::is_existing_matching_card(d, Some(&|d, x| tdfilter(d, x, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    // Make 1 Dragon monster you control gain ATK equal to its Level x 200
    // local b2=not Duel.HasFlagEffect(tp,id+1) and not (dmg_step and Duel.IsDamageCalculated())
    //     and Duel.IsExistingTarget(s.atkfilter,tp,LOCATION_MZONE,0,1,nil)
    let b2 = !duel::has_flag_effect(d, tp, ID + 1, None)
        && !(dmg_step && duel::is_damage_calculated(d))
        && duel::is_existing_target(d, Some(&atkfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    // Negate an effect that targets a Dragon monster(s) you control
    // local b3=not Duel.HasFlagEffect(tp,id+2) and not dmg_step
    //     and ex and te_re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) and te_tg
    //     and te_tg:IsExists(s.disfilter,1,nil,tp) and Duel.IsChainDisablable(ev)
    let b3 = !duel::has_flag_effect(d, tp, ID + 2, None)
        && !dmg_step
        && ex
        && effect::is_has_property(d, te_re.nil(), EFFECT_FLAG_CARD_TARGET, 0)
        && te_tg.as_ref().is_some_and(|tg| group::is_exists(d, tg, &|d, x| disfilter(d, x, tp), 1, NO_EX))
        && duel::is_chain_disablable(d, a.ev as u8);
    // if chk==0 then return b1 or b2 or b3 end
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)},
    //     {b3,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3))]);
    // e:SetLabel(op,te_ev or 0)
    effect::set_label(d, a.e, &[op.map_or(0, |v| v as i64), te_ev.unwrap_or(0) as i64]);
    // if op==1 then
    if op == Some(1) {
        // e:SetCategory(CATEGORY_TODECK+CATEGORY_TOHAND+CATEGORY_SEARCH)
        effect::set_category(d, a.e, CATEGORY_TODECK + CATEGORY_TOHAND + CATEGORY_SEARCH);
        // e:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
        effect::set_property(d, a.e, EFFECT_FLAG_DAMAGE_STEP, 0);
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_HAND)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, LOCATION_HAND as i32);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // elseif op==2 then
    } else if op == Some(2) {
        // e:SetCategory(CATEGORY_ATKCHANGE)
        effect::set_category(d, a.e, CATEGORY_ATKCHANGE);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
        // Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATKDEF as u64);
        // Duel.SelectTarget(tp,s.atkfilter,tp,LOCATION_MZONE,0,1,1,nil)
        duel::select_target(d, tp, Some(&atkfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // elseif op==3 then
    } else if op == Some(3) {
        // e:SetCategory(CATEGORY_DISABLE)
        effect::set_category(d, a.e, CATEGORY_DISABLE);
        // e:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
        effect::set_property(d, a.e, EFFECT_FLAG_DAMAGE_STEP, 0);
        // Duel.RegisterFlagEffect(tp,id+2,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID + 2, RESET_PHASE | PHASE_END, 0, 1, 0);
        // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, tp, 0);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op,te_ev=e:GetLabel()
    let labels = effect::get_labels(d, a.e);
    let op = labels.first().copied().unwrap_or(0);
    let te_ev = labels.get(1).copied();
    // if op==1 then
    if op == 1 {
        // Shuffle 1 Level 8 Dragon monster from your hand into the Deck, and if you do, add 1 Level 8 Dragon monster with a different Attribute from your Deck to your hand
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.tdfilter,tp,LOCATION_HAND,0,1,1,nil,tp):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&|d, x| tdfilter(d, x, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        let Some(sc) = group::get_first(d, &g) else { return };
        // if not sc then return end
        // Duel.ConfirmCards(1-tp,sc)
        duel::confirm_cards(d, 1 - tp, CardSet::from_card(sc));
        // if Duel.SendtoDeck(sc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)==0 or not sc:IsLocation(LOCATION_DECK) then return end
        if duel::send_to_deck(d, CardSet::from_card(sc), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) == 0 || !card::is_location(d, sc, LOCATION_DECK) {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,sc:GetAttribute())
        let attr = card::get_attribute(d, sc);
        let g = duel::select_matching_card(d, tp, Some(&|d, x| thfilter(d, x, attr)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
        }
    // elseif op==2 then
    } else if op == 2 {
        // Make 1 Dragon monster you control gain ATK equal to its Level x 200
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:HasLevel() then
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && card::has_level(d, tc) {
            // Gains ATK equal to its Level x 200
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(tc:GetLevel()*200)
            let lv = card::get_level(d, tc) as i64;
            effect::set_value(d, e1, lv * 200);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    // elseif op==3 then
    } else if op == 3 {
        // Negate an effect that targets a Dragon monster(s) you control
        // Duel.NegateEffect(te_ev)
        duel::negate_effect(d, te_ev.unwrap_or(0) as u8);
    }
}
