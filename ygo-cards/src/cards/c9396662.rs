//! Mirror Mage of the Ice Barrier
use super::prelude::*;

pub const ID: u32 = 9396662;
/// `TOKEN_ICE_BARRIER`
const TOKEN_ICE_BARRIER: u32 = 44308318;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[9396662, 44308318], listed_series: &[SET_ICE_BARRIER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon up to 3 "Ice Barrier Tokens"
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN + CATEGORY_LVCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(tkcost)));
    effect::set_target(d, e1, Some(cost_target(tktg)));
    effect::set_operation(d, e1, Some(operation(tkop)));
    card::register_effect(d, c, e1, false);
    // Add 1 "Ice Barrier" card to the hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SEARCH + CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}
/// `s.tkcostfilter(c,tp)`
fn tkcostfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_type(d, c, TYPE_EFFECT) && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
/// `s.tkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tkcost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        // return Duel.CheckReleaseGroupCost(tp,s.tkcostfilter,1,false,nil,c,tp)
        return aux::check_release_group_cost(d, a.tp, Some(&|d, x| tkcostfilter(d, x, a.tp)), 1, 1, false, None, c.into());
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.tkcostfilter,1,1,false,nil,c,tp)
    let g = aux::select_release_group_cost(d, a.tp, Some(&|d, x| tkcostfilter(d, x, a.tp)), 1, 1, false, None, c.into());
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `TOKEN_ICE_BARRIER` as a monster for `Duel.IsPlayerCanSpecialSummonMonster`
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_ICE_BARRIER as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(1),
        race: Some(RACE_AQUA),
        attribute: Some(ATTRIBUTE_WATER),
    }
}
/// `s.tktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tktg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // return e:GetHandler():HasLevel() and Duel.IsPlayerCanSpecialSummonMonster(...)
        let c = effect::get_handler(d, a.e).nil();
        return card::has_level(d, c)
            && duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_ICE_BARRIER, token_data(), Some(POS_FACEUP as u8), None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    true
}
/// `s.tkop(e,tp,eg,ep,ev,re,r,rp)`
fn tkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // Cannot Special Summon from the Extra Deck, except WATER Synchro Monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 2));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(
        d,
        e1,
        Some(effect_card_fn(|d, _e, x| {
            !(card::is_type(d, x, TYPE_SYNCHRO) && card::is_attribute(d, x, ATTRIBUTE_WATER)) && card::is_location(d, x, LOCATION_EXTRA)
        })),
    );
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // Clock Lizard check
    // aux.addTempLizardCheck(c,tp,function(_,c) return not c:IsOriginalType(TYPE_SYNCHRO) or not c:IsOriginalAttribute(ATTRIBUTE_WATER) end)
    aux::add_temp_lizard_check(
        d,
        c,
        a.tp,
        Some(effect_card_fn(|d, _e, x| !card::is_original_type(d, x, TYPE_SYNCHRO) || !card::is_original_attribute(d, x, ATTRIBUTE_WATER))),
        None,
        None,
        None,
        None,
    );
    // local ft=math.min(3,Duel.GetLocationCount(tp,LOCATION_MZONE))
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0.min(3);
    // if ft==0 or not Duel.IsPlayerCanSpecialSummonMonster(...) then return end
    if ft == 0 || !duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_ICE_BARRIER, token_data(), Some(POS_FACEUP as u8), None, 0) {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // local ct=Duel.AnnounceNumberRange(tp,1,ft)
    let ct = duel::announce_number_range(d, a.tp, Some(1), Some(ft as u32), &[]).nil().0;
    // for i=1,ct do
    for _ in 0..ct {
        // local token=Duel.CreateToken(tp,TOKEN_ICE_BARRIER)
        let token = duel::create_token(d, a.tp, TOKEN_ICE_BARRIER).nil();
        // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
    // ct=Duel.SpecialSummonComplete()
    let ct = duel::special_summon_complete(d);
    // if ct and ct>0 and c:IsFaceup() and c:IsRelateToEffect(e) then c:UpdateLevel(ct) end
    if ct > 0 && card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        card::update_level(d, c, ct as i64, None, None);
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ICE_BARRIER as u16])
        && card::is_able_to_hand(d, c, None)
        && (card::is_location(d, c, LOCATION_DECK) || card::is_faceup(d, c))
        && !card::is_code(d, c, &[ID])
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_REMOVED) as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK | LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
