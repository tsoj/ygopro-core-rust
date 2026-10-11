//! Darklord Tezcatlipoca
use super::prelude::*;

pub const ID: u32 = 88234365;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DARKLORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetSPSummonOnce(id)
    card::set_spsummon_once(d, c, ID);
    // destroy replace
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e1, EFFECT_DESTROY_REPLACE);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetTarget(s.reptg)
    effect::set_target(d, e1, Some(cost_target(reptg)));
    // e1:SetValue(s.repval)
    effect::set_value_fn(d, e1, effect_card_fn(repval));
    // e1:SetOperation(s.repop)
    effect::set_operation(d, e1, Some(operation(repop)));
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
// function s.repfilter(c,tp)
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_DARKLORD) and c:IsLocation(LOCATION_MZONE) and c:IsControler(tp)
    //     and not c:IsReason(REASON_REPLACE) and c:IsReason(REASON_EFFECT|REASON_BATTLE)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_DARKLORD as u16])
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_controler(d, c, tp)
        && !card::is_reason(d, c, REASON_REPLACE)
        && card::is_reason(d, c, REASON_EFFECT | REASON_BATTLE)
}
// function s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let h = effect::get_handler(d, e).nil();
    // if chk==0 then return e:GetHandler():IsDiscardable() and eg:IsExists(s.repfilter,1,nil,tp) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return card::is_discardable(d, h, None, None)
            && group::is_exists(d, &eg, &|d, c| repfilter(d, c, tp), 1, NO_EX);
    }
    // return Duel.SelectEffectYesNo(tp,e:GetHandler(),96)
    duel::select_effect_yes_no(d, tp, h, Some(96))
}
// function s.repval(e,c)
fn repval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return s.repfilter(c,e:GetHandlerPlayer())
    let p = effect::get_handler_player(d, e);
    repfilter(d, c, p)
}
// function s.repop(e,tp,eg,ep,ev,re,r,rp)
fn repop(d: &mut Duel, a: &EffArgs) {
    // Duel.SendtoGrave(e:GetHandler(),REASON_EFFECT|REASON_DISCARD)
    let h = effect::get_handler(d, a.e).nil();
    duel::send_to_grave(d, h, REASON_EFFECT | REASON_DISCARD, None, None);
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
