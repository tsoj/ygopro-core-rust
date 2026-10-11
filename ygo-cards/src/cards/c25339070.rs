//! Darklord Nasten
use super::prelude::*;

pub const ID: u32 = 25339070;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DARKLORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetSPSummonOnce(id)
    card::set_spsummon_once(d, c, ID);
    // spsummon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // copy effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.PayLP(1000))
    effect::set_cost(d, e2, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    // e2:SetTarget(s.cptg)
    effect::set_target(d, e2, Some(cost_target(cptg)));
    // e2:SetOperation(s.cpop)
    effect::set_operation(d, e2, Some(operation(cpop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_DARKLORD) and c:IsDiscardable()
    card::is_set_card(d, c, &[SET_DARKLORD as u16]) && card::is_discardable(d, c, None, None)
}
// function s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let h = effect::get_handler(d, e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,2,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_HAND, 0, 2, h.into());
    }
    // Duel.DiscardHand(tp,s.cfilter,2,2,REASON_COST|REASON_DISCARD,e:GetHandler())
    duel::discard_hand(d, tp, Some(&cfilter), 2, 2, REASON_COST | REASON_DISCARD, h.into());
    true
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let h = effect::get_handler(d, e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, h, e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
}
// function s.cpfilter(c)
fn cpfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_DARKLORD) and c:IsSpellTrap() and c:IsAbleToDeck() and c:CheckActivateEffect(false,true,false)~=nil
    card::is_set_card(d, c, &[SET_DARKLORD as u16])
        && card::is_spell_trap(d, c)
        && card::is_able_to_deck(d, c)
        && card::check_activate_effect(d, c, false, true, false).is_some()
}
// function s.cptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn cptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.cpfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && cpfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.cpfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&cpfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.cpfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&cpfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), 1, 0, 0);
    true
}
// function s.cpop(e,tp,eg,ep,ev,re,r,rp)
fn cpop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    // if not (tc and tc:IsRelateToEffect(e)) then return end
    let Some(tc) = duel::first_target(d) else { return };
    if !card::is_relate_to_effect(d, tc, e) {
        return;
    }
    // local te,ceg,cep,cev,cre,cr,crp=tc:CheckActivateEffect(false,true,true)
    // if not te then return end
    let Some((te, _)) = card::check_activate_effect(d, tc, false, true, true) else { return };
    // local tg=te:GetTarget()
    let tg = effect::get_target(d, te);
    // local op=te:GetOperation()
    let op = effect::get_operation(d, te);
    // if tg then tg(te,tp,Group.CreateGroup(),PLAYER_NONE,0,e,REASON_EFFECT,PLAYER_NONE,1) end
    if let Some(tg) = tg {
        tg.call(d, &copy_args(te, tp, e));
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // tc:CreateEffectRelation(te)
    card::create_effect_relation(d, tc, te);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned()).nil();
    // for etc in aux.Next(g) do etc:CreateEffectRelation(te) end
    for etc in group::iter(d, &g) {
        card::create_effect_relation(d, etc, te);
    }
    // if op then op(te,tp,Group.CreateGroup(),PLAYER_NONE,0,e,REASON_EFFECT,PLAYER_NONE,1) end
    if let Some(op) = op {
        op.call(d, &copy_args(te, tp, e));
    }
    // tc:ReleaseEffectRelation(te)
    card::release_effect_relation(d, tc, te);
    // for etc in aux.Next(g) do etc:ReleaseEffectRelation(te) end
    for etc in group::iter(d, &g) {
        card::release_effect_relation(d, etc, te);
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.SendtoDeck(te:GetHandler(),nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
    let h = effect::get_handler(d, te).nil();
    duel::send_to_deck(d, h, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
}
// Arguments of `tg(te,tp,Group.CreateGroup(),PLAYER_NONE,0,e,REASON_EFFECT,PLAYER_NONE,1)`.
fn copy_args(te: EffectId, tp: u8, e: EffectId) -> [Value; 9] {
    [
        Value::Effect(te),
        Value::Int(tp as i64),
        Value::Group(CardSet::new()),
        Value::Int(PLAYER_NONE as i64),
        Value::Int(0),
        Value::effect(Some(e)),
        Value::Int(REASON_EFFECT as i64),
        Value::Int(PLAYER_NONE as i64),
        Value::Int(1),
    ]
}
