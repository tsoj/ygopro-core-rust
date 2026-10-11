//! Subsurface Stage Divers
use super::prelude::*;

pub const ID: u32 = 43994202;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and not c:IsType(TYPE_TOKEN)
    card::is_faceup(d, c) && !card::is_type(d, c, TYPE_TOKEN)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetFieldGroup(tp,LOCATION_MZONE,0)
    let g = duel::get_field_group(d, a.tp, LOCATION_MZONE, 0);
    // return #g==3 and g:FilterCount(s.cfilter,nil)==3
    //     and g:GetClassCount(Card.GetCode)==1
    g.len() == 3
        && group::filter_count(d, &g, &|d, c| cfilter(d, c), NO_EX) == 3
        && group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) == 1
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToHand()
    card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thcheck(sg,e,tp,mg)`
fn thcheck(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetCode)==1
    (group::get_class_count(d, sg, &|d, c| card::get_code(d, c)[0] as i64) == 1, false)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_DECK,0,1,nil)
        let g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_DECK, 0, NO_EX);
        // return aux.SelectUnselectGroup(g,e,tp,3,3,s.thcheck,0)
        return matches!(
            aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(&thcheck), 0, tp, None, None, None, false),
            Err(true)
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,3,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 3, tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local dg=Duel.GetMatchingGroup(s.filter,tp,LOCATION_DECK,0,1,nil)
    let dg = duel::get_matching_group(d, Some(&filter), tp, LOCATION_DECK, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(dg,e,tp,3,3,s.thcheck,1,tp,HINTMSG_ATOHAND)
    let g = aux::select_unselect_group(d, &dg, Some(e), tp, Some(3), Some(3), Some(&thcheck), 1, tp, Some(HINTMSG_ATOHAND as u64), None, None, false).unwrap_or_default();
    // if #g==3 and Duel.SendtoHand(g,nil,REASON_EFFECT)==3 then
    if g.len() == 3 && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) == 3 {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g.clone());
        // local tc=g:GetFirst()
        let tc = group::get_first(d, &g).nil();
        // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
        if !effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) {
            return;
        }
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_CANNOT_SUMMON)
        effect::set_code(d, e1, EFFECT_CANNOT_SUMMON);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetTarget(s.sumlimit)
        effect::set_target(d, e1, Some(effect_card_fn(sumlimit)));
        // e1:SetLabel(tc:GetCode())
        let code = card::get_code(d, tc)[0] as i64;
        effect::set_label(d, e1, &[code]);
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
        // local e2=e1:Clone()
        // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, tp);
        // local e3=e1:Clone()
        // e3:SetCode(EFFECT_CANNOT_MSET)
        let e3 = effect::clone(d, e1);
        effect::set_code(d, e3, EFFECT_CANNOT_MSET);
        // Duel.RegisterEffect(e3,tp)
        duel::register_effect(d, e3, tp);
        // local e4=e1:Clone()
        // e4:SetCode(EFFECT_CANNOT_ACTIVATE)
        let e4 = effect::clone(d, e1);
        effect::set_code(d, e4, EFFECT_CANNOT_ACTIVATE);
        // e4:SetValue(s.aclimit)
        effect::set_value_fn(d, e4, lua_fn(aclimit));
        // Duel.RegisterEffect(e4,tp)
        duel::register_effect(d, e4, tp);
    }
}
/// `s.sumlimit(e,c)`
fn sumlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsCode(e:GetLabel())
    let label = effect::get_label(d, e) as u32;
    card::is_code(d, c, &[label])
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // return re:GetHandler():IsCode(e:GetLabel()) and re:IsMonsterEffect()
    let e = a.e(0);
    let re = a.e(1);
    let label = effect::get_label(d, e) as u32;
    let h = effect::get_handler(d, re).nil();
    card::is_code(d, h, &[label]) && effect::is_monster_effect(d, re)
}
