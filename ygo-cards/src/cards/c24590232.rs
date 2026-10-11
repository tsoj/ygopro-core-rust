//! King's Consonance
use super::prelude::*;

pub const ID: u32 = 24590232;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_REMOVE);
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
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:GetFirst():IsControler(1-tp) and Duel.GetAttackTarget()==nil
    let eg = a.eg.clone().nil();
    let first = group::get_first(d, &eg).nil();
    card::is_controler(d, first, 1 - a.tp) && duel::get_attack_target(d).is_none()
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,2,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 2, tp, LOCATION_GRAVE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
// function s.filter1(c,e,tp)
fn filter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsType(TYPE_SYNCHRO) and c:GetLevel()<9
    //   and (Duel.GetLocationCountFromEx(tp,tp,nil,c)>0 or Duel.IsPlayerAffectedByEffect(e:GetHandlerPlayer(),CARD_SPIRIT_ELIMINATION))
    //   and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_SYNCHRO,tp,false,false)
    //   and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil,e,tp,c)
    if !card::is_type(d, c, TYPE_SYNCHRO) || card::get_level(d, c) >= 9 {
        return false;
    }
    let zone_ok = duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0;
    let hp = effect::get_handler_player(d, e);
    if !(zone_ok || duel::is_player_affected_by_effect(d, hp, CARD_SPIRIT_ELIMINATION).is_some()) {
        return false;
    }
    card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_SYNCHRO, tp, false, false, None, None, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| filter2(d, x, e, tp, c)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX)
}
// function s.filter2(c,e,tp,sc)
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, sc: CardId) -> bool {
    // local rg=Duel.GetMatchingGroup(s.filter3,tp,LOCATION_MZONE|LOCATION_GRAVE,0,c)
    let rg = duel::get_matching_group(d, Some(&filter3), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, c.into());
    // return c:IsType(TYPE_TUNER) and c:IsAbleToRemove() and aux.SpElimFilter(c,true)
    //   and aux.SelectUnselectGroup(rg,e,tp,nil,nil,s.rescon(c,sc),0)
    if !(card::is_type(d, c, TYPE_TUNER) && card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, true, false)) {
        return false;
    }
    let rc = rescon(c, sc);
    matches!(aux::select_unselect_group(d, &rg, Some(e), tp, None, None, Some(&rc), 0, tp, None, None, None, false), Err(true))
}
// function s.rescon(tuner,scard)
// return function(sg,e,tp,mg)
//   sg:AddCard(tuner)
//   local res=Duel.GetLocationCountFromEx(tp,tp,sg,scard)>0 and sg:CheckWithSumEqual(Card.GetLevel,scard:GetLevel(),#sg,#sg)
//   sg:RemoveCard(tuner)
//   return res
// end
fn rescon(tuner: CardId, scard: CardId) -> impl Fn(&mut Duel, &CardSet, Option<EffectId>, u8, &CardSet, Option<CardId>) -> (bool, bool) {
    move |d, sg, _e, tp, _mg, _c| {
        let mut sg = sg.clone();
        group::add_card(&mut sg, &CardSet::from_card(tuner));
        let zone_ok = duel::get_location_count_from_ex(d, tp, Some(tp), (&sg).into(), duel::ExtraCard::Card(scard), None).0 > 0;
        let res = zone_ok && {
            let lv = card::get_level(d, scard);
            let n = sg.len() as i32;
            group::check_with_sum_equal(d, &sg, &|d, c| card::get_level(d, c) as i64, lv, n, n).0
        };
        (res, false)
    }
}
// function s.filter3(c)
fn filter3(d: &mut Duel, c: CardId) -> bool {
    // return c:HasLevel() and not c:IsType(TYPE_TUNER) and c:IsAbleToRemove() and aux.SpElimFilter(c,true)
    card::has_level(d, c) && !card::is_type(d, c, TYPE_TUNER) && card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, true, false)
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.CreateGroup(),tp,nil,nil,REASON_SYNCHRO)
    let pg = aux::get_must_be_material_group(d, tp, &CardSet::new(), tp, None, &CardSet::new(), REASON_SYNCHRO);
    // if Duel.NegateAttack() and #pg<=0 and Duel.IsExistingMatchingCard(s.filter1,tp,LOCATION_EXTRA,0,1,nil,e,tp)
    //   and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if !duel::negate_attack(d) || pg.len() > 0 {
        return;
    }
    if !duel::is_existing_matching_card(d, Some(&|d, c| filter1(d, c, e, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX) {
        return;
    }
    if !duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        return;
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g1=Duel.SelectMatchingCard(tp,s.filter1,tp,LOCATION_EXTRA,0,1,1,nil,e,tp)
    let g1 = duel::select_matching_card(d, tp, Some(&|d, c| filter1(d, c, e, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // local sc=g1:GetFirst()
    let sc = group::get_first(d, &g1).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g2=Duel.SelectMatchingCard(tp,s.filter2,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil,e,tp,sc)
    let g2 = duel::select_matching_card(d, tp, Some(&|d, c| filter2(d, c, e, tp, sc)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // local tuner=g2:GetFirst()
    let tuner = group::get_first(d, &g2).nil();
    // local rg=Duel.GetMatchingGroup(s.filter3,tp,LOCATION_MZONE|LOCATION_GRAVE,0,tuner)
    let rg = duel::get_matching_group(d, Some(&filter3), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, tuner.into());
    // local sg=aux.SelectUnselectGroup(rg,e,tp,nil,nil,s.rescon(tuner,sc),1,tp,HINTMSG_REMOVE,s.rescon(tuner,sc))
    let rc1 = rescon(tuner, sc);
    let rc2 = rescon(tuner, sc);
    let mut sg = aux::select_unselect_group(d, &rg, Some(e), tp, None, None, Some(&rc1), 1, tp, Some(HINTMSG_REMOVE as u64), Some(&rc2), None, false)
        .unwrap_or_default();
    // sg:AddCard(tuner)
    group::add_card(&mut sg, &CardSet::from_card(tuner));
    // Duel.Remove(sg,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, sg, POS_FACEUP as u8, REASON_EFFECT, None, None);
    // Duel.SpecialSummon(sc,SUMMON_TYPE_SYNCHRO,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, sc, SUMMON_TYPE_SYNCHRO, tp, tp, false, false, POS_FACEUP as u8, None);
    // sc:CompleteProcedure()
    card::complete_procedure(d, sc);
}
