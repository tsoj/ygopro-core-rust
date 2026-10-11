//! Nekroz of Trishula
use super::prelude::*;

pub const ID: u32 = 52068432;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_NEKROZ],
    mat_filter: Some(mat_filter),
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --cannot special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(aux.ritlimit)
    effect::set_value_fn(d, e1, aux::ritlimit());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --negate
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_NEGATE)
    effect::set_category(d, e2, CATEGORY_NEGATE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.negcon)
    effect::set_condition(d, e2, Some(condition(negcon)));
    // e2:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e2, Some(cost::self_discard()));
    // e2:SetTarget(s.negtg)
    effect::set_target(d, e2, Some(cost_target(negtg)));
    // e2:SetOperation(s.negop)
    effect::set_operation(d, e2, Some(operation(negop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --remove
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e3, CATEGORY_REMOVE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.remcon)
    effect::set_condition(d, e3, Some(condition(remcon)));
    // e3:SetTarget(s.remtg)
    effect::set_target(d, e3, Some(cost_target(remtg)));
    // e3:SetOperation(s.remop)
    effect::set_operation(d, e3, Some(operation(remop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.mat_filter(c)`
fn mat_filter(d: &mut Duel, c: CardId, _tp: u8) -> bool {
    // return c:GetLevel()~=9
    card::get_level(d, c) != 9
}
/// `s.tfilter(c,tp)`
fn tfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_NEKROZ) and c:IsControler(tp) and c:IsLocation(LOCATION_MZONE)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_NEKROZ as u16]) && card::is_controler(d, c, tp) && card::is_location(d, c, LOCATION_MZONE)
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return false end
    let re = a.re.nil();
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    // return g and g:IsExists(s.tfilter,1,nil,tp) and Duel.IsChainNegatable(ev)
    let g = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned();
    match g {
        Some(g) => group::is_exists(d, &g, &|d, c| tfilter(d, c, a.tp), 1, NO_EX) && duel::is_chain_negatable(d, a.ev as u8),
        None => false,
    }
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) && effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE) {
        let rh = effect::get_handler(d, a.re.nil()).nil();
        if card::is_relate_to_effect(d, rh, a.re.nil()) {
            // Duel.SendtoGrave(eg,REASON_EFFECT)
            duel::send_to_grave(d, a.eg.clone().nil(), REASON_EFFECT, None, None);
        }
    }
}
/// `s.remcon(e,tp,eg,ep,ev,re,r,rp)`
fn remcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsRitualSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_ritual_summoned(d, h)
}
/// `s.remtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn remtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(e:GetHandlerPlayer(),CARD_SPIRIT_ELIMINATION)
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_HAND,1,nil)
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,1,nil)
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        let hp = effect::get_handler_player(d, a.e);
        return duel::is_player_affected_by_effect(d, hp, CARD_SPIRIT_ELIMINATION).is_none()
            && duel::is_existing_matching_card(d, Some(&is_able_to_remove), a.tp, 0, LOCATION_HAND, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&is_able_to_remove), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&is_able_to_remove), a.tp, 0, LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,0,LOCATION_ONFIELD|LOCATION_GRAVE|LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 0, (LOCATION_ONFIELD | LOCATION_GRAVE | LOCATION_HAND) as i32);
    true
}
/// `Card.IsAbleToRemove` as a filter
fn is_able_to_remove(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_remove(d, c, None, None, None)
}
/// `s.remop(e,tp,eg,ep,ev,re,r,rp)`
fn remop(d: &mut Duel, a: &EffArgs) {
    // local g1=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_HAND,nil)
    let g1 = duel::get_matching_group(d, Some(&is_able_to_remove), a.tp, 0, LOCATION_HAND, NO_EX);
    // local g2=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,nil)
    let g2 = duel::get_matching_group(d, Some(&is_able_to_remove), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // local g3=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_GRAVE,nil)
    let g3 = duel::get_matching_group(d, Some(&is_able_to_remove), a.tp, 0, LOCATION_GRAVE, NO_EX);
    // if #g1>0 and #g2>0 and #g3>0 then
    if !g1.is_empty() && !g2.is_empty() && !g3.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // local sg1=g1:RandomSelect(tp,1)
        let mut sg1 = group::random_select(d, &g1, a.tp, 1);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // local sg2=g2:Select(tp,1,1,nil)
        let sg2 = group::select(d, &g2, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // local sg3=g3:Select(tp,1,1,nil)
        let sg3 = group::select(d, &g3, a.tp, 1, 1, false, NO_EX).nil();
        // sg1:Merge(sg2)
        group::add_card(&mut sg1, &sg2);
        // sg1:Merge(sg3)
        group::add_card(&mut sg1, &sg3);
        // Duel.HintSelection(sg1)
        duel::hint_selection(d, sg1.clone(), true);
        // Duel.Remove(sg1,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, sg1, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
