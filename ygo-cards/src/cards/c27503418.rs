//! King's Synchro
use super::prelude::*;

pub const ID: u32 = 27503418;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=Duel.GetAttackTarget()
    let tc = duel::get_attack_target(d);
    // return tc and tc:IsFaceup() and tc:IsControler(tp) and tc:IsType(TYPE_SYNCHRO)
    match tc {
        Some(tc) => card::is_faceup(d, tc) && card::is_controler(d, tc, a.tp) && card::is_type(d, tc, TYPE_SYNCHRO),
        None => false,
    }
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,2,tp,LOCATION_MZONE|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 2, tp, (LOCATION_MZONE | LOCATION_GRAVE) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
fn filter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId, lv: i64) -> bool {
    // local rlv=c:GetLevel()-lv
    let rlv = card::get_level(d, c) as i64 - lv;
    // return rlv>0 and c:IsType(TYPE_SYNCHRO) and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_SYNCHRO,tp,false,false)
    //   and Duel.GetLocationCountFromEx(tp,tp,tc,c)>0
    //   and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,c,tp,rlv)
    rlv > 0
        && card::is_type(d, c, TYPE_SYNCHRO)
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_SYNCHRO, tp, false, false, None, None, None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), tc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, x| filter2(d, x, tp, rlv)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, c.into())
}
fn filter2(d: &mut Duel, c: CardId, _tp: u8, lv: i64) -> bool {
    // local rlv=lv-c:GetLevel()
    let rlv = lv - card::get_level(d, c) as i64;
    // return rlv==0 and c:IsType(TYPE_TUNER) and c:IsAbleToRemove() and aux.SpElimFilter(c,true)
    rlv == 0
        && card::is_type(d, c, TYPE_TUNER)
        && card::is_able_to_remove(d, c, None, None, None)
        && aux::sp_elim_filter(d, c, true, false)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetAttackTarget()
    let tc = duel::get_attack_target(d);
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.CreateGroup(),tp,nil,nil,REASON_SYNCHRO)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::new()), tp, None, None, REASON_SYNCHRO);
    // if Duel.NegateAttack() and Duel.IsExistingMatchingCard(s.filter1,tp,LOCATION_EXTRA,0,1,nil,e,tp,tc,tc:GetLevel())
    //   and tc:IsAbleToRemove() and #pg<=0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if duel::negate_attack(d) {
        let tc_lv = card::get_level(d, tc.nil()) as i64;
        if duel::is_existing_matching_card(d, Some(&|d, x| filter1(d, x, e, tp, tc.nil(), tc_lv)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
            && card::is_able_to_remove(d, tc.nil(), None, None, None)
            && pg.is_empty()
            && duel::select_yes_no(d, tp, aux::stringid(ID, 0))
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local g1=Duel.SelectMatchingCard(tp,s.filter1,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,tc,tc:GetLevel())
            let g1 = duel::select_matching_card(d, tp, Some(&|d, x| filter1(d, x, e, tp, tc.nil(), tc_lv)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
            // local lv=g1:GetFirst():GetLevel()-tc:GetLevel()
            let first = group::get_first(d, &g1).nil();
            let lv = card::get_level(d, first) as i64 - card::get_level(d, tc.nil()) as i64;
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
            // local g2=Duel.SelectMatchingCard(tp,s.filter2,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil,tp,lv)
            let mut g2 = duel::select_matching_card(d, tp, Some(&|d, x| filter2(d, x, tp, lv)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
            // g2:AddCard(tc)
            group::add_card(&mut g2, &CardSet::from_card(tc.nil()));
            // if Duel.Remove(g2,POS_FACEUP,REASON_EFFECT)==2 then
            if duel::remove(d, g2, POS_FACEUP as u8, REASON_EFFECT, None, None) == 2 {
                // Duel.SpecialSummon(g1,SUMMON_TYPE_SYNCHRO,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, g1.clone(), SUMMON_TYPE_SYNCHRO, tp, tp, false, false, POS_FACEUP as u8, None);
                // g1:GetFirst():CompleteProcedure()
                let first = group::get_first(d, &g1).nil();
                card::complete_procedure(d, first);
            }
        }
    }
}
