//! Level Up!
use super::prelude::*;

pub const ID: u32 = 25290459;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LV], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.costfilter(c,e,tp)`
fn costfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if not c:IsSetCard(SET_LV) or not c:IsAbleToGraveAsCost() or not c:IsFaceup() then return false end
    if !card::is_set_card(d, c, &[SET_LV as u16]) || !card::is_able_to_grave_as_cost(d, c) || !card::is_faceup(d, c) {
        return false;
    }
    // return c.listed_names and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,c,e,tp)
    let listed = crate::card_info(d, c).listed_names;
    !listed.is_empty() && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, listed, e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.spfilter(c,class,e,tp)`: `class.listed_names` is passed as `listed`
fn spfilter(d: &mut Duel, c: CardId, listed: &[u32], e: EffectId, tp: u8) -> bool {
    // return c:IsMonster() and c:IsCanBeSpecialSummoned(e,0,tp,true,true)
    //   and class.listed_names and c:IsCode(table.unpack(class.listed_names))
    card::is_monster(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, true, None, None, None)
        && !listed.is_empty()
        && card::is_code(d, c, listed)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // if chk==0 then return true end
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=1 then return false end
        if effect::get_label(d, a.e) != 1 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_MZONE,0,1,nil,e,tp)
        //   and Duel.GetLocationCount(tp,LOCATION_MZONE)>-1
        return duel::is_existing_matching_card(d, Some(&|d, x| costfilter(d, x, a.e, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > -1;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| costfilter(d, x, a.e, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g.clone(), REASON_COST, None, None);
    // local code=g:GetFirst():GetOriginalCode()
    let tc = group::get_first(d, &g).nil();
    let code = card::get_original_code(d, tc);
    // Duel.SetTargetParam(code)
    duel::set_target_param(d, code);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local code=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let code = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int() as u32;
    // local class=Duel.GetMetatable(code)
    let class = crate::script_info(code);
    // if class==nil or class.listed_names==nil then return end
    let listed = class.listed_names;
    if listed.is_empty() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,class,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter(d, x, listed, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // Duel.SpecialSummon(tc,0,tp,tp,true,true,POS_FACEUP)
        duel::special_summon(d, tc, 0, a.tp, a.tp, true, true, POS_FACEUP as u8, None);
        // if tc:GetPreviousLocation()==LOCATION_DECK then Duel.ShuffleDeck(tp) end
        if card::get_previous_location(d, tc) == LOCATION_DECK {
            duel::shuffle_deck(d, a.tp);
        }
    }
}
