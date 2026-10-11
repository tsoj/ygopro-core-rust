//! Simorgh Sky Battle
//! Scripted by AlphaKretin
use super::prelude::*;

pub const ID: u32 = 57823578;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SIMORGH], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Limit battle target
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    // e2:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e2:SetValue(s.atlimit)
    effect::set_value_fn(d, e2, effect_card_fn(atlimit));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Prevent effect target
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e3, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e3:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e3:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    // e3:SetTarget(s.atlimit)
    effect::set_target(d, e3, Some(effect_card_fn(atlimit)));
    // e3:SetValue(aux.tgoval)
    effect::set_value_fn(d, e3, aux::tgoval());
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Return cards to hand and inflict damage
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_TOHAND+CATEGORY_DAMAGE)
    effect::set_category(d, e4, CATEGORY_TOHAND + CATEGORY_DAMAGE);
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e4, EVENT_FREE_CHAIN);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetCost(s.thcost)
    effect::set_cost(d, e4, Some(cost_target(thcost)));
    // e4:SetTarget(s.thtg)
    effect::set_target(d, e4, Some(cost_target(thtg)));
    // e4:SetOperation(s.thop)
    effect::set_operation(d, e4, Some(operation(thop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    let mut a0 = a.clone();
    a0.chk = Some(0);
    let mut a1 = a.clone();
    a1.chk = Some(1);
    // if s.thcost(e,tp,eg,ep,ev,re,r,rp,0) and s.thtg(e,tp,eg,ep,ev,re,r,rp,0)
    //     and Duel.SelectYesNo(tp,94) then
    if thcost(d, &a0) && thtg(d, &a0) && duel::select_yes_no(d, a.tp, 94) {
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_DAMAGE)
        effect::set_category(d, a.e, CATEGORY_TOHAND + CATEGORY_DAMAGE);
        // s.thcost(e,tp,eg,ep,ev,re,r,rp,1)
        thcost(d, &a1);
        // s.thtg(e,tp,eg,ep,ev,re,r,rp,1)
        thtg(d, &a1);
        // e:SetOperation(s.thop)
        effect::set_operation(d, a.e, Some(operation(thop)));
    } else {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
        // e:SetOperation(nil)
        effect::set_operation(d, a.e, None);
    }
    true
}
/// `s.atkval(tp)`
fn atkval(d: &mut Duel, tp: u8) -> i64 {
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsRace,RACE_WINGEDBEAST),tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_WINGEDBEAST)), tp, LOCATION_MZONE, 0, NO_EX);
    // local _,val=g:GetMaxGroup(Card.GetAttack)
    // return val
    group::get_max_group(d, &g, &|d, c| card::get_attack(d, c) as i64).map(|x| x.1).nil()
}
/// `s.atlimit(e,c)`
fn atlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_WINGEDBEAST) and c:GetAttack()<s.atkval(e:GetHandlerPlayer())
    let tp = effect::get_handler_player(d, e);
    card::is_faceup(d, c) && card::is_race(d, c, RACE_WINGEDBEAST) && (card::get_attack(d, c) as i64) < atkval(d, tp)
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAbleToGraveAsCost() and c:IsSetCard(SET_SIMORGH) and c:IsLevelAbove(7)
    card::is_faceup(d, c) && card::is_able_to_grave_as_cost(d, c) && card::is_set_card(d, c, &[SET_SIMORGH as u16]) && card::is_level_above(d, c, 7)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetOriginalAttribute)==#sg and Duel.IsExistingMatchingCard(Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,sg+e:GetHandler())
    let h = effect::get_handler(d, e.nil()).nil();
    let classes = group::get_class_count(d, sg, &|d, c| card::get_original_attribute(d, c) as i64);
    let mut ex = sg.clone();
    group::add_card(&mut ex, &CardSet::from_card(h));
    let res = classes == sg.len()
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, (&ex).into());
    (res, false)
}
/// `s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    // return true
    true
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
    // if chk==0 then return e:GetLabel()==100 and c:IsAbleToGraveAsCost() and c:IsStatus(STATUS_EFFECT_ENABLED)
    //     and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return effect::get_label(d, a.e) == 100
            && card::is_able_to_grave_as_cost(d, c)
            && card::is_status(d, c, STATUS_EFFECT_ENABLED)
            && matches!(aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 0, a.tp, None, None, None, false), Err(true));
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_TOGRAVE)
    let mut tg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 1, a.tp, Some(HINTMSG_TOGRAVE as u64), None, None, false)
        .unwrap_or_default();
    // tg:AddCard(c)
    group::add_card(&mut tg, &CardSet::from_card(c));
    // Duel.SendtoGrave(tg,REASON_COST)
    duel::send_to_grave(d, &tg, REASON_COST, None, None);
    // e:SetLabel(0)
    effect::set_label(d, a.e, &[0]);
    // local g=Duel.GetMatchingGroup(Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // if Duel.SendtoHand(g,nil,REASON_EFFECT)>0 then
    if duel::send_to_hand(d, &g, None, REASON_EFFECT, None) > 0 {
        // local og=Duel.GetOperatedGroup():Filter(Card.IsLocation,nil,LOCATION_HAND)
        let opg = duel::get_operated_group(d);
        let og = group::filter(d, &opg, &|d, c| card::is_location(d, c, LOCATION_HAND), NO_EX);
        // if #og>0 then
        if !og.is_empty() {
            // local dam=Duel.Damage(tp,#og*500,REASON_EFFECT)
            let dam = duel::damage(d, tp, (og.len() * 500) as i64, REASON_EFFECT, false, None);
            // if dam>0 then
            if dam > 0 {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Damage(1-tp,dam,REASON_EFFECT)
                duel::damage(d, 1 - tp, dam as i64, REASON_EFFECT, false, None);
            }
        }
    }
}
