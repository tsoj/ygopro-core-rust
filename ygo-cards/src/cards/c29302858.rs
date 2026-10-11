//! Vanquish Soul Razen
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 29302858;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VANQUISH_SOUL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Search 1 non-Warrior "Vanquish Soul" monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.opccost)
    effect::set_cost(d, e1, Some(cost_target(opccost)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Make this card indestructible by effects, OR destroy all monsters in this card's column
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCost(s.opccost)
    effect::set_cost(d, e3, Some(cost_target(opccost)));
    // e3:SetTarget(s.vstg)
    effect::set_target(d, e3, Some(cost_target(vstg)));
    // e3:SetOperation(s.vsop)
    effect::set_operation(d, e3, Some(operation(vsop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.opccost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn opccost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFlagEffect(tp,id)==0 end
    if a.chk == Some(0) {
        return duel::get_flag_effect(d, a.tp, ID) == 0;
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_CHAIN,0,1)
    duel::register_flag_effect(d, a.tp, ID, RESET_CHAIN, 0, 1, 0);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_VANQUISH_SOUL) and c:IsMonster() and not c:IsRace(RACE_WARRIOR) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_VANQUISH_SOUL as u16])
        && card::is_monster(d, c)
        && !card::is_race(d, c, RACE_WARRIOR)
        && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
/// `s.vscostfilter(c,att)`
fn vscostfilter(d: &mut Duel, c: CardId, att: u32) -> bool {
    // return c:IsAttribute(att) and not c:IsPublic()
    card::is_attribute(d, c, att) && !card::is_public(d, c)
}
/// `s.vsrescon(sg)`
fn vsrescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:FilterCount(Card.IsAttribute,nil,ATTRIBUTE_FIRE)>0
    //     and sg:FilterCount(Card.IsAttribute,nil,ATTRIBUTE_DARK)>0
    let res = group::filter_count(d, sg, &|d, c| card::is_attribute(d, c, ATTRIBUTE_FIRE), NO_EX) > 0
        && group::filter_count(d, sg, &|d, c| card::is_attribute(d, c, ATTRIBUTE_DARK), NO_EX) > 0;
    (res, false)
}
/// `s.vstg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn vstg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local cg1=Duel.GetMatchingGroup(s.vscostfilter,tp,LOCATION_HAND,0,nil,ATTRIBUTE_FIRE)
    let cg1 = duel::get_matching_group(d, Some(&|d, c| vscostfilter(d, c, ATTRIBUTE_FIRE)), tp, LOCATION_HAND, 0, NO_EX);
    // local b1=#cg1>0
    let b1 = cg1.len() > 0;
    // local cg2=cg1+Duel.GetMatchingGroup(s.vscostfilter,tp,LOCATION_HAND,0,nil,ATTRIBUTE_DARK)
    let mut cg2 = cg1.clone();
    let cg_d = duel::get_matching_group(d, Some(&|d, c| vscostfilter(d, c, ATTRIBUTE_DARK)), tp, LOCATION_HAND, 0, NO_EX);
    group::add_card(&mut cg2, &cg_d);
    // local colg=e:GetHandler():GetColumnGroup():Match(Card.IsLocation,nil,LOCATION_MZONE)
    let h = effect::get_handler(d, a.e).nil();
    let mut colg = card::get_column_group(d, h, 0, 0);
    group::match_(d, &mut colg, &|d, c| card::is_location(d, c, LOCATION_MZONE), NO_EX);
    // local b2=#colg>0 and aux.SelectUnselectGroup(cg2,e,tp,1,2,s.vsrescon,0)
    let rs: aux::Rescon = &vsrescon;
    let b2 = colg.len() > 0
        && matches!(aux::select_unselect_group(d, &cg2, Some(a.e), tp, Some(1), Some(2), Some(rs), 0, tp, None, None, None, false), Err(true));
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,2)}, {b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op.map_or(0, |v| v as i64)]);
    // if op==1 then
    if op == Some(1) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
        // local g=cg1:Select(tp,1,1,nil)
        let g = group::select(d, &cg1, tp, 1, 1, false, NO_EX).nil();
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
    // elseif op==2 then
    } else if op == Some(2) {
        // local g=aux.SelectUnselectGroup(cg2,e,tp,1,2,s.vsrescon,1,tp,HINTMSG_CONFIRM,s.vsrescon)
        let g = aux::select_unselect_group(
            d,
            &cg2,
            Some(a.e),
            tp,
            Some(1),
            Some(2),
            Some(rs),
            1,
            tp,
            Some(HINTMSG_CONFIRM as u64),
            Some(rs),
            None,
            false,
        )
        .unwrap_or_default();
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // e:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, a.e, CATEGORY_DESTROY);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,colg,#colg,0,0)
        let n = colg.len() as u8;
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(colg), n, 0, 0);
    }
    true
}
/// `s.vsop(e,tp,eg,ep,ev,re,r,rp)`
fn vsop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 and c:IsFaceup() then
    if op == 1 && card::is_faceup(d, c) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3001)
        effect::set_description(d, e1, 3001);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    // elseif op==2 then
    } else if op == 2 {
        // local cg=c:GetColumnGroup():Match(Card.IsLocation,nil,LOCATION_MZONE)
        let mut cg = card::get_column_group(d, c, 0, 0);
        group::match_(d, &mut cg, &|d, x| card::is_location(d, x, LOCATION_MZONE), NO_EX);
        // if #cg==0 then return end
        if cg.is_empty() {
            return;
        }
        // Duel.Destroy(cg,REASON_EFFECT)
        duel::destroy(d, cg, REASON_EFFECT, None, None);
    }
}
