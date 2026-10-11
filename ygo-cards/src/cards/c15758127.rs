//! Ursarctic Quint Charge
use super::prelude::*;

pub const ID: u32 = 15758127;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_URSARCTIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Make the opponent shuffle cards into the Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.tdcon)
    effect::set_condition(d, e2, Some(condition(tdcon)));
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Add 1 "Ursarctic" monster to the hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
    effect::set_count_limit(d, e3, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    // e3:SetCost(Cost.PayLP(700))
    effect::set_cost(d, e3, Some(cost::pay_lp(cost::PayLp::Amount(700))));
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Special Summon 1 "Ursarctic" monster from the Extra Deck
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e4, aux::stringid(ID, 3));
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    // e4:SetTarget(s.sptg)
    effect::set_target(d, e4, Some(cost_target(sptg)));
    // e4:SetOperation(s.spop)
    effect::set_operation(d, e4, Some(operation(spop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.tdcon(e,tp,eg,ep,ev,re,r,rp)
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=eg:GetFirst()
    let tc = group::get_first(d, &a.eg.clone().nil()).nil();
    // local bc=tc:GetBattleTarget()
    let bc = card::get_battle_target(d, tc);
    // return tc:IsPreviousControler(tp) and tc:IsPreviousSetCard(SET_URSARCTIC)
    //     and tc:GetType()&TYPE_MONSTER+TYPE_SYNCHRO==TYPE_MONSTER+TYPE_SYNCHRO
    //     and (tc:GetBattlePosition()&POS_FACEUP)>0
    //     and bc:IsRelateToBattle() and bc:IsControler(1-tp) and bc==Duel.GetAttacker()
    card::is_previous_controler(d, tc, a.tp)
        && card::is_previous_set_card(d, tc, &[SET_URSARCTIC as u16])
        && (card::get_type(d, tc) & (TYPE_MONSTER + TYPE_SYNCHRO)) == (TYPE_MONSTER + TYPE_SYNCHRO)
        && (card::get_battle_position(d, tc) & POS_FACEUP) > 0
        && {
            let bc = bc.nil();
            card::is_relate_to_battle(d, bc) && card::is_controler(d, bc, 1 - a.tp) && Some(bc) == duel::get_attacker(d)
        }
}
// function s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD|LOCATION_GRAVE|LOCATION_HAND)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE | LOCATION_HAND);
    // if chk==0 then return #g>7 end
    if a.chk == Some(0) {
        return g.len() > 7;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g-7,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g.clone()), (g.len() as i64 - 7) as u8, 0, 0);
    true
}
// function s.tdop(e,tp,eg,ep,ev,re,r,rp)
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD|LOCATION_GRAVE|LOCATION_HAND)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE | LOCATION_HAND);
    // if #g>7 then
    if g.len() > 7 {
        // local ct=#g-7
        let ct = g.len() - 7;
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_TODECK as u64);
        // local sg=g:Select(1-tp,ct,ct,nil)
        let sg = group::select(d, &g, 1 - a.tp, ct as u16, ct as u16, false, NO_EX).nil();
        // Duel.SendtoDeck(sg,nil,SEQ_DECKSHUFFLE,REASON_RULE,1-tp)
        duel::send_to_deck(d, sg, None, SEQ_DECKSHUFFLE as i32, REASON_RULE, Some(1 - a.tp));
    }
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_URSARCTIC) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_URSARCTIC as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_OPSELECTED,0,aux.Stringid(id,2))
    duel::hint(d, HINT_OPSELECTED as u8, 0, aux::stringid(ID, 2));
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,tc)
        duel::confirm_cards(d, 1 - a.tp, tc);
    }
}
// function s.spcfilter(c)
fn spcfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_URSARCTIC) and c:HasLevel() and c:IsReleasableByEffect()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_URSARCTIC as u16]) && card::has_level(d, c) && card::is_releasable_by_effect(d, c, None, None)
}
// function s.spfilter(c,e,tp,sg)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, sg: &CardSet) -> bool {
    // local tc1=sg:GetFirst()
    // local tc2=sg:GetNext()
    let order = group::iter(d, sg);
    let tc1 = order.first().copied().nil();
    let tc2 = order.get(1).copied().nil();
    // local lv=math.abs(tc1:GetLevel()-tc2:GetLevel())
    let lv = (card::get_level(d, tc1) as i64 - card::get_level(d, tc2) as i64).abs();
    // return c:IsSetCard(SET_URSARCTIC) and c:IsLevel(lv) and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    //     and Duel.GetLocationCountFromEx(tp,tp,sg,c)>0
    card::is_set_card(d, c, &[SET_URSARCTIC as u16])
        && card::is_level(d, c, &[lv as u32])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), sg.into(), duel::ExtraCard::Card(c), None).0 > 0
}
// function s.spcheck(sg,e,tp,mg)
fn spcheck(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let e = e.nil();
    // return #sg==2 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,sg)
    let ok = sg.len() == 2 && duel::is_existing_matching_card(d, Some(&|d: &mut Duel, c: CardId| spfilter(d, c, e, tp, sg)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    (ok, false)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.spcfilter,tp,LOCATION_MZONE,0,nil)
        let g = duel::get_matching_group(d, Some(&spcfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
        // return aux.SelectUnselectGroup(g,e,tp,2,2,s.spcheck,0)
        return matches!(aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&spcheck), 0, a.tp, None, None, None, false), Err(true));
    }
    // Duel.Hint(HINT_OPSELECTED,0,aux.Stringid(id,3))
    duel::hint(d, HINT_OPSELECTED as u8, 0, aux::stringid(ID, 3));
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.spcfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&spcfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
    // local rg=aux.SelectUnselectGroup(g,e,tp,2,2,s.spcheck,1,tp,HINTMSG_TOGRAVE,s.spcheck)
    let rg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&spcheck), 1, a.tp, Some(HINTMSG_TOGRAVE as u64), Some(&spcheck), None, false).unwrap_or_default();
    // if #rg~=2 then return end
    if rg.len() != 2 {
        return;
    }
    // local sg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_EXTRA,0,nil,e,tp,rg)
    let sg = duel::get_matching_group(d, Some(&|d: &mut Duel, c: CardId| spfilter(d, c, a.e, a.tp, &rg)), a.tp, LOCATION_EXTRA, 0, NO_EX);
    // if Duel.Release(rg,REASON_EFFECT)==2 then
    if duel::release(d, rg.clone(), REASON_EFFECT, None) == 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sc=sg:Select(tp,1,1,nil)
        let sc = group::select(d, &sg, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.SpecialSummon(sc,0,tp,tp,true,false,POS_FACEUP)
        duel::special_summon(d, sc, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None);
    }
}
