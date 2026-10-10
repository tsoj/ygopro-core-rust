//! Super Robolady
use super::prelude::*;

pub const ID: u32 = 75923050;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[1412158], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion Material
    card::enable_revive_limit(d, c);
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(92421852), fusion::Mat::Code(38916461)]);
    // Increase ATK
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(atkcon)));
    effect::set_value(d, e1, 1000);
    card::register_effect(d, c, e1, false);
    // Special Summon "Super Roboyarou"
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ph=Duel.GetCurrentPhase()
    let ph = duel::get_current_phase(d);
    // if not (ph==PHASE_DAMAGE or ph==PHASE_DAMAGE_CAL) then return false end
    if !(ph == PHASE_DAMAGE || ph == PHASE_DAMAGE_CAL) {
        return false;
    }
    // return Duel.GetAttacker()==e:GetHandler() and not Duel.GetAttackTarget()
    let h = effect::get_handler(d, a.e).nil();
    duel::get_attacker(d) == Some(h) && duel::get_attack_target(d).is_none()
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetTurnID()~=Duel.GetTurnCount()
    let c = effect::get_handler(d, a.e).nil();
    card::get_turn_id(d, c) as i32 != duel::get_turn_count(d, None)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId) -> bool {
    // c:IsCode(1412158) and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[1412158])
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:GetOwner()==tp and c:IsAbleToExtra() and Duel.IsExistingMatchingCard(...) end
    if a.chk == Some(0) {
        return card::get_owner(d, c) == a.tp
            && card::is_able_to_extra(d, c)
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, a.tp, c)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsRelateToEffect(e) and c:IsFaceup()) then return end
    if !(card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c)) {
        return;
    }
    // if Duel.SendtoDeck(c,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 then
    if duel::send_to_deck(d, c, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sg=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,c)
        let sg = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp, c)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        // if #sg==0 then return end
        if sg.is_empty() {
            return;
        }
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
