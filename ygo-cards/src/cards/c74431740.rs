//! Assault Reboot
use super::prelude::*;

pub const ID: u32 = 74431740;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ASSAULT_MODE], listed_names: &[CARD_ASSAULT_MODE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // shuffle
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // return true
    true
}
/// `s.cfilter(c,e,tp,ft)`
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    // return c:IsSetCard(SET_ASSAULT_MODE) and (ft>0 or (c:GetSequence()<5 and c:IsControler(tp)))
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,e,tp,c:GetCode())
    card::is_set_card(d, c, &[SET_ASSAULT_MODE as u16])
        && (ft > 0 || (card::get_sequence(d, c) < 5 && card::is_controler(d, c, tp)))
        && {
            let cd = card::get_code(d, c)[0];
            duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, e, tp, cd)), tp, LOCATION_DECK, 0, 1, NO_EX)
        }
}
/// `s.filter(c,e,tp,cd)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, cd: u32) -> bool {
    // return c:IsSetCard(SET_ASSAULT_MODE) and not c:IsCode(cd) and c:IsCanBeSpecialSummoned(e,0,tp,true,false,POS_FACEUP_DEFENSE)
    card::is_set_card(d, c, &[SET_ASSAULT_MODE as u16])
        && !card::is_code(d, c, &[cd])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=1 then return false end
        if effect::get_label(d, e) != 1 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // return ft>-1 and Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,nil,nil,e,tp,ft)
        return ft > -1
            && duel::check_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, e, tp, ft)), 1, 1, false, None, NO_EX);
    }
    // local rg=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,nil,nil,e,tp,ft)
    let rg = duel::select_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, e, tp, ft)), 1, 1, false, None, NO_EX);
    // Duel.SetTargetParam(rg:GetFirst():GetCode())
    let first = group::get_first(d, &rg).nil();
    let code = card::get_code(d, first)[0];
    duel::set_target_param(d, code);
    // Duel.Release(rg,REASON_COST)
    duel::release(d, rg, REASON_COST, None);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local code=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let code = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int() as u32;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil,e,tp,code):GetFirst()
    let sg = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, e, tp, code)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if tc then
    if let Some(tc) = group::get_first(d, &sg) {
        // Duel.SpecialSummon(tc,0,tp,tp,true,false,POS_FACEUP_DEFENSE)
        duel::special_summon(d, CardSet::from_card(tc), 0, tp, tp, true, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
/// `s.tdfilter(c,e)`
fn tdfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return (c:IsCode(CARD_ASSAULT_MODE) or c:ListsCode(CARD_ASSAULT_MODE)) and not c:IsCode(id)
    //     and c:IsCanBeEffectTarget(e) and c:IsAbleToDeck()
    (card::is_code(d, c, &[CARD_ASSAULT_MODE]) || card::lists_code(d, c, &[CARD_ASSAULT_MODE]))
        && !card::is_code(d, c, &[ID])
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_able_to_deck(d, c)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tdfilter,tp,LOCATION_GRAVE,0,1,nil,e) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| tdfilter(d, c, e)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_GRAVE,0,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| tdfilter(d, c, e)), tp, LOCATION_GRAVE, 0, NO_EX);
    // local ct=g:GetClassCount(Card.GetCode)
    let ct = group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64);
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,ct,aux.dncheck,1,tp,HINTMSG_TODECK)
    let dn = aux::dncheck();
    let rs: aux::Rescon = &dn;
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(ct), Some(rs), 1, tp, Some(HINTMSG_TODECK as u64), None, None, false)
        .unwrap_or_default();
    // Duel.SetTargetCard(sg)
    duel::set_target_card(d, sg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,sg,#sg,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(sg.clone()), sg.len() as u8, 0, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg==0 then return end
    if tg.len() == 0 {
        return;
    }
    // Duel.SendtoDeck(tg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
    duel::send_to_deck(d, tg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
}
