//! Repair Genex Controller
use super::prelude::*;

pub const ID: u32 = 8173184;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[8173184],
    listed_series: &[SET_GENEX],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // 1 Level 4 or lower "Genex" Monster
    // Link.AddProcedure(c,s.matfilter,1,1)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| matfilter(d, c, sc, st, tp))), 1, Some(1), None, None, None);
    // Can only Special Summon "Repaired Genex Controller" once per turn
    // c:SetSPSummonOnce(id)
    card::set_spsummon_once(d, c, ID);
    // Add 1 "Genex" monster from the GY to the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(function(e) return e:GetHandler():IsLinkSummoned() end)
    effect::set_condition(d, e1, Some(condition(linkcon)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Normal Summon 1 "Genex" monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SUMMON)
    effect::set_category(d, e2, CATEGORY_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_HAND)
    effect::set_code(d, e2, EVENT_TO_HAND);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,0,EFFECT_COUNT_CODE_CHAIN)
    effect::set_count_limit(d, e2, 1, 0, 0, EFFECT_COUNT_CODE_CHAIN as u8);
    // e2:SetCondition(function(_,tp,eg) return eg:IsExists(s.nsconfilter,1,nil,tp) end)
    effect::set_condition(d, e2, Some(condition(nscon)));
    // e2:SetTarget(s.nstg)
    effect::set_target(d, e2, Some(cost_target(nstg)));
    // e2:SetOperation(s.nsop)
    effect::set_operation(d, e2, Some(operation(nsop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.matfilter(c,sc,st,tp)
fn matfilter(d: &mut Duel, c: CardId, sc: Option<CardId>, st: u64, tp: u8) -> bool {
    // return c:IsSetCard(SET_GENEX,sc,st,tp) and c:IsLevelBelow(4)
    card::is_set_card_ctx(d, c, &[SET_GENEX as u16], sc, st, Some(tp)) && card::is_level_below(d, c, 4)
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GENEX) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_GENEX as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
// function(e) return e:GetHandler():IsLinkSummoned() end
fn linkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, h)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if group::get_count(&g) > 0 {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
// function s.nsconfilter(c,tp)
fn nsconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsSetCard(SET_GENEX) and not c:IsReason(REASON_DRAW)
    card::is_controler(d, c, tp) && card::is_set_card(d, c, &[SET_GENEX as u16]) && !card::is_reason(d, c, REASON_DRAW)
}
// function(_,tp,eg) return eg:IsExists(s.nsconfilter,1,nil,tp) end
fn nscon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| nsconfilter(d, c, tp), 1, NO_EX)
}
// function s.nsfilter(c)
fn nsfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GENEX) and c:IsSummonable(true,nil)
    card::is_set_card(d, c, &[SET_GENEX as u16]) && card::is_summonable(d, c, true, None, 0, Some(0x1f))
}
// function s.nstg(e,tp,eg,ep,ev,re,r,rp,chk)
fn nstg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.nsfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&nsfilter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_MZONE) as i32);
    true
}
// function s.nsop(e,tp,eg,ep,ev,re,r,rp)
fn nsop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.nsfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&nsfilter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if tc then
    if let Some(tc) = tc {
        // Duel.Summon(tp,tc,true,nil)
        duel::summon(d, a.tp, tc, true, None, 0, None);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Cannot Special Summon from the Extra Deck, except by Synchro Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(_e,_c,_tp,st,pos,target_p,sumeff,proc_eff) return _c:IsLocation(LOCATION_EXTRA) and (st&SUMMON_TYPE_SYNCHRO~=SUMMON_TYPE_SYNCHRO or proc_eff==nil) end)
    effect::set_target(d, e1, Some(lua_fn(|d, a| cannot_sp_target(d, &a))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // Clock Lizard Check
    // aux.addTempLizardCheck(c,tp,aux.TRUE)
    aux::add_temp_lizard_check(d, c, a.tp, Some(aux_true()), None, None, None, None);
    // Must use at least 1 "Genex" Tuner to Synchro Summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetCode(EFFECT_SYNCHRO_MATERIAL_CUSTOM)
    effect::set_code(d, e2, EFFECT_SYNCHRO_MATERIAL_CUSTOM);
    // e2:SetTargetRange(LOCATION_MZONE|LOCATION_HAND,0)
    effect::set_target_range(d, e2, LOCATION_MZONE | LOCATION_HAND, 0);
    // e2:SetTarget(function(_,_c) return not (_c:IsSetCard(SET_GENEX) and _c:IsType(TYPE_TUNER)) end)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| !(card::is_set_card(d, c, &[SET_GENEX as u16]) && card::is_type(d, c, TYPE_TUNER)))));
    // e2:SetOperation(s.synop)
    effect::set_operation(d, e2, Some(lua_fn(|d, a| synop(d, &a))));
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
}
// function(e,tp,eg,ep,ev,re,r,rp,chk) of e1 (the Cannot Special Summon target)
// function(_e,_c,_tp,st,pos,target_p,sumeff,proc_eff) return _c:IsLocation(LOCATION_EXTRA) and (st&SUMMON_TYPE_SYNCHRO~=SUMMON_TYPE_SYNCHRO or proc_eff==nil) end
fn cannot_sp_target(d: &mut Duel, a: &Args) -> bool {
    let c = a.c(1);
    let st = a.int(3);
    let proc_nil = a.v(7).is_nil();
    card::is_location(d, c, LOCATION_EXTRA) && ((st & SUMMON_TYPE_SYNCHRO as i64) != SUMMON_TYPE_SYNCHRO as i64 || proc_nil)
}
// function s.gtfilter(c,sc,tp)
fn gtfilter(d: &mut Duel, c: CardId, sc: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GENEX,sc,SUMMON_TYPE_SYNCHRO,tp) and c:IsType(TYPE_TUNER,sc,SUMMON_TYPE_SYNCHRO,tp)
    card::is_set_card_ctx(d, c, &[SET_GENEX as u16], Some(sc), SUMMON_TYPE_SYNCHRO as u64, Some(tp))
        && card::is_type_ctx(d, c, TYPE_TUNER, Some(sc), SUMMON_TYPE_SYNCHRO as u64, Some(tp))
}
// function s.synop(e,tg,ntg,sg,lv,sc,tp)
fn synop(d: &mut Duel, a: &Args) -> Value {
    let e = a.e(0);
    let tg = a.g(1);
    let ntg = a.g(2);
    let tp = a.p(6);
    let sc = a.c(5);
    // return e:GetHandlerPlayer()==1-tp or tg:IsExists(Card.IsSetCard,1,nil,SET_GENEX,sc,SUMMON_TYPE_SYNCHRO,tp)
    //     or ntg:IsExists(s.gtfilter,1,nil,sc,tp)
    let cond = effect::get_handler_player(d, e) == 1 - tp
        || group::is_exists(
            d,
            &tg,
            &|d, c| card::is_set_card_ctx(d, c, &[SET_GENEX as u16], Some(sc), SUMMON_TYPE_SYNCHRO as u64, Some(tp)),
            1,
            NO_EX,
        )
        || group::is_exists(d, &ntg, &|d, c| gtfilter(d, c, sc, tp), 1, NO_EX);
    Value::Bool(cond)
}
