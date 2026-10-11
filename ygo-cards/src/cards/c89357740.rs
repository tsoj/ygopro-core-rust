//! Union Pilot
//! ユニオン・パイロット
use super::prelude::*;

pub const ID: u32 = 89357740;
pub const INFO: ScriptInfo = ScriptInfo { listed_card_types: &[TYPE_UNION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Union procedure
    // aux.AddUnionProcedure(c,aux.FilterBoolFunction(Card.IsType,TYPE_EFFECT))
    aux::add_union_procedure(d, c, Some(std::sync::Arc::new(|d: &mut Duel, cc: CardId| card::is_type(d, cc, TYPE_EFFECT))), false, None);
    // Equip 1 banished Union monster to an appropriate monster you control and Special Summon this card from your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_EQUIP + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsEquipCard() end)
    effect::set_condition(
        d,
        e1,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::is_equip_card(d, h)
        })),
    );
    // e1:SetCost(s.eqcost)
    effect::set_cost(d, e1, Some(cost_target(eqcost)));
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.eqcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn eqcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHandAsCost() end
    if a.chk == Some(0) {
        return card::is_able_to_hand_as_cost(d, c);
    }
    // Duel.SendtoHand(c,nil,REASON_COST)
    duel::send_to_hand(d, c, None, REASON_COST, None);
    // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_CHAIN,0,1)
    card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD | RESET_CHAIN, 0, 1, 0, 0);
    true
}
/// `s.equnionfilter(c,tp)`
fn equnionfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsType(TYPE_UNION) and c:IsFaceup()
    //     and Duel.IsExistingMatchingCard(s.eqtargetfilter,tp,LOCATION_MZONE,0,1,nil,c)
    card::is_type(d, c, TYPE_UNION)
        && card::is_faceup(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, cc| eqtargetfilter(d, cc, c)), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.eqtargetfilter(c,ec)`
fn eqtargetfilter(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    // return ec:CheckUnionTarget(c) and aux.CheckUnionEquip(ec,c)
    card::check_union_target(d, ec, c) && aux::check_union_equip(d, ec, c)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.equnionfilter,tp,LOCATION_REMOVED,LOCATION_REMOVED,1,nil,tp)
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,e:GetHandler():GetOriginalType(),2100,1000,5,RACE_MACHINE,ATTRIBUTE_LIGHT) end
    if a.chk == Some(0) {
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return false;
        }
        if !duel::is_existing_matching_card(d, Some(&|d, c| equnionfilter(d, c, a.tp)), a.tp, LOCATION_REMOVED, LOCATION_REMOVED, 1, NO_EX) {
            return false;
        }
        let h = effect::get_handler(d, a.e).nil();
        let orig_type = card::get_original_type(d, h);
        return duel::is_player_can_special_summon_monster(
            d,
            a.tp,
            ID,
            duel::MonsterData {
                setcodes: Some(vec![0]),
                type_: Some(orig_type),
                attack: Some(2100),
                defense: Some(1000),
                level: Some(5),
                race: Some(RACE_MACHINE as u64),
                attribute: Some(ATTRIBUTE_LIGHT),
            },
            None,
            None,
            0,
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,nil,1,tp,LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, a.tp, LOCATION_REMOVED as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // local ec=Duel.SelectMatchingCard(tp,s.equnionfilter,tp,LOCATION_REMOVED,LOCATION_REMOVED,1,1,nil,tp):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&|d, c| equnionfilter(d, c, a.tp)), a.tp, LOCATION_REMOVED, LOCATION_REMOVED, 1, 1, false, NO_EX).nil();
    let Some(ec) = group::get_first(d, &sel) else {
        return;
    };
    // Duel.HintSelection(ec)
    duel::hint_selection(d, CardSet::from_card(ec), true);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.eqtargetfilter,tp,LOCATION_MZONE,0,1,1,nil,ec):GetFirst()
    let sel2 = duel::select_matching_card(d, a.tp, Some(&|d, c| eqtargetfilter(d, c, ec)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let Some(tc) = group::get_first(d, &sel2) else {
        return;
    };
    // Duel.HintSelection(tc)
    duel::hint_selection(d, CardSet::from_card(tc), true);
    // if Duel.Equip(tp,ec,tc) then
    if duel::equip(d, a.tp, ec, tc, true, false) {
        // aux.SetUnionState(ec)
        aux::set_union_state(d, ec);
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // if c:HasFlagEffect(id) then
        if card::has_flag_effect(d, c, ID, None) {
            // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
