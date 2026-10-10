//! Number C1: Numeron Chaos Gate Sunya
use super::prelude::*;

pub const ID: u32 = 79747096;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[15232745, CARD_NUMERON_NETWORK], xyz_number: Some(1), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    xyz::add_procedure(
        d,
        c,
        None,
        Some(2),
        4,
        Some(xyz::alter_filter(|d, c, tp, lc| ovfilter(d, c, tp, lc))),
        Some(aux::stringid(ID, 0)),
        None,
        None,
        false,
        None,
    );
    card::enable_revive_limit(d, c);
    // Banish
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 1));
    effect::set_category(d, e1, CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e1, Some(condition(rmcon)));
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    effect::set_operation(d, e1, Some(operation(rmop)));
    card::register_effect(d, c, e1, false);
    // Special summon
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e2, EVENT_REMOVE);
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
}
/// `s.ovfilter(c,tp,lc)`
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, lc: CardId) -> bool {
    // return c:IsFaceup() and c:IsSummonCode(lc,SUMMON_TYPE_XYZ,tp,15232745)
    card::is_faceup(d, c) && card::is_summon_code(d, c, Some(lc), SUMMON_TYPE_XYZ as u64, tp, &[15232745])
}
/// `s.rmcon(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsXyzSummoned()
    card::is_xyz_summoned(d, effect::get_handler(d, a.e).nil())
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetFieldGroup(tp,LOCATION_MZONE,LOCATION_MZONE)
    let g = duel::get_field_group(d, a.tp, LOCATION_MZONE, LOCATION_MZONE);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, 0, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,LOCATION_MZONE,LOCATION_MZONE)
    let g = duel::get_field_group(d, a.tp, LOCATION_MZONE, LOCATION_MZONE);
    // if #g>0 then Duel.Remove(g,POS_FACEUP,REASON_EFFECT) end
    if !g.is_empty() {
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
/// `s.op(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local res=(Duel.IsPhase(PHASE_STANDBY) and Duel.IsTurnPlayer(tp)) and 2 or 1
    let res: u16 = if duel::is_phase(d, PHASE_STANDBY) && duel::is_turn_player(d, a.tp) { 2 } else { 1 };
    // local turn_asc=(Duel.GetCurrentPhase()<PHASE_STANDBY and Duel.IsTurnPlayer(tp)) and 0 or (Duel.IsTurnPlayer(tp)) and 2 or 1
    let turn_asc: i64 = if duel::get_current_phase(d) < PHASE_STANDBY && duel::is_turn_player(d, a.tp) {
        0
    } else if duel::is_turn_player(d, a.tp) {
        2
    } else {
        1
    };
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_description(d, e1, aux::stringid(ID, 2));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    effect::set_range(d, e1, LOCATION_REMOVED);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    // e1:SetLabel(turn_asc+Duel.GetTurnCount())
    effect::set_label(d, e1, &[turn_asc + duel::get_turn_count(d, None) as i64]);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_STANDBY|RESET_SELF_TURN,res)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, res);
    // e:GetHandler():RegisterEffect(e1)
    card::register_effect(d, h, e1, false);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetTurnCount()==e:GetLabel() and Duel.IsTurnPlayer(tp)
    duel::get_turn_count(d, None) as i64 == effect::get_label(d, a.e) && duel::is_turn_player(d, a.tp)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local atk=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsType,TYPE_XYZ),tp,LOCATION_REMOVED,LOCATION_REMOVED,c):GetSum(Card.GetAttack)
    let rg = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x) && card::is_type(d, x, TYPE_XYZ)), a.tp, LOCATION_REMOVED, LOCATION_REMOVED, c.into());
    let atk = group::get_sum(d, &rg, &|d, x| card::get_attack(d, x) as i64);
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0
    //   and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_NUMERON_NETWORK),tp,LOCATION_ONFIELD,0,1,nil) and atk>0 then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0
        && duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x) && card::is_code(d, x, &[CARD_NUMERON_NETWORK])), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        && atk > 0
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Damage(1-tp,atk,REASON_EFFECT)
        duel::damage(d, 1 - a.tp, atk, REASON_EFFECT, false, None);
    }
}
