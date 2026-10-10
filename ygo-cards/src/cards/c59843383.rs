//! Shiranui Sunsaga
use super::prelude::*;

pub const ID: u32 = 59843383;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SHIRANUI as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetSPSummonOnce(id)
    card::set_spsummon_once(d, c, ID);
    // synchro summon
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_ZOMBIE),1,1,Synchro.NonTunerEx(Card.IsRace,RACE_ZOMBIE),1,99)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_ZOMBIE, sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner_ex(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_ZOMBIE, sc, st, Some(tp)))),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // to deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetTarget(s.tdtg)
    effect::set_target(d, e1, Some(cost_target(tdtg)));
    // e1:SetOperation(s.tdop)
    effect::set_operation(d, e1, Some(operation(tdop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy replace
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTarget(s.reptg)
    effect::set_target(d, e2, Some(cost_target(reptg)));
    // e2:SetValue(s.repval)
    effect::set_value_fn(d, e2, lua_fn(repval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsLocation(LOCATION_GRAVE) or c:IsFaceup()) and c:IsType(TYPE_SYNCHRO) and c:IsRace(RACE_ZOMBIE) and c:IsAbleToDeck()
    (card::is_location(d, c, LOCATION_GRAVE) || card::is_faceup(d, c)) && card::is_type(d, c, TYPE_SYNCHRO) && card::is_race(d, c, RACE_ZOMBIE) && card::is_able_to_deck(d, c)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tdfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tdfilter), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,nil)
    let g = duel::get_matching_group(d, Some(&tdfilter), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local sg=g:Select(tp,1,#g,nil)
    let sg = group::select(d, &g, tp, 1, g.len() as u16, false, NO_EX).nil();
    // local ct=Duel.SendtoDeck(sg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
    let ct = duel::send_to_deck(d, sg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    // local dg=Duel.GetMatchingGroup(nil,tp,0,LOCATION_ONFIELD,nil)
    let dg = duel::get_matching_group(d, None, tp, 0, LOCATION_ONFIELD, NO_EX);
    // if ct>0 and #dg>=ct and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if ct > 0 && dg.len() as i32 >= ct && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local sg2=dg:Select(tp,ct,ct,nil)
        let sg2 = group::select(d, &dg, tp, ct as u16, ct as u16, false, NO_EX).nil();
        // Duel.Destroy(sg2,REASON_EFFECT)
        duel::destroy(d, sg2, REASON_EFFECT, None, None);
    }
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsControler(tp) and c:IsLocation(LOCATION_MZONE) and c:IsRace(RACE_ZOMBIE)
    //     and not c:IsReason(REASON_REPLACE) and c:IsReason(REASON_BATTLE|REASON_EFFECT)
    card::is_faceup(d, c)
        && card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_race(d, c, RACE_ZOMBIE)
        && !card::is_reason(d, c, REASON_REPLACE)
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SHIRANUI) and c:IsMonster() and c:IsAbleToRemove() and aux.SpElimFilter(c,true)
    card::is_set_card(d, c, &[SET_SHIRANUI as u16]) && card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return eg:IsExists(s.repfilter,1,nil,tp)
    //     and Duel.IsExistingMatchingCard(s.rmfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return group::is_exists(d, &eg, &|d, c| repfilter(d, c, tp), 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&rmfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // if Duel.SelectEffectYesNo(tp,e:GetHandler(),96) then
    let h = effect::get_handler(d, a.e).nil();
    if duel::select_effect_yes_no(d, tp, h, Some(96)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.rmfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&rmfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // Duel.Remove(g,POS_FACEUP,REASON_EFFECT|REASON_REPLACE)
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT | REASON_REPLACE, None, None);
        // return true
        return true;
    }
    // return false
    false
}
/// `s.repval(e,c)`
fn repval(d: &mut Duel, a: Args) -> bool {
    // return s.repfilter(c,e:GetHandlerPlayer())
    let c = a.c(1);
    let tp = effect::get_handler_player(d, a.e(0));
    repfilter(d, c, tp)
}
