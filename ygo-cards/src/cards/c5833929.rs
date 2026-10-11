//! Melffy Godmommy
use super::prelude::*;

pub const ID: u32 = 5833929;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[76833149], listed_series: &[SET_MELFFY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz.AddProcedure(c,aux.FilterBoolFunction(Card.IsRace,RACE_BEAST),2,3,s.altxyzmatfilter,aux.Stringid(id,0),Xyz.InfiniteMats,s.altxyzop)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, _sc, _st, _tp| card::is_race(d, c, RACE_BEAST as u64))),
        Some(2),
        3,
        Some(xyz::alter_filter(|d, c, tp, xyzc| altxyzmatfilter(d, c, tp, xyzc))),
        Some(aux::stringid(ID, 0)),
        Some(xyz::INFINITE_MATS),
        Some(lua_fn(altxyzop)),
        false,
        None,
    );
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_card_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCost(Cost.DetachFromSelf(5))
    effect::set_cost(d, e3, Some(cost::detach_from_self(cost::Count::N(5), None, None)));
    // e3:SetTarget(s.rthtg)
    effect::set_target(d, e3, Some(cost_target(rthtg)));
    // e3:SetOperation(s.rthop)
    effect::set_operation(d, e3, Some(operation(rthop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.altxyzmatfilter(c,tp,xyzc)`
fn altxyzmatfilter(d: &mut Duel, c: CardId, tp: u8, xyzc: CardId) -> bool {
    // return c:IsFaceup() and c:IsSummonCode(xyzc,SUMMON_TYPE_XYZ,tp,76833149)
    card::is_faceup(d, c) && card::is_summon_code(d, c, Some(xyzc), SUMMON_TYPE_XYZ as u64, tp, &[76833149])
}
/// `s.altxyzop(e,tp,chk)`
fn altxyzop(d: &mut Duel, a: Args) -> Value {
    let tp = a.p(1);
    // if chk==0 then return not Duel.HasFlagEffect(tp,id) end
    if a.int(2) == 0 {
        return Value::Bool(duel::get_flag_effect(d, tp, ID) == 0);
    }
    // return Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,EFFECT_FLAG_OATH,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, EFFECT_FLAG_OATH, 1, 0);
    Value::Bool(true)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MELFFY) and c:IsSpellTrap() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_MELFFY as u16]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,...,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.thop(e,tp,...)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil)
    let nf = aux::necro_valley_filter(|d, c| thfilter(d, c));
    let g = duel::select_matching_card(d, tp, Some(&nf), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // local g=e:GetHandler():GetOverlayGroup():Filter(Card.IsSetCard,nil,SET_MELFFY)
    let og = card::get_overlay_group(d, c);
    let g = group::filter(d, &og, &|d, c| card::is_set_card(d, c, &[SET_MELFFY as u16]), NO_EX);
    // return g:GetSum(Card.GetAttack)+g:GetSum(Card.GetDefense)
    let atk = group::get_sum(d, &g, &|d, c| card::get_attack(d, c) as i64);
    let def = group::get_sum(d, &g, &|d, c| card::get_defense(d, c) as i64);
    atk + def
}
/// `s.rthfilter`-less: `Card.IsAbleToHand` on the opponent's field
fn rthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToHand,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToHand,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, a.tp, 0);
    true
}
/// `s.rthop(e,tp,...)`
fn rthop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsAbleToHand,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
