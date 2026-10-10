//! Temple of the Kings
use super::prelude::*;

pub const ID: u32 = 29762407;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[89194033],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Allow 1 Trap card to be activated the turn it was Set
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_TRAP_ACT_IN_SET_TURN)
    effect::set_code(d, e2, EFFECT_TRAP_ACT_IN_SET_TURN);
    // e2:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e2, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_SZONE,0)
    effect::set_target_range(d, e2, LOCATION_SZONE, 0);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 1 monster from the hand, Deck or Extra Deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCost(s.cost)
    effect::set_cost(d, e3, Some(cost_target(cost)));
    // e3:SetTarget(s.target)
    effect::set_target(d, e3, Some(cost_target(target)));
    // e3:SetOperation(s.operation)
    effect::set_operation(d, e3, Some(operation(op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.cfilter(c,e,tp,rp)`
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rp: u8) -> bool {
    // if c:IsFacedown() or not c:IsCode(89194033) or not c:IsAbleToGraveAsCost() then return false end
    if card::is_facedown(d, c) || !card::is_code(d, c, &[89194033]) || !card::is_able_to_grave_as_cost(d, c) {
        return false;
    }
    // return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK|LOCATION_HAND|LOCATION_EXTRA,0,1,nil,e,tp,rp,Group.FromCards(c,e:GetHandler()))
    let h = effect::get_handler(d, e).nil();
    let mut sg = CardSet::from_card(c);
    group::add_card(&mut sg, &CardSet::from_card(h));
    duel::is_existing_matching_card(
        d,
        Some(&|d, x| filter(d, x, e, tp, rp, Some(&sg))),
        tp,
        LOCATION_DECK | LOCATION_HAND | LOCATION_EXTRA,
        0,
        1,
        NO_EX,
    )
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let _ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then return e:GetHandler():IsAbleToGraveAsCost()
    //     and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_ONFIELD,0,1,nil,e,tp,rp) end
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_grave_as_cost(d, h)
            && duel::is_existing_matching_card(d, Some(&|d, x| cfilter(d, x, a.e, a.tp, a.rp)), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_ONFIELD,0,1,1,nil,e,tp,rp)
    let mut g = duel::select_matching_card(d, a.tp, Some(&|d, x| cfilter(d, x, a.e, a.tp, a.rp)), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // g:AddCard(e:GetHandler())
    group::add_card(&mut g, &CardSet::from_card(h));
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.filter(c,e,tp,rp,sg)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rp: u8, sg: Option<&CardSet>) -> bool {
    // if not c:IsCanBeSpecialSummoned(e,0,tp,false,false) then return false end
    if !card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None) {
        return false;
    }
    // if c:IsLocation(LOCATION_HAND|LOCATION_DECK) then
    if card::is_location(d, c, LOCATION_HAND | LOCATION_DECK) {
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    } else {
        // return c:IsType(TYPE_FUSION) and Duel.GetLocationCountFromEx(tp,rp,sg,c)>0
        let ex = match sg {
            Some(g) => Exclude::Group(g),
            None => NO_EX,
        };
        card::is_type(d, c, TYPE_FUSION) && duel::get_location_count_from_ex(d, tp, Some(rp), ex, duel::ExtraCard::Card(c), None).0 > 0
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_HAND|LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_DECK | LOCATION_HAND | LOCATION_EXTRA) as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK|LOCATION_HAND|LOCATION_EXTRA,0,1,1,nil,e,tp,rp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| filter(d, x, a.e, a.tp, a.rp, None)), a.tp, LOCATION_DECK | LOCATION_HAND | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
