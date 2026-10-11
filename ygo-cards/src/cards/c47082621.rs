//! Regenesis Overlord
use super::prelude::*;

pub const ID: u32 = 47082621;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // c:AddMustBeSpecialSummoned()
    card::add_must_be_special_summoned(d, c);
    // Must be Special Summoned (from your GY) by banishing 2 monsters from your GY with 2500 ATK or DEF
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e0, aux::stringid(ID, 0));
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e0, EFFECT_SPSUMMON_PROC);
    // e0:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e0, LOCATION_GRAVE);
    // e0:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e0, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e0:SetCondition(s.selfspcon)
    effect::set_condition(d, e0, Some(lua_fn(selfspcon)));
    // e0:SetTarget(s.selfsptg)
    effect::set_target(d, e0, Some(lua_fn(selfsptg)));
    // e0:SetOperation(s.selfspop)
    effect::set_operation(d, e0, Some(lua_fn(selfspop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Return cards in your opponent's field and/or GY up to the number of your banished monsters with 2500 ATK or DEF to the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetTarget(s.rthtg)
    effect::set_target(d, e1, Some(cost_target(rthtg)));
    // e1:SetOperation(s.rthop)
    effect::set_operation(d, e1, Some(operation(rthop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Monsters your opponent controls lose 2500 ATK during the Battle Phase only
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    // e2:SetCondition(function() return Duel.IsBattlePhase() end)
    effect::set_condition(d, e2, Some(condition(battle_phase_cond)));
    // e2:SetValue(-2500)
    effect::set_value(d, e2, -2500);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function() return Duel.IsBattlePhase() end`
fn battle_phase_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_battle_phase(d, None)
}
/// `s.selfspconfilter(c)`
fn selfspconfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsAttack(2500) or c:IsDefense(2500)) and c:IsFaceup() and c:IsAbleToRemoveAsCost()
    //     and aux.SpElimFilter(c,true)
    (card::is_attack(d, c, &[2500]) || card::is_defense(d, c, &[2500]))
        && card::is_faceup(d, c)
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, false)
}
/// `s.selfspcon(e,c)`
fn selfspcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local g=Duel.GetMatchingGroup(s.selfspconfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,c)
    let g = duel::get_matching_group(d, Some(&selfspconfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, c.into());
    // return #g>=2 and Duel.GetMZoneCount(tp,g)>0
    g.len() >= 2 && duel::get_m_zone_count(d, tp, (&g).into(), None, None, None).0 > 0
}
/// `s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk,c)`
fn selfsptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let c = a.c(9);
    // local rg=Duel.GetMatchingGroup(s.selfspconfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,c)
    let rg = duel::get_matching_group(d, Some(&selfspconfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, c.into());
    // local g=aux.SelectUnselectGroup(rg,e,tp,2,2,aux.ChkfMMZ(1),1,tp,HINTMSG_REMOVE,nil,nil,true)
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(&aux::chkf_mmz(1)), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.selfspop(e,tp,eg,ep,ev,re,r,rp,c)`
fn selfspop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Value::Group(g) = effect::get_label_object(d, e) else {
        return;
    };
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
}
/// `s.rthctfilter(c)`
fn rthctfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsAttack(2500) or c:IsDefense(2500)) and c:IsFaceup()
    (card::is_attack(d, c, &[2500]) || card::is_defense(d, c, &[2500])) && card::is_faceup(d, c)
}
/// `s.rthtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rthtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_ONFIELD|LOCATION_GRAVE) and chkc:IsAbleToHand() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp)
            && card::is_location(d, chkc, LOCATION_ONFIELD | LOCATION_GRAVE)
            && card::is_able_to_hand(d, chkc, None);
    }
    // local ct=Duel.GetMatchingGroupCount(s.rthctfilter,tp,LOCATION_REMOVED,0,nil)
    let ct = duel::get_matching_group_count(d, Some(&rthctfilter), tp, LOCATION_REMOVED, 0, NO_EX);
    // if chk==0 then return ct>0 and Duel.IsExistingTarget(Card.IsAbleToHand,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return ct > 0
            && duel::is_existing_target(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToHand,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE,1,ct,nil)
    let g = duel::select_target(
        d,
        tp,
        Some(&|d, c| card::is_able_to_hand(d, c, None)),
        tp,
        0,
        LOCATION_ONFIELD | LOCATION_GRAVE,
        1,
        ct as u16,
        false,
        NO_EX,
    )
    .nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, tp, 0);
    true
}
/// `s.rthop(e,tp,eg,ep,ev,re,r,rp)`
fn rthop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.SendtoHand(tg,nil,REASON_EFFECT)
        duel::send_to_hand(d, tg, None, REASON_EFFECT, None);
    }
}
