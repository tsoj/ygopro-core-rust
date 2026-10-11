//! Battlewasp Wind
use super::prelude::*;

pub const ID: u32 = 67441879;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BATTLEWASP], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // e0:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e0, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Add 1 "Battlewasp" monster from your Deck to your hand with less ATK than a "Battlewasp" monster that was Normal or Special Summoned
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e1, EVENT_CUSTOM + ID);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(2)
    effect::set_count_limit(d, e1, 2, 0, 0, 0);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local g=Group.CreateGroup()
    // g:KeepAlive()
    let g = CardSet::new();
    // e1:SetLabelObject(g)
    effect::set_label_object(d, e1, Value::Group(g));
    // Keep track of the "Battlewasp" monsters that are Normal or Special Summoned to your field
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1a, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1a:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1a, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1a, EVENT_SUMMON_SUCCESS);
    // e1a:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1a, LOCATION_SZONE);
    // e1a:SetLabelObject(e1)
    effect::set_label_object(d, e1a, Value::Effect(e1));
    // e1a:SetOperation(s.regop)
    effect::set_operation(d, e1a, Some(operation(regop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // Make 1 Insect monster you control able to be used as a Tuner that turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.tunertg)
    effect::set_target(d, e2, Some(cost_target(tunertg)));
    // e2:SetOperation(s.tunerop)
    effect::set_operation(d, e2, Some(operation(tunerop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.tgfilter(c,e,tp)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_BATTLEWASP) and c:IsLocation(LOCATION_MZONE) and c:IsFaceup() and c:IsControler(tp)
    //     and c:IsCanBeEffectTarget(e) and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,c:GetAttack())
    card::is_set_card(d, c, &[SET_BATTLEWASP as u16])
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_faceup(d, c)
        && card::is_controler(d, c, tp)
        && card::is_can_be_effect_target(d, c, Some(e))
        && {
            let atk = card::get_attack(d, c);
            duel::is_existing_matching_card(d, Some(&|d, cc| thfilter(d, cc, atk)), tp, LOCATION_DECK, 0, 1, NO_EX)
        }
}
/// `s.thfilter(c,atk)`
fn thfilter(d: &mut Duel, c: CardId, atk: i32) -> bool {
    // return c:IsSetCard(SET_BATTLEWASP) and c:IsMonster() and c:GetAttack()<atk and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_BATTLEWASP as u16])
        && card::is_monster(d, c)
        && card::get_attack(d, c) < atk
        && card::is_able_to_hand(d, c, None)
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local tg=eg:Filter(s.tgfilter,nil,e,tp)
    let eg = a.eg();
    let tg = group::filter(d, &eg, &|d, c| tgfilter(d, c, e, tp), NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // for tc in aux.Next(tg) do
        for tc in group::iter(d, &tg) {
            // tc:RegisterFlagEffect(id,RESET_CHAIN,0,1)
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        // local g=e:GetLabelObject():GetLabelObject()
        let e1 = effect::get_label_object(d, e).as_effect().nil();
        let mut g = effect::get_label_object(d, e1).as_group().cloned().nil();
        // if Duel.GetCurrentChain()==0 then g:Clear() end
        if duel::get_current_chain(d, false) == 0 {
            g = CardSet::new();
        }
        // g:Merge(tg)
        group::add_card(&mut g, &tg);
        // g:Remove(function(c) return c:GetFlagEffect(id)==0 end,nil)
        group::remove(d, &mut g, &|d, c| card::get_flag_effect(d, c, ID) == 0, NO_EX);
        // e:GetLabelObject():SetLabelObject(g)
        effect::set_label_object(d, e1, Value::Group(g.clone()));
        // if #g>0 and not Duel.HasFlagEffect(tp,id) then
        if !g.is_empty() && !duel::has_flag_effect(d, tp, ID, None) {
            // Duel.RegisterFlagEffect(tp,id,RESET_CHAIN,0,1)
            duel::register_flag_effect(d, tp, ID, RESET_CHAIN, 0, 1, 0);
            // Duel.RaiseEvent(g,EVENT_CUSTOM+id,re,r,tp,ep,ev)
            duel::raise_event(d, g, EVENT_CUSTOM + ID, a.re, a.r, tp, a.ep, a.ev);
        }
    }
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // local g=e:GetLabelObject():Filter(s.tgfilter,nil,e,tp)
    let stored = effect::get_label_object(d, e).as_group().cloned().nil();
    let mut g = group::filter(d, &stored, &|d, c| tgfilter(d, c, e, tp), NO_EX);
    // if chkc then return g:IsContains(chkc) and s.tgfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&g, chkc) && tgfilter(d, chkc, e, tp);
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // Duel.ResetFlagEffect(tp,id)
        duel::reset_flag_effect(d, tp, ID);
        // for tc in g:Iter() do tc:ResetFlagEffect(id) end
        for tc in group::iter(d, &g) {
            card::reset_flag_effect(d, tc, ID);
        }
        // return rp==tp and #g>0 and not e:GetHandler():HasFlagEffect(id)
        let c = effect::get_handler(d, e).nil();
        return a.rp == tp && !g.is_empty() && !card::has_flag_effect(d, c, ID, None);
    }
    // e:GetHandler():RegisterFlagEffect(id,RESET_CHAIN,0,1)
    let c = effect::get_handler(d, e).nil();
    card::register_flag_effect(d, c, ID, RESET_CHAIN, 0, 1, 0, 0);
    // if #g==1 then
    if g.len() == 1 {
        // Duel.SetTargetCard(g)
        duel::set_target_card(d, g.clone());
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
        // g=g:Select(tp,1,1,nil)
        g = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // Duel.SetTargetCard(g)
        duel::set_target_card(d, g.clone());
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and tc:IsFaceup()) then return end
    if !(card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,tc:GetAttack())
    let atk = card::get_attack(d, tc);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| thfilter(d, c, atk)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
/// `s.tunertgfilter(c)`
fn tunertgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_INSECT) and c:HasLevel() and not c:IsType(TYPE_TUNER) and not c:IsHasEffect(EFFECT_CAN_BE_TUNER)
    //     and c:IsFaceup()
    card::is_race(d, c, RACE_INSECT)
        && card::has_level(d, c)
        && !card::is_type(d, c, TYPE_TUNER)
        && card::is_has_effect(d, c, EFFECT_CAN_BE_TUNER, None).is_empty()
        && card::is_faceup(d, c)
}
/// `s.tunertg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tunertg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.tunertgfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && tunertgfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tunertgfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tunertgfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.tunertgfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let _ = duel::select_target(d, tp, Some(&tunertgfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.tunerop(e,tp,eg,ep,ev,re,r,rp)`
fn tunerop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, e) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let c = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,2))
        effect::set_description(d, e1, aux::stringid(ID, 2));
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CAN_BE_TUNER)
        effect::set_code(d, e1, EFFECT_CAN_BE_TUNER);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
