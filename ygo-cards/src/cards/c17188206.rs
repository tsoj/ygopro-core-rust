//! Ｒ．Ｂ．ラムダブレード
//! R.B. Lambda Blade
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 17188206;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_RB], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send 1 "R.B." card from your Deck to the GY, except "R.B. Lambda Blade"
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    // e1a:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e1a, CATEGORY_TOGRAVE);
    // e1a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1a:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY, 0);
    // e1a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1a, EVENT_SUMMON_SUCCESS);
    // e1a:SetCountLimit(1,id)
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    // e1a:SetTarget(s.tgtg)
    effect::set_target(d, e1a, Some(cost_target(tgtg)));
    // e1a:SetOperation(s.tgop)
    effect::set_operation(d, e1a, Some(operation(tgop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // Destroy this card, and if you do, take control of an opponent's monster, but destroy it during the End Phase card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_CONTROL)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_CONTROL);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetCost(Cost.PayLP(1400))
    effect::set_cost(d, e2, Some(cost::pay_lp(cost::PayLp::Amount(1400))));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_RB) and c:IsAbleToGrave() and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_RB as u16]) && card::is_able_to_grave(d, c, None) && !card::is_code(d, c, &[ID])
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
/// `s.desconfilter(c,ec,lg)`
fn desconfilter(d: &mut Duel, c: CardId, ec: CardId, lg: &CardSet) -> bool {
    // return c:IsSetCard(SET_RB) and c:IsLinkMonster() and c:IsFaceup() and (lg:IsContains(c) or c:GetLinkedGroup():IsContains(ec))
    card::is_set_card(d, c, &[SET_RB as u16])
        && card::is_link_monster(d, c)
        && card::is_faceup(d, c)
        && (lg.contains(c) || card::get_linked_group(d, c).contains(ec))
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return Duel.IsMainPhase(1-tp) and Duel.IsExistingMatchingCard(s.desconfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,c,c,c:GetLinkedGroup())
    if !duel::is_main_phase(d, Some(1 - a.tp)) {
        return false;
    }
    let lg = card::get_linked_group(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| desconfilter(d, x, c, &lg)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into())
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsAbleToChangeControler() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && card::is_able_to_change_controler(d, chkc);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetMZoneCount(tp,c,tp,LOCATION_REASON_CONTROL)>0
    //     and Duel.IsExistingTarget(Card.IsAbleToChangeControler,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::get_m_zone_count(d, tp, c.into(), Some(tp), Some(LOCATION_REASON_CONTROL), None).0 > 0
            && duel::is_existing_target(d, Some(&|d, x| card::is_able_to_change_controler(d, x)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToChangeControler,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, x| card::is_able_to_change_controler(d, x)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(c)), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(g), 1, tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    // if c:IsRelateToEffect(e) and Duel.Destroy(c,REASON_EFFECT)>0
    //     and tc:IsRelateToEffect(e) and not tc:IsImmuneToEffect(e)
    //     and tc:IsControler(1-tp) and Duel.GetControl(tc,tp) then
    if !card::is_relate_to_effect(d, c, a.e) || duel::destroy(d, c, REASON_EFFECT, None, None) <= 0 {
        return;
    }
    let tc = duel::first_target(d).nil();
    if !(card::is_relate_to_effect(d, tc, a.e)
        && !card::is_immune_to_effect(d, tc, a.e)
        && card::is_controler(d, tc, 1 - tp)
        && duel::get_control(d, tc, tp, 0, 0, Some(0xff), None) != 0)
    {
        return;
    }
    // aux.DelayedOperation(tc,PHASE_END,id,e,tp,function(ag) Duel.Destroy(ag,REASON_EFFECT) end,nil,0,0,aux.Stringid(id,2))
    let oper: aux::DelayedFn = std::sync::Arc::new(|d: &mut Duel, ag: &CardSet, _a: &EffArgs| {
        duel::destroy(d, ag.clone(), REASON_EFFECT, None, None);
        true
    });
    aux::delayed_operation(d, tc, PHASE_END, ID, a.e, tp, Some(oper), None, Some(0), Some(0), Some(aux::stringid(ID, 2)), None);
}
