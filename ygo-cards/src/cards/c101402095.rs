//! Angelechy Endgame Problem
use super::prelude::*;

pub const ID: u32 = 101402095;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ANGELECHY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // ● 1+: Once per turn, during your opponent's Standby Phase: You can draw 1 card
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_STANDBY);
    effect::set_range(d, e1, LOCATION_FZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(|d, a| duel::is_turn_player(d, 1 - a.tp) && angelcountcondition(d, a.e, 1))));
    effect::set_target(d, e1, Some(cost_target(drwtg)));
    effect::set_operation(d, e1, Some(operation(drwop)));
    card::register_effect(d, c, e1, false);
    // ● 2+: When your opponent would Special Summon a monster(s), you choose the zone(s) to place it
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e2, EFFECT_OPPO_CHOOSES_SPSUMMON_ZONE);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_target_range(d, e2, 0, 1);
    effect::set_condition(d, e2, Some(condition(|d, a| angelcountcondition(d, a.e, 2))));
    effect::set_operation(d, e2, Some(lua_fn(oppo_zone_op)));
    card::register_effect(d, c, e2, false);
    // ● 3+: Once per turn, you can activate 1 "Angelechy" Trap Card the turn it was Set
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_property(d, e3, EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_code(d, e3, EFFECT_TRAP_ACT_IN_SET_TURN);
    effect::set_range(d, e3, LOCATION_FZONE);
    effect::set_target_range(d, e3, LOCATION_SZONE, 0);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(|d, a| angelcountcondition(d, a.e, 3))));
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_ANGELECHY as u16]) && card::is_trap(d, c))));
    card::register_effect(d, c, e3, false);
    // ● 4+: Once per turn, when your opponent Special Summons a monster(s) (except during the Damage Step): You can banish that monster(s)
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 2));
    effect::set_category(d, e4, CATEGORY_REMOVE);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e4, LOCATION_FZONE);
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    effect::set_condition(d, e4, Some(condition(|d, a| a.ep == 1 - a.tp && angelcountcondition(d, a.e, 4))));
    effect::set_target(d, e4, Some(cost_target(banishtg)));
    effect::set_operation(d, e4, Some(operation(banishop)));
    card::register_effect(d, c, e4, false);
    // ● 5: Once per turn: You can Special Summon 1 "Angelechy" monster from your Extra Deck
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 3));
    effect::set_category(d, e5, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e5, LOCATION_FZONE);
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    effect::set_condition(d, e5, Some(condition(|d, a| angelcountcondition(d, a.e, 5))));
    effect::set_target(d, e5, Some(cost_target(sptg)));
    effect::set_operation(d, e5, Some(operation(spop)));
    card::register_effect(d, c, e5, false);
}
// s.angelechyfilter(c)
fn angelechyfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ANGELECHY) and c:IsMonsterCard() and c:IsFaceup()
    card::is_set_card(d, c, &[SET_ANGELECHY as u16]) && card::is_monster_card(d, c) && card::is_faceup(d, c)
}
// s.angelcountcondition(required) returned function(e), called with e
fn angelcountcondition(d: &mut Duel, e: EffectId, required: usize) -> bool {
    // return Duel.GetMatchingGroupCount(s.angelechyfilter,e:GetHandlerPlayer(),LOCATION_STZONE,0,nil)>=required
    let p = effect::get_handler_player(d, e);
    duel::get_matching_group_count(d, Some(&angelechyfilter), p, LOCATION_STZONE, 0, NO_EX) >= required
}
// s.drwtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn drwtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 1);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
// s.drwop(e,tp,eg,ep,ev,re,r,rp)
fn drwop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
// function(e,tp,eg,ep,ev,re,r,rp,c,...) of EFFECT_OPPO_CHOOSES_SPSUMMON_ZONE: (e, c, tp, r)
fn oppo_zone_op(_d: &mut Duel, a: Args) -> bool {
    // return (r&LOCATION_REASON_SPSUMMON)==LOCATION_REASON_SPSUMMON
    let r = a.int(3) as u32;
    (r & LOCATION_REASON_SPSUMMON) == LOCATION_REASON_SPSUMMON
}
// s.banishtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn banishtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let eg = a.eg.clone().nil();
    // local g=eg:Filter(Card.IsAbleToRemove,nil)
    let g = group::filter(d, &eg, &|d, c| card::is_able_to_remove(d, c, None, None, None), NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // for sc in eg:Iter() do sc:CreateEffectRelation(e) end
    for sc in group::iter(d, &eg) {
        card::create_effect_relation(d, sc, e);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, tp, 0);
    true
}
// s.banishop(e,tp,eg,ep,ev,re,r,rp)
fn banishop(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    // local g=eg:Filter(Card.IsRelateToEffect,nil,e)
    let mut g = group::filter(d, &eg, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    // if #g>0 then Duel.Remove(g,POS_FACEUP,REASON_EFFECT) end
    if !g.is_empty() {
        duel::remove(d, std::mem::take(&mut g), POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
// s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ANGELECHY) and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_ANGELECHY as u16])
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
// s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP) end
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
