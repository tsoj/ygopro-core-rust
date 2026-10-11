//! Void Feast
use super::prelude::*;

pub const ID: u32 = 31548814;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VOID, SET_INFERNOID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon up to 3 "Infernoid" monsters from the Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.costfilter(c,tp)`
fn costfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_VOID) and c:IsSpellTrap() and c:IsAbleToGraveAsCost()
    //     and (c:IsFaceup() or c:IsLocation(LOCATION_HAND)) and Duel.GetMZoneCount(tp,c)>0
    card::is_set_card(d, c, &[SET_VOID as u16])
        && card::is_spell_trap(d, c)
        && card::is_able_to_grave_as_cost(d, c)
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND))
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
/// `s.cost(e,tp,...,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // local h=e:GetHandler()
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,e:GetHandler(),tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| costfilter(d, c, tp)), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,1,e:GetHandler(),tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| costfilter(d, c, tp)), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, 1, false, h.into()).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_INFERNOID) and c:IsMonster() and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_set_card(d, c, &[SET_INFERNOID as u16]) && card::is_monster(d, c) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
/// `function(c) return c:GetLevel() end` (the `Card.GetLevel` value function)
fn level_of(d: &mut Duel, c: CardId) -> i64 {
    card::get_level(d, c) as i64
}
/// `math.min(Duel.GetLocationCount(tp,LOCATION_MZONE),3)` with the Blue-Eyes Spirit rule
fn ft_of(d: &mut Duel, tp: u8) -> i32 {
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0.min(3);
    // if ft>1 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if ft > 1 && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    ft
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        // local ft=math.min(...)
        let ft = ft_of(d, tp);
        // if e:GetLabel()==0 and ft<=0 then return false end
        if effect::get_label(d, a.e) == 0 && ft <= 0 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp)
        let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, NO_EX);
        // return #g>0 and g:CheckWithSumEqual(Card.GetLevel,8,1,ft)
        return !g.is_empty() && group::check_with_sum_equal(d, &g, &level_of, 8, 1, ft).0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ft=math.min(Duel.GetLocationCount(tp,LOCATION_MZONE),3)
    // if ft>1 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    let ft = ft_of(d, tp);
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, NO_EX);
    // if g:CheckWithSumEqual(Card.GetLevel,8,1,ft) then
    if group::check_with_sum_equal(d, &g, &level_of, 8, 1, ft).0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=g:SelectWithSumEqual(tp,Card.GetLevel,8,1,ft)
        let sg = group::select_with_sum_equal(d, &g, tp, &level_of, 8, 1, ft).nil();
        // if #sg>0 then Duel.SpecialSummon(sg,0,tp,tp,true,false,POS_FACEUP) end
        if !sg.is_empty() {
            duel::special_summon(d, sg, 0, tp, tp, true, false, POS_FACEUP as u8, None);
        }
    }
}
