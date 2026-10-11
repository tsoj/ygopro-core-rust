//! Vampire Grimson
use super::prelude::*;

pub const ID: u32 = 33438666;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destruction replacement
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EFFECT_DESTROY_REPLACE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target(d, e1, Some(cost_target(reptg)));
    effect::set_value_fn(d, e1, effect_card_fn(repval));
    card::register_effect(d, c, e1, false);
    // Register when destroying a monster
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
    effect::set_operation(d, e2, Some(operation(regop)));
    card::register_effect(d, c, e2, false);
    // Special summon the monsters this card destroyed by battle
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e3, EVENT_PHASE | PHASE_BATTLE);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(spcon)));
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsLocation(LOCATION_MZONE)
    //     and (c:IsReason(REASON_BATTLE) or (c:IsReason(REASON_EFFECT) and c:GetReasonPlayer()==1-tp))
    card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_MZONE)
        && (card::is_reason(d, c, REASON_BATTLE) || (card::is_reason(d, c, REASON_EFFECT) && card::get_reason_player(d, c) == 1 - tp))
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=eg:FilterCount(s.repfilter,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    let ct = group::filter_count(d, &eg, &|d, c| repfilter(d, c, tp), NO_EX) as i64;
    // if chk==0 then return ct>0 and Duel.CheckLPCost(tp,1000*ct) end
    if a.chk == Some(0) {
        return ct > 0 && duel::check_lp_cost(d, tp, (1000 * ct) as u32);
    }
    // if Duel.SelectEffectYesNo(tp,e:GetHandler(),96) then
    let h = effect::get_handler(d, a.e).nil();
    if duel::select_effect_yes_no(d, tp, h, Some(96)) {
        // Duel.PayLPCost(tp,1000*ct)
        duel::pay_lp_cost(d, tp, (1000 * ct) as u32);
        // return true
        true
    } else {
        // else return false end
        false
    }
}
/// `s.repval(e,c)`
fn repval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return s.repfilter(c,e:GetHandlerPlayer())
    let p = effect::get_handler_player(d, e);
    repfilter(d, c, p)
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE,0,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 0, 1, 0, 0);
}
/// `s.spcon` (`function(e) return e:GetHandler():HasFlagEffect(id) end`)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::has_flag_effect(d, h, ID, None)
}
/// `s.spfilter(c,e,tp,rc,tid)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rc: Option<CardId>, tid: i32) -> bool {
    // return c:IsReason(REASON_BATTLE) and c:GetReasonCard()==rc and c:GetTurnID()==tid
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_reason(d, c, REASON_BATTLE)
        && card::get_reason_card(d, c) == rc
        && card::get_turn_id(d, c) as i32 == tid
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,nil,e,tp,e:GetHandler(),Duel.GetTurnCount()) end
    let h = effect::get_handler(d, e);
    let tid = duel::get_turn_count(d, None);
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, h, tid)), tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_GRAVE,LOCATION_GRAVE,nil,e,tp,e:GetHandler(),Duel.GetTurnCount())
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp, h, tid)), tp, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft==0 then return end
    if ft == 0 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // local g=nil
    // local tg=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,LOCATION_GRAVE,nil,e,tp,e:GetHandler(),Duel.GetTurnCount())
    let h = effect::get_handler(d, e);
    let tid = duel::get_turn_count(d, None);
    let tg = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, e, tp, h, tid))), tp, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX);
    let g = if (tg.len() as i32) > ft {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // g=tg:Select(tp,ft,ft,nil)
        group::select(d, &tg, tp, ft as u16, ft as u16, false, NO_EX).nil()
    } else {
        // g=tg
        tg
    };
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
