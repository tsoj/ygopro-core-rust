//! Numeron Wall
use super::prelude::*;

pub const ID: u32 = 42352091;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID, CARD_NUMERON_NETWORK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate "Numeron Network"
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_cost(d, e1, Some(cost::self_to_grave()));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Special Summon and end Battle Phase
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_BATTLE_DAMAGE);
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_condition(d, e2, Some(condition(spcond)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return not (c:IsFaceup() and c:IsCode(id))
    !(card::is_faceup(d, c) && card::is_code(d, c, &[ID]))
}
/// `s.field(c,tp)`
fn field(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsCode(CARD_NUMERON_NETWORK) and c:GetActivateEffect():IsActivatable(tp,true,true) and not c:IsForbidden()
    card::is_code(d, c, &[CARD_NUMERON_NETWORK]) && activatable(d, c, tp) && !card::is_forbidden(d, c)
}
/// `c:GetActivateEffect():IsActivatable(tp,true,true)`
fn activatable(d: &mut Duel, c: CardId, tp: u8) -> bool {
    let te = card::get_activate_effect(d, c).first().copied().nil();
    effect::is_activatable(d, te, tp, true, true)
}
/// `s.condition(e,tp,...)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,LOCATION_ONFIELD,0)==0 or not Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_ONFIELD,0,1,nil)
    duel::get_field_group_count(d, a.tp, LOCATION_ONFIELD, 0) == 0
        || !duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.field,tp,LOCATION_DECK|LOCATION_HAND,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| field(d, c, a.tp)), a.tp, LOCATION_DECK | LOCATION_HAND, 0, 1, NO_EX);
    }
    // if not Duel.CheckPhaseActivity() then Duel.RegisterFlagEffect(tp,CARD_MAGICAL_MIDBREAKER,RESET_CHAIN,0,1) end
    if !duel::check_phase_activity(d) {
        duel::register_flag_effect(d, a.tp, CARD_MAGICAL_MIDBREAKER, RESET_CHAIN, 0, 1, 0);
    }
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.field,tp,LOCATION_DECK|LOCATION_HAND,0,1,1,nil,tp):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| field(d, c, a.tp)), a.tp, LOCATION_DECK | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // return Duel.ActivateFieldSpell(tc,e,tp,eg,ep,ev,re,r,rp)
    duel::activate_field_spell(d, tc, a, None);
}
/// `s.spcond(e,tp,...)`
fn spcond(_d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==tp
    a.ep == a.tp
}
/// `s.sptg(e,tp,...,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        // Duel.SkipPhase(Duel.GetTurnPlayer(),PHASE_BATTLE,RESET_PHASE|PHASE_BATTLE_STEP,1)
        let tp = duel::get_turn_player(d);
        duel::skip_phase(d, tp, PHASE_BATTLE, RESET_PHASE | PHASE_BATTLE_STEP, 1, 0);
    }
}
