//! Abyss Actors' Dress Rehearsal
use super::prelude::*;

pub const ID: u32 = 6004133;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ABYSS_ACTOR, SET_ABYSS_SCRIPT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add Add 1 "Abyss Actor" card and 1 "Abyss Script" Spell to the hand
    // local e1=Effect.CreateEffect(c)
    // e1:SetDescription(aux.Stringid(id,0))
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    // e1:SetCode(EVENT_FREE_CHAIN)
    // e1:SetCondition(s.condition)
    // e1:SetTarget(s.target)
    // e1:SetOperation(s.activate)
    // c:RegisterEffect(e1)
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsPhase(PHASE_MAIN1) and not Duel.CheckPhaseActivity()
    duel::is_phase(d, PHASE_MAIN1) && !duel::check_phase_activity(d)
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToHand() and (c:IsSetCard(SET_ABYSS_ACTOR) or (c:IsSpell() and c:IsSetCard(SET_ABYSS_SCRIPT)))
    card::is_able_to_hand(d, c, None)
        && (card::is_set_card(d, c, &[SET_ABYSS_ACTOR as u16]) || (card::is_spell(d, c) && card::is_set_card(d, c, &[SET_ABYSS_SCRIPT as u16])))
}
// function s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(Card.IsSetCard,1,nil,SET_ABYSS_ACTOR) and sg:IsExists(function(c) return c:IsSpell() and c:IsSetCard(SET_ABYSS_SCRIPT) end,1,nil)
    let b1 = group::is_exists(d, sg, &|d, c| card::is_set_card(d, c, &[SET_ABYSS_ACTOR as u16]), 1, NO_EX);
    let b2 = b1 && group::is_exists(d, sg, &|d, c| card::is_spell(d, c) && card::is_set_card(d, c, &[SET_ABYSS_SCRIPT as u16]), 1, NO_EX);
    (b2, false)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
        // return #g>1 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0)
        if g.len() <= 1 {
            return false;
        }
        let rs: aux::Rescon = &rescon;
        return matches!(aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(rs), 0, tp, None, None, None, false), Err(true));
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 2, tp, LOCATION_DECK as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
    // local rg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_ATOHAND)
    let rs: aux::Rescon = &rescon;
    let rg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(rs), 1, tp, Some(HINTMSG_ATOHAND as u64), None, None, false).unwrap_or_default();
    // if #rg>0 then
    if !rg.is_empty() {
        // Duel.SendtoHand(rg,nil,REASON_EFFECT)
        duel::send_to_hand(d, rg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,rg)
        duel::confirm_cards(d, 1 - tp, rg);
    }
    // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // Cannot Pendulum Summon for the rest of the turn, except "Abyss Actor" monsters
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(lua_fn(splimit)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
// function s.splimit(e,c,sump,sumtype,sumpos,targetp,se)
fn splimit(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    // return not c:IsSetCard(SET_ABYSS_ACTOR) and aux.penlimit(e,se,sump,sumtype)
    if card::is_set_card(d, c, &[SET_ABYSS_ACTOR as u16]) {
        return false;
    }
    let args = [Value::Effect(a.e(0)), a.v(6).clone(), a.v(2).clone(), a.v(3).clone()];
    aux::penlimit().call(d, &args).truthy()
}
