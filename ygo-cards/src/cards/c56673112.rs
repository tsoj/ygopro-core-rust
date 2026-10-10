//! Imperial Bower
use super::prelude::*;

pub const ID: u32 = 56673112;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_JACK_KNIGHT, CARD_KING_KNIGHT, CARD_QUEEN_KNIGHT],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon 2 face card monsters from deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcond)
    effect::set_condition(d, e1, Some(condition(spcond)));
    // e1:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e1, Some(cost::self_tribute()));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spcond(e,tp,eg,ep,ev,re,r,rp)`
fn spcond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)<=1
    duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0) <= 1
}
/// `s.spfilter(c,e,tp,ft)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    // return c:IsCode(CARD_JACK_KNIGHT,CARD_KING_KNIGHT,CARD_QUEEN_KNIGHT)
    //     and ((c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP) and ft>0) or c:IsAbleToHand())
    card::is_code(d, c, &[CARD_JACK_KNIGHT, CARD_KING_KNIGHT, CARD_QUEEN_KNIGHT])
        && ((card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), None, None) && ft > 0) || card::is_able_to_hand(d, c, None))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp,ft)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, ft)), a.tp, LOCATION_DECK, 0, NO_EX);
    // if chk==0 then return aux.SelectUnselectGroup(g,e,tp,2,2,aux.dncheck,0) end
    if a.chk == Some(0) {
        let dn = aux::dncheck();
        return matches!(aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&dn), 0, a.tp, None, None, None, false), Err(true));
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,2,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 2, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp,ft)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, ft)), a.tp, LOCATION_DECK, 0, NO_EX);
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dncheck,1,tp,HINTMSG_SPSUMMON)
    let dn = aux::dncheck();
    let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&dn), 1, a.tp, Some(HINTMSG_SPSUMMON as u64), None, None, false).unwrap_or_default();
    // if #sg==2 then
    if sg.len() == 2 {
        // local tc=sg:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &sg, a.tp, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &sel).nil();
        // aux.ToHandOrElse(tc,tp,function(c)
        //     return tc:IsCanBeSpecialSummoned(e,0,tp,false,false) and ft>0 end,
        // function(c)
        //     Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP) end,
        // aux.Stringid(id,1))
        let check = |d: &mut Duel, _c: CardId| card::is_can_be_special_summoned(d, tc, a.e, 0, a.tp, false, false, None, None, None) && ft > 0;
        let oper = |d: &mut Duel, _g: &CardSet| {
            duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
            0
        };
        aux::to_hand_or_else(d, tc, a.tp, Some(&check), Some(&oper), Some(aux::stringid(ID, 1)));
        // tc=(sg-tc):Select(tp,1,1,nil):GetFirst()
        let mut rest = sg.clone();
        group::remove_card(&mut rest, &CardSet::from_card(tc));
        let sel2 = group::select(d, &rest, a.tp, 1, 1, false, NO_EX).nil();
        let tc2 = group::get_first(d, &sel2).nil();
        // aux.ToHandOrElse(tc,tp,...) with the new tc
        let check2 = |d: &mut Duel, _c: CardId| card::is_can_be_special_summoned(d, tc2, a.e, 0, a.tp, false, false, None, None, None) && ft > 0;
        let oper2 = |d: &mut Duel, _g: &CardSet| {
            duel::special_summon_step(d, tc2, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
            0
        };
        aux::to_hand_or_else(d, tc2, a.tp, Some(&check2), Some(&oper2), Some(aux::stringid(ID, 1)));
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
