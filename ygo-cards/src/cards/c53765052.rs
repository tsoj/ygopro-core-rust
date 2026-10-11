//! Diabellze the Original Sinkeeper
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 53765052;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SINFUL_SPOILS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card (from your hand)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Your opponent cannot activate Spell/Trap Cards that were not Set
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(0,1)
    effect::set_target_range(d, e2, 0, 1);
    // e2:SetValue(function(e,re,tp) return re:IsHasType(EFFECT_TYPE_ACTIVATE) and not re:GetHandler():IsLocation(LOCATION_SZONE) end)
    effect::set_value_fn(d, e2, lua_fn(e2_value));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy 1 card you control and 1 card your opponent controls
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e3:SetCode(EVENT_SSET)
    effect::set_code(d, e3, EVENT_SSET);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.spcon(e,c)
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // return Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(Card.IsSetCard,0,LOCATION_GRAVE,LOCATION_GRAVE,1,nil,SET_SINFUL_SPOILS)
    let p = card::get_controler(d, c);
    duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(
            d,
            Some(&|d, x| card::is_set_card(d, x, &[SET_SINFUL_SPOILS as u16])),
            0,
            LOCATION_GRAVE,
            LOCATION_GRAVE,
            1,
            NO_EX,
        )
}
// function(e,re,tp) return re:IsHasType(EFFECT_TYPE_ACTIVATE) and not re:GetHandler():IsLocation(LOCATION_SZONE) end
fn e2_value(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && {
        let h = effect::get_handler(d, re).nil();
        !card::is_location(d, h, LOCATION_SZONE)
    }
}
// function s.desrescon(sg,e,tp,mg)
fn desrescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:FilterCount(Card.IsControler,nil,tp)==1
    (group::filter_count(d, sg, &|d, c| card::is_controler(d, c, tp), NO_EX) == 1, false)
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    let e = a.e;
    // local g=Duel.GetMatchingGroup(Card.IsCanBeEffectTarget,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil,e)
    let g = duel::get_matching_group(
        d,
        Some(&|d, c| card::is_can_be_effect_target(d, c, Some(e))),
        a.tp,
        LOCATION_ONFIELD,
        LOCATION_ONFIELD,
        NO_EX,
    );
    // if chk==0 then return aux.SelectUnselectGroup(g,e,tp,2,2,s.desrescon,0) end
    if a.chk == Some(0) {
        return matches!(
            aux::select_unselect_group(d, &g, Some(e), a.tp, Some(2), Some(2), Some(&desrescon), 0, a.tp, None, None, None, false),
            Err(true)
        );
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,s.desrescon,1,tp,HINTMSG_DESTROY)
    let tg = aux::select_unselect_group(
        d,
        &g,
        Some(e),
        a.tp,
        Some(2),
        Some(2),
        Some(&desrescon),
        1,
        a.tp,
        Some(HINTMSG_DESTROY as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tg,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(tg), 2, a.tp, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g>0 then Duel.Destroy(g,REASON_EFFECT) end
    if !g.is_empty() {
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
