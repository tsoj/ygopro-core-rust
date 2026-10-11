//! Evil HERO Dead-End Prison
use super::prelude::*;

pub const ID: u32 = 30583090;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_HERO], listed_names: &[CARD_DARK_FUSION], ..ScriptInfo::DEFAULT };

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
    // Make this card a Fusion substitute monster for any 1 material that is mentioned on a "HERO" Fusion Monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(s.fsubcost)
    effect::set_cost(d, e2, Some(cost_target(fsubcost)));
    // e2:SetOperation(s.fsubop)
    effect::set_operation(d, e2, Some(operation(fsubop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.spconfilter(c)
fn spconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c.dark_calling and c:IsType(TYPE_FUSION) and c:IsFaceup()
    crate::card_info(d, c).dark_calling && card::is_type(d, c, TYPE_FUSION) && card::is_faceup(d, c)
}
// function s.spcon(e,c)
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let c = match a.card(1) {
        None => return true,
        Some(c) => c,
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spconfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&spconfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
// function s.fsubcostfilter(c)
fn fsubcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_HERO) and c:IsMonster() and c:IsAbleToGraveAsCost()
    card::is_set_card(d, c, &[SET_HERO as u16]) && card::is_monster(d, c) && card::is_able_to_grave_as_cost(d, c)
}
// function s.fsubcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn fsubcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.fsubcostfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&fsubcostfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.fsubcostfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&fsubcostfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
// function s.fsubop(e,tp,eg,ep,ev,re,r,rp)
fn fsubop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // If you Fusion Summon a "HERO" monster this turn, this card can be used as a substitute for any 1 Fusion Material whose name is mentioned on it
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,2))
        effect::set_description(d, e1, aux::stringid(ID, 2));
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_FUSION_SUBSTITUTE)
        effect::set_code(d, e1, EFFECT_FUSION_SUBSTITUTE);
        // e1:SetValue(function(e,fc) return fc:IsSetCard(SET_HERO) end)
        effect::set_value_fn(d, e1, effect_card_fn(|d, _e, fc| card::is_set_card(d, fc, &[SET_HERO as u16])));
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
