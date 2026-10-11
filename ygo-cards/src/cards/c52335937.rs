//! Exosister Betrayal
use super::prelude::*;

pub const ID: u32 = 52335937;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_EXOSISTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_HANDES);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SUMMON)
    effect::set_category(d, e2, CATEGORY_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.nstg)
    effect::set_target(d, e2, Some(cost_target(nstg)));
    // e2:SetOperation(s.nsop)
    effect::set_operation(d, e2, Some(operation(nsop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3a=Effect.CreateEffect(c)
    let e3a = effect::create_effect(d, c);
    // e3a:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3a, aux::stringid(ID, 2));
    // e3a:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e3a, CATEGORY_REMOVE);
    // e3a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3a:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3a, EFFECT_FLAG_DELAY, 0);
    // e3a:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e3a, EVENT_CUSTOM + ID);
    // e3a:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3a, LOCATION_SZONE);
    // e3a:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3a, 1, ID, 2, 0);
    // e3a:SetTarget(s.rmtg)
    effect::set_target(d, e3a, Some(cost_target(rmtg)));
    // e3a:SetOperation(s.rmop)
    effect::set_operation(d, e3a, Some(operation(rmop)));
    // c:RegisterEffect(e3a)
    card::register_effect(d, c, e3a, false);
    // local g=Group.CreateGroup()
    let g = CardSet::new();
    // e3a:SetLabelObject(g)
    effect::set_label_object(d, e3a, Value::Group(g));
    // local e3b=Effect.CreateEffect(c)
    let e3b = effect::create_effect(d, c);
    // e3b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3b, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3b:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3b, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3b:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3b, EVENT_TO_GRAVE);
    // e3b:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3b, LOCATION_SZONE);
    // e3b:SetLabelObject(e3a)
    effect::set_label_object(d, e3b, Value::Effect(e3a));
    // e3b:SetOperation(s.regop)
    effect::set_operation(d, e3b, Some(operation(regop)));
    // c:RegisterEffect(e3b)
    card::register_effect(d, c, e3b, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_EXOSISTER) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_EXOSISTER as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,2,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 2, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 2, a.tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,1,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 1, a.tp, 1);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
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
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.DiscardHand(tp,nil,1,1,REASON_EFFECT|REASON_DISCARD,nil)
        duel::discard_hand(d, a.tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
    }
}
/// `s.tgfilter(c,tp)`
fn tgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and Duel.IsExistingMatchingCard(s.nsfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil,c)
    card::is_faceup(d, c) && duel::is_existing_matching_card(d, Some(&|d, x| nsfilter(d, x, c)), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.nsfilter(c,tc)`
fn nsfilter(d: &mut Duel, c: CardId, tc: CardId) -> bool {
    // return c:IsSetCard(SET_EXOSISTER) and c:IsSummonable(true,nil) and tc:ListsCode(c:GetCode())
    let codes = card::get_code(d, c);
    card::is_set_card(d, c, &[SET_EXOSISTER as u16]) && card::is_summonable(d, c, true, None, 0, None) && card::lists_code(d, tc, &codes)
}
/// `s.nstg(e,tp,...,chk,chkc)`
fn nstg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.tgfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    let _g = duel::select_target(d, tp, Some(&|d, c| tgfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_MZONE) as i32);
    true
}
/// `s.nsop(e,tp,...)`
fn nsop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.nsfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil,tc):GetFirst()
        let g = duel::select_matching_card(d, a.tp, Some(&|d, c| nsfilter(d, c, tc)), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        // if sc then
        if let Some(sc) = group::get_first(d, &g) {
            // Duel.Summon(tp,sc,true,nil)
            duel::summon(d, a.tp, sc, true, None, 0, None);
        }
    }
}
/// `s.regfilter(c,opp)`
fn regfilter(d: &mut Duel, c: CardId, opp: u8) -> bool {
    // return c:IsMonster() and c:IsControler(opp)
    card::is_monster(d, c) && card::is_controler(d, c, opp)
}
/// `s.regop(e,tp,...)`
fn regop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=eg:Filter(s.regfilter,nil,1-tp)
    let eg = a.eg.clone().nil();
    let tg = group::filter(d, &eg, &|d, c| regfilter(d, c, 1 - tp), NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // for tc in tg:Iter() do
        for tc in group::iter(d, &tg) {
            // tc:RegisterFlagEffect(id,RESET_CHAIN,0,1)
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        // local g=e:GetLabelObject():GetLabelObject()
        let e3a = effect::get_label_object(d, a.e).as_effect().nil();
        let mut g = effect::get_label_object(d, e3a).as_group().cloned().nil();
        // if Duel.GetCurrentChain()==0 then g:Clear() end
        if duel::get_current_chain(d, false) == 0 {
            g = CardSet::new();
        }
        // g:Merge(tg)
        group::add_card(&mut g, &tg);
        // g:Remove(function(c) return c:GetFlagEffect(id)==0 end,nil)
        group::remove(d, &mut g, &|d, c| card::get_flag_effect(d, c, ID) == 0, NO_EX);
        // e:GetLabelObject():SetLabelObject(g)
        effect::set_label_object(d, e3a, Value::Group(g));
        // Duel.RaiseSingleEvent(e:GetHandler(),EVENT_CUSTOM+id,e,0,tp,tp,0)
        let h = effect::get_handler(d, a.e).nil();
        duel::raise_single_event(d, h, EVENT_CUSTOM + ID, Some(a.e), 0, tp, tp, 0);
    }
}
/// `s.rmfilter(c,opp)`
fn rmfilter(d: &mut Duel, c: CardId, opp: u8) -> bool {
    // return c:IsControler(opp) and c:IsLocation(LOCATION_GRAVE) and c:IsAbleToRemove(opp)
    card::is_controler(d, c, opp) && card::is_location(d, c, LOCATION_GRAVE) && card::is_able_to_remove(d, c, Some(opp), Some(POS_FACEUP as u8), Some(REASON_EFFECT))
}
/// `s.rmtg(e,tp,...,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=e:GetLabelObject():Filter(s.rmfilter,nil,1-tp)
    let lo = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let g = group::filter(d, &lo, &|d, c| rmfilter(d, c, 1 - tp), NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,1-tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 1, 1 - tp, 0);
    true
}
/// `s.rmop(e,tp,...)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetTargetCards(e)
    let mut g = duel::get_target_cards(d, a.e).nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local opp=1-tp
    let opp = 1 - tp;
    // Duel.Hint(HINT_SELECTMSG,opp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, opp, HINTMSG_REMOVE as u64);
    // g=g:FilterSelect(opp,Card.IsAbleToRemove,1,1,nil,opp)
    g = group::filter_select(d, &g, opp, &|d, c| card::is_able_to_remove(d, c, Some(opp), Some(POS_FACEUP as u8), Some(REASON_EFFECT)), 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.Remove(g,POS_FACEUP,REASON_EFFECT,PLAYER_NONE,opp)
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, Some(PLAYER_NONE as u8), Some(opp));
    }
}
