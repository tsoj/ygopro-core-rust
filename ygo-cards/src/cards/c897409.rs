//! Galaxy Hundred
use super::prelude::*;

pub const ID: u32 = 897409;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_GALAXYEYES_P_DRAGON], listed_series: &[SET_PHOTON, SET_GALAXY, SET_NUMBER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
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
    // Look at opponent's Extra Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.edcon)
    effect::set_condition(d, e2, Some(condition(edcon)));
    // e2:SetTarget(s.edtg)
    effect::set_target(d, e2, Some(cost_target(edtg)));
    // e2:SetOperation(s.edop)
    effect::set_operation(d, e2, Some(operation(edop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsSetCard(SET_PHOTON) or c:IsSetCard(SET_GALAXY)) and c:IsAbleToGrave()
    (card::is_set_card(d, c, &[SET_PHOTON as u16]) || card::is_set_card(d, c, &[SET_GALAXY as u16])) && card::is_able_to_grave(d, c, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&tgfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if #g==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,0)) then return end
    if g.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local tg=g:Select(tp,1,1,nil)
    let tg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.SendtoGrave(tg,REASON_EFFECT)
        duel::send_to_grave(d, tg, REASON_EFFECT, None, None);
    }
}
/// `s.edconfilter(c,tp)`
fn edconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsCode(CARD_GALAXYEYES_P_DRAGON) and c:IsFaceup() and c:IsControler(tp)
    card::is_code(d, c, &[CARD_GALAXYEYES_P_DRAGON]) && card::is_faceup(d, c) && card::is_controler(d, c, tp)
}
/// `s.edcon(e,tp,eg,ep,ev,re,r,rp)`
fn edcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.edconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| edconfilter(d, c, tp), 1, NO_EX)
}
/// `s.edtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn edtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_EXTRA)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, tp, 0, LOCATION_EXTRA) > 0;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - tp, LOCATION_EXTRA as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,1-tp,LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 1 - tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_NUMBER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
    card::is_set_card(d, c, &[SET_NUMBER as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
}
/// `s.edop(e,tp,eg,ep,ev,re,r,rp)`
fn edop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_EXTRA)
    let g = duel::get_field_group(d, tp, 0, LOCATION_EXTRA);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.ConfirmCards(tp,g)
    duel::confirm_cards(d, tp, g.clone());
    // local rg=g:Filter(Card.IsAbleToRemove,nil,tp,POS_FACEUP)
    let rg = group::filter(d, &g, &|d, c| card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEUP as u8), None), NO_EX);
    // local sg=g:Filter(s.spfilter,nil,e,tp)
    let sg = group::filter(d, &g, &|d, c| spfilter(d, c, e, tp), NO_EX);
    // local b1=#rg>0
    let b1 = !rg.is_empty();
    // local b2=#sg>0
    let b2 = !sg.is_empty();
    // if not ((b1 or b2) and Duel.SelectYesNo(tp,aux.Stringid(id,2))) then return Duel.ShuffleExtra(1-tp) end
    if !((b1 || b2) && duel::select_yes_no(d, tp, aux::stringid(ID, 2))) {
        duel::shuffle_extra(d, 1 - tp);
        return;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,3)},{b2,aux.Stringid(id,4)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 3)), (b2, aux::stringid(ID, 4))]);
    // Banish 1 monster
    // if op==1 then
    if op == Some(1) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        // local tg=rg:Select(tp,1,1,nil)
        let tg = group::select(d, &rg, tp, 1, 1, false, NO_EX).nil();
        // if #tg==0 then return end
        if tg.is_empty() {
            return;
        }
        // Duel.Remove(tg,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, tg, POS_FACEUP as u8, REASON_EFFECT, None, None);
    // Special Summon 1 "Number"
    // elseif op==2 then
    } else if op == Some(2) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local tg=sg:Select(tp,1,1,nil)
        let tg = group::select(d, &sg, tp, 1, 1, false, NO_EX).nil();
        // if #tg==0 then return end
        if tg.is_empty() {
            return;
        }
        // Duel.SpecialSummon(tg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
    // Duel.ShuffleExtra(1-tp)
    duel::shuffle_extra(d, 1 - tp);
}
