//! Desrook Archfiend
use super::prelude::*;

pub const ID: u32 = 72192100;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[35975813], roll_dice: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Once per turn, during your Standby Phase, you must pay 500 LP (this is not optional), or this card is destroyed
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(turn_con)));
    effect::set_operation(d, e1, Some(operation(mtop)));
    card::register_effect(d, c, e1, false);
    // Roll a six-sided die when resolving an opponent's card effect that targets this card
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_operation(d, e2, Some(operation(disop)));
    card::register_effect(d, c, e2, false);
    // Special Summon 1 "Terrorking Archfiend" from your GY
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    effect::set_range(d, e3, LOCATION_HAND);
    effect::set_cost(d, e3, Some(cost::self_to_grave()));
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
}
fn turn_con(d: &mut Duel, a: &EffArgs) -> bool {
    // function(e,tp) return Duel.IsTurnPlayer(tp) end
    duel::is_turn_player(d, a.tp)
}
fn mtop(d: &mut Duel, a: &EffArgs) {
    // if Duel.CheckLPCost(tp,500) then
    if duel::check_lp_cost(d, a.tp, 500) {
        // Duel.PayLPCost(tp,500)
        duel::pay_lp_cost(d, a.tp, 500);
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
    // if dc~=3 then return end
    if dc != 3 {
        return;
    }
    // if Duel.NegateEffect(ev) and rc:IsRelateToEffect(re) then
    if duel::negate_effect(d, a.ev as u8) && card::is_relate_to_effect(d, rc, re) {
        // Duel.Destroy(rc,REASON_EFFECT)
        duel::destroy(d, rc, REASON_EFFECT, None, None);
    }
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // c:IsCode(35975813) and c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_ONFIELD)
    //     and c:IsReason(REASON_DESTROY) and c:IsCanBeEffectTarget(e) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[35975813])
        && card::is_previous_controler(d, c, tp)
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
        && card::is_reason(d, c, REASON_DESTROY)
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // if chkc then return eg:IsContains(chkc) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        let eg = a.eg.clone().nil();
        return group::is_contains(&eg, chkc) && spfilter(d, chkc, e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and eg:IsExists(s.spfilter,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && {
            let eg = a.eg.clone().nil();
            group::is_exists(d, &eg, &|d, c| spfilter(d, c, e, tp), 1, NO_EX)
        };
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=eg:FilterSelect(tp,s.spfilter,1,1,nil,e,tp)
    let eg = a.eg.clone().nil();
    let g = group::filter_select(d, &eg, tp, &|d, c| spfilter(d, c, e, tp), 1, 1, false, NO_EX).nil();
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
