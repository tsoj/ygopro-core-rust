//! Terrorking Archfiend
use super::prelude::*;

pub const ID: u32 = 35975813;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ARCHFIEND], roll_dice: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot be Normal/Flip summoned unless you control an "Archfiend" monster
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CANNOT_SUMMON);
    effect::set_condition(d, e1, Some(condition(excon)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_CANNOT_FLIP_SUMMON);
    card::register_effect(d, c, e2, false);
    // Once per turn, during your Standby Phase, you must pay 800 LP (this is not optional), or this card is destroyed
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e3, EVENT_PHASE | PHASE_STANDBY);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(turn_con)));
    effect::set_operation(d, e3, Some(operation(mtop)));
    card::register_effect(d, c, e3, false);
    // Roll a six-sided die when resolving an opponent's card effect that targets this card
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e4, EVENT_CHAIN_SOLVING);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_operation(d, e4, Some(operation(disop)));
    card::register_effect(d, c, e4, false);
    // Negate the effects of monsters destroyed by battle with this card
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e5, EVENT_BATTLED);
    effect::set_range(d, e5, LOCATION_MZONE);
    effect::set_operation(d, e5, Some(operation(disop2)));
    card::register_effect(d, c, e5, false);
}
fn excon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_ARCHFIEND),e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    let p = effect::get_handler_player(d, a.e);
    !duel::is_existing_matching_card(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_ARCHFIEND as u16]))),
        p,
        LOCATION_MZONE,
        0,
        1,
        NO_EX,
    )
}
fn turn_con(d: &mut Duel, a: &EffArgs) -> bool {
    // function(e,tp) return Duel.IsTurnPlayer(tp) end
    duel::is_turn_player(d, a.tp)
}
fn mtop(d: &mut Duel, a: &EffArgs) {
    // if Duel.CheckLPCost(tp,800) then
    if duel::check_lp_cost(d, a.tp, 800) {
        // Duel.PayLPCost(tp,800)
        duel::pay_lp_cost(d, a.tp, 800);
    } else {
        // Duel.Destroy(e:GetHandler(),REASON_COST)
        let h = effect::get_handler(d, a.e).nil();
        duel::destroy(d, h, REASON_COST, None, None);
    }
}
fn disop(d: &mut Duel, a: &EffArgs) {
    // if not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return false end
    let re = a.re.nil();
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsRelateToEffect(re) and ep==1-tp) then return end
    if !(card::is_relate_to_effect(d, c, re) && a.ep == 1 - a.tp) {
        return;
    }
    // local tg=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let tg = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned();
    // if not tg or not tg:IsContains(c) or not Duel.IsChainDisablable(ev) then return false end
    if !tg.as_ref().is_some_and(|tg| group::is_contains(tg, c)) || !duel::is_chain_disablable(d, a.ev as u8) {
        return;
    }
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, re).nil();
    // Duel.Hint(HINT_CARD,1-tp,id)
    duel::hint(d, HINT_CARD as u8, 1 - a.tp, ID as u64);
    // local dc=Duel.TossDice(tp,1)
    let dc = duel::toss_dice(d, a.tp, 1, 0)[0];
    // if dc~=2 and dc~=5 then return end
    if dc != 2 && dc != 5 {
        return;
    }
    // if Duel.NegateEffect(ev) and rc:IsRelateToEffect(re) then
    if duel::negate_effect(d, a.ev as u8) && card::is_relate_to_effect(d, rc, re) {
        // Duel.Destroy(rc,REASON_EFFECT)
        duel::destroy(d, rc, REASON_EFFECT, None, None);
    }
}
fn disop2(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=c:GetBattleTarget()
    let tc = card::get_battle_target(d, c);
    // if tc and tc:IsType(TYPE_EFFECT) and tc:IsStatus(STATUS_BATTLE_DESTROYED) then
    if let Some(tc) = tc {
        if card::is_type(d, tc, TYPE_EFFECT) && card::is_status(d, tc, STATUS_BATTLE_DESTROYED) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_DISABLE);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_EXC_GRAVE, 1);
            card::register_effect(d, tc, e1, false);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD_EXC_GRAVE, 1);
            card::register_effect(d, tc, e2, false);
        }
    }
}
