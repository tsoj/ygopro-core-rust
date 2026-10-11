//! Dark World Archives
//! 暗黒界の文殿
use super::prelude::*;

pub const ID: u32 = 76672730;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DARK_WORLD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Discard 1 "Dark World" monster and increase the ATK of "Dark World" monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_HANDES+CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_HANDES + CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(cost_target(atktg)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Discard 1 card, then draw 2 cards
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_HANDES+CATEGORY_DRAW)
    effect::set_category(d, e3, CATEGORY_HANDES + CATEGORY_DRAW);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e3:SetCode(EVENT_DISCARD)
    effect::set_code(d, e3, EVENT_DISCARD);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.drwcond)
    effect::set_condition(d, e3, Some(condition(drwcond)));
    // e3:SetTarget(s.drwtg)
    effect::set_target(d, e3, Some(cost_target(drwtg)));
    // e3:SetOperation(s.drwop)
    effect::set_operation(d, e3, Some(operation(drwop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_DARK_WORLD) and c:IsMonster() and c:IsDiscardable()
    card::is_set_card(d, c, &[SET_DARK_WORLD as u16]) && card::is_monster(d, c) && card::is_discardable(d, c, None, None)
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,nil)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_DARK_WORLD),tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_HAND, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_DARK_WORLD as u16]))), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, tp, 1);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
    // local dg=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_HAND,0,1,1,nil)
    let dg = duel::select_matching_card(d, tp, Some(&cfilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #dg>0 and Duel.SendtoGrave(dg,REASON_EFFECT|REASON_DISCARD)>0 then
    if !dg.is_empty() && duel::send_to_grave(d, dg, REASON_EFFECT | REASON_DISCARD, None, None) > 0 {
        // local og=Duel.GetOperatedGroup()
        let og = duel::get_operated_group(d);
        // local sg=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_DARK_WORLD),tp,LOCATION_MZONE,0,nil)
        let sg = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_DARK_WORLD as u16]))), tp, LOCATION_MZONE, 0, NO_EX);
        // if #sg==0 then return end
        if sg.is_empty() {
            return;
        }
        // local atk=og:GetFirst():GetLevel()*100
        let first = group::get_first(d, &og).nil();
        let atk = card::get_level(d, first) as i64 * 100;
        // for tc in sg:Iter() do
        for tc in group::iter(d, &sg) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(atk)
            effect::set_value(d, e1, atk);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
/// `s.dfilter(c,tp)`
fn dfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsPreviousControler(tp) and c:IsOriginalRace(RACE_FIEND) and c:IsReason(REASON_EFFECT)
    card::is_controler(d, c, tp) && card::is_previous_controler(d, c, tp) && card::is_original_race(d, c, RACE_FIEND) && card::is_reason(d, c, REASON_EFFECT)
}
/// `s.drwcond(e,tp,eg,ep,ev,re,r,rp)`
fn drwcond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return eg:IsExists(s.dfilter,1,nil,tp) and (re:GetHandler():IsSetCard(SET_DARK_WORLD) or rp==1-tp)
    let eg = a.eg.clone().nil();
    if !group::is_exists(d, &eg, &|d, c| dfilter(d, c, tp), 1, NO_EX) {
        return false;
    }
    let rc = effect::get_handler(d, a.re.nil()).nil();
    card::is_set_card(d, rc, &[SET_DARK_WORLD as u16]) || a.rp == 1 - tp
}
/// `s.drwtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drwtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil) and Duel.IsPlayerCanDraw(tp,2) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), tp, LOCATION_HAND, 0, 1, NO_EX) && duel::is_player_can_draw(d, tp, 2);
    }
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, tp, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 2);
    true
}
/// `s.drwop(e,tp,eg,ep,ev,re,r,rp)`
fn drwop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsDiscardable,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_discardable(d, c, None, None)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SendtoGrave(g,REASON_EFFECT|REASON_DISCARD)>0 then
    if !g.is_empty() && duel::send_to_grave(d, g, REASON_EFFECT | REASON_DISCARD, None, None) > 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,2,REASON_EFFECT)
        duel::draw(d, tp, 2, REASON_EFFECT);
    }
}
