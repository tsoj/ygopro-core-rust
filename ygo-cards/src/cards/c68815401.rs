//! Wild Fire
use super::prelude::*;

pub const ID: u32 = 68815401;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[68815402], listed_series: &[SET_BLAZE_ACCELERATOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER));
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `aux.FaceupFilter(Card.IsSetCard,SET_BLAZE_ACCELERATOR)`
fn blaze_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_BLAZE_ACCELERATOR as u16])
}
/// The token data of `id+1`: `TYPES_TOKEN,1000,1000,3,RACE_PYRO,ATTRIBUTE_FIRE`.
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(1000),
        defense: Some(1000),
        level: Some(3),
        race: Some(RACE_PYRO),
        attribute: Some(ATTRIBUTE_FIRE),
    }
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::check_lp_cost(d, a.tp, 500) && duel::get_activity_count(d, a.tp, &[ACTIVITY_ATTACK])[0] == 0;
    }
    // Duel.PayLPCost(tp,500)
    duel::pay_lp_cost(d, a.tp, 500);
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&blaze_faceup), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, None, tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
            && duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), None, None, 0);
    }
    // local dg1=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_BLAZE_ACCELERATOR),tp,LOCATION_ONFIELD,0,nil)
    let mut dg1 = duel::get_matching_group(d, Some(&blaze_faceup), tp, LOCATION_ONFIELD, 0, NO_EX);
    // local dg2=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let dg2 = duel::get_matching_group(d, None, tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // dg1:Merge(dg2)
    group::add_card(&mut dg1, &dg2);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg1,#dg1,0,0)
    let n = dg1.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg1), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local dg1=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_BLAZE_ACCELERATOR),tp,LOCATION_ONFIELD,0,nil)
    let dg1 = duel::get_matching_group(d, Some(&blaze_faceup), tp, LOCATION_ONFIELD, 0, NO_EX);
    // if Duel.Destroy(dg1,REASON_EFFECT)>0 then
    if duel::destroy(d, dg1, REASON_EFFECT, None, None) > 0 {
        // local dg2=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
        let dg2 = duel::get_matching_group(d, None, tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
        // if Duel.Destroy(dg2,REASON_EFFECT)>0 and Duel.IsPlayerCanSpecialSummonMonster(...) then
        if duel::destroy(d, dg2, REASON_EFFECT, None, None) > 0
            && duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), None, None, 0)
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // local token=Duel.CreateToken(tp,id+1)
            let token = duel::create_token(d, tp, ID + 1).nil();
            // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP_ATTACK)
            duel::special_summon(d, token, 0, tp, tp, false, false, POS_FACEUP_ATTACK as u8, None);
        }
    }
}
