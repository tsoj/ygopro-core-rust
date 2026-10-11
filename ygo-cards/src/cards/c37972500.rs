//! GranSolfachord Musecia
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 37972500;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SOLFACHORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_PENDULUM),2,2)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_PENDULUM, sc, st, Some(tp)))),
        2,
        Some(2),
        None,
        None,
        None,
    );
    // to hand (extra)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.xthtg)
    effect::set_target(d, e1, Some(cost_target(xthtg)));
    // e1:SetOperation(s.xthop)
    effect::set_operation(d, e1, Some(operation(xthop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // to hand (deck)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.xthfilter1(c,tp)
fn xthfilter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsType(TYPE_PENDULUM) and not c:IsForbidden()
    //     and Duel.IsExistingMatchingCard(s.xthfilter2,tp,LOCATION_EXTRA,0,1,c,c)
    card::is_type(d, c, TYPE_PENDULUM)
        && !card::is_forbidden(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, cc| xthfilter2(d, cc, c)), tp, LOCATION_EXTRA, 0, 1, c.into())
}
// function s.xthfilter2(c,ac)
fn xthfilter2(d: &mut Duel, c: CardId, ac: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_PENDULUM) and c:IsAbleToHand()
    //     and (c:GetScale()+ac:GetScale()) % 2 ~= 0
    if !(card::is_faceup(d, c) && card::is_type(d, c, TYPE_PENDULUM) && card::is_able_to_hand(d, c, None)) {
        return false;
    }
    let sum = card::get_scale(d, c) + card::get_scale(d, ac);
    sum % 2 != 0
}
// function s.xthtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn xthtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.xthfilter1,tp,LOCATION_HAND,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| xthfilter1(d, c, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
// function s.xthop(e,tp,eg,ep,ev,re,r,rp)
fn xthop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    // local xtc=Duel.SelectMatchingCard(tp,s.xthfilter1,tp,LOCATION_HAND,0,1,1,nil,tp):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| xthfilter1(d, c, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let xtc = group::get_first(d, &sel);
    // if xtc and Duel.SendtoExtraP(xtc,nil,REASON_EFFECT)~=0 then
    if let Some(xtc) = xtc {
        if duel::send_to_extra_p(d, xtc, None, REASON_EFFECT, None) != 0 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
            // local g=Duel.SelectMatchingCard(tp,s.xthfilter2,tp,LOCATION_EXTRA,0,1,1,xtc,xtc)
            let g = duel::select_matching_card(d, tp, Some(&|d, c| xthfilter2(d, c, xtc)), tp, LOCATION_EXTRA, 0, 1, 1, false, xtc.into()).nil();
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
        }
    }
}
// function s.thfilter1(c,e,tp)
fn thfilter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_SOLFACHORD) and c:IsSummonPlayer(tp)
    //     and c:IsPendulumSummoned() and c:IsCanBeEffectTarget(e)
    //     and Duel.IsExistingMatchingCard(s.thfilter2,tp,LOCATION_DECK,0,1,nil,c:GetScale())
    if !(card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_SOLFACHORD as u16])
        && card::is_summon_player(d, c, tp)
        && card::is_pendulum_summoned(d, c)
        && card::is_can_be_effect_target(d, c, Some(e)))
    {
        return false;
    }
    let lv = card::get_scale(d, c);
    duel::is_existing_matching_card(d, Some(&|d, cc| thfilter2(d, cc, lv)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
// function s.thfilter2(c,lv)
fn thfilter2(d: &mut Duel, c: CardId, lv: u32) -> bool {
    // return c:IsType(TYPE_PENDULUM) and c:IsSetCard(SET_SOLFACHORD) and c:IsLevel(lv) and c:IsAbleToHand()
    card::is_type(d, c, TYPE_PENDULUM) && card::is_set_card(d, c, &[SET_SOLFACHORD as u16]) && card::is_level(d, c, &[lv]) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    let eg = a.eg.clone().nil();
    // if chk==0 then return eg:IsExists(s.thfilter1,1,nil,e,tp) end
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &|d, c| thfilter1(d, c, e, tp), 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SELECT)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SELECT as u64);
    // local g=eg:FilterSelect(tp,s.thfilter1,1,1,nil,e,tp)
    let g = group::filter_select(d, &eg, tp, &|d, c| thfilter1(d, c, e, tp), 1, 1, false, NO_EX).nil();
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not (tc and tc:IsRelateToEffect(e) and tc:IsFaceup()) then return end
    let tc = match tc {
        Some(tc) if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) => tc,
        _ => return,
    };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter2,tp,LOCATION_DECK,0,1,1,nil,tc:GetScale())
    let lv = card::get_scale(d, tc);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| thfilter2(d, c, lv)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
