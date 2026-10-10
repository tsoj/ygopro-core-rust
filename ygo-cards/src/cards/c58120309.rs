//! Starlight Road
use super::prelude::*;

pub const ID: u32 = 58120309;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_STARDUST_DRAGON],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DISABLE+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DISABLE + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsOnField()
    card::is_controler(d, c, tp) && card::is_on_field(d, c)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // if not Duel.IsChainNegatable(ev) then return false end
    if !duel::is_chain_negatable(d, a.ev as u8) {
        return false;
    }
    // local ex,tg,tc=Duel.GetOperationInfo(ev,CATEGORY_DESTROY)
    let Some(info) = duel::get_operation_info(d, a.ev, CATEGORY_DESTROY) else {
        return false;
    };
    // return ex and tg~=nil and tc+tg:FilterCount(s.cfilter,nil,tp)-#tg>1
    let Some(tg) = info.op_cards else {
        return false;
    };
    let fc = group::filter_count(d, &tg, &|d, x| cfilter(d, x, a.tp), NO_EX) as i64;
    info.op_count as i64 + fc - group::get_count(&tg) as i64 > 1
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // if rc:IsDestructable() and rc:IsRelateToEffect(re) then
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_STARDUST_DRAGON) and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[CARD_STARDUST_DRAGON])
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.NegateEffect(ev) and re:GetHandler():IsRelateToEffect(re) and Duel.Destroy(eg,REASON_EFFECT)>0 then
    if !duel::negate_effect(d, a.ev as u8) {
        return;
    }
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    if !card::is_relate_to_effect(d, rc, re) {
        return;
    }
    if duel::destroy(d, a.eg.clone().nil(), REASON_EFFECT, None, None) <= 0 {
        return;
    }
    // local sg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_EXTRA,0,nil,e,tp)
    let sg = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if not (#sg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1))) then return end
    if sg.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=sg:Select(tp,1,1,nil)
    let sc = group::select(d, &sg, tp, 1, 1, false, NO_EX).nil();
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
}
