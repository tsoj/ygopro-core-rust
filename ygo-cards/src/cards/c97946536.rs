//! Zektrike Kou-ou
use super::prelude::*;

pub const ID: u32 = 97946536;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_INZEKTOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c,e,tp,ft)`
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    // return c:IsSetCard(SET_INZEKTOR) and c:IsAbleToGraveAsCost() and (c:IsFaceup() or c:IsLocation(LOCATION_HAND))
    //   and (Duel.IsExistingMatchingCard(s.monfilter,tp,LOCATION_DECK,0,1,nil,e,tp,ft,c)
    //   or Duel.IsExistingMatchingCard(s.eqspfilter,tp,LOCATION_DECK,0,1,nil,tp,ft,c))
    card::is_set_card(d, c, &[SET_INZEKTOR as u16])
        && card::is_able_to_grave_as_cost(d, c)
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND))
        && (duel::is_existing_matching_card(d, Some(&|d, x| monfilter(d, x, e, tp, ft, Some(c))), tp, LOCATION_DECK, 0, 1, NO_EX)
            || duel::is_existing_matching_card(d, Some(&|d, x| eqspfilter(d, x, tp, ft, Some(c))), tp, LOCATION_DECK, 0, 1, NO_EX))
}
/// `s.monfilter(c,e,tp,ft,sc)`
fn monfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32, sc: Option<CardId>) -> bool {
    // return c:IsSetCard(SET_INZEKTOR) and c:IsMonster() and (s.monspfilter(c,e,tp,sc) or s.moneqfilter(c,tp,ft,sc))
    card::is_set_card(d, c, &[SET_INZEKTOR as u16])
        && card::is_monster(d, c)
        && (monspfilter(d, c, e, tp, sc) || moneqfilter(d, c, tp, ft, sc))
}
/// `Duel.GetMZoneCount(tp,sc)>0` with `sc` possibly nil
fn mzone_count(d: &mut Duel, tp: u8, sc: Option<CardId>) -> i32 {
    duel::get_m_zone_count(d, tp, ex_of(sc), None, None, None).0
}
/// `(sc and sc:IsLocation(LOCATION_SZONE) and sc:GetSequence()<5)` with `sc` possibly nil
fn sc_in_szone_low(d: &mut Duel, sc: Option<CardId>) -> bool {
    match sc {
        Some(s) => card::is_location(d, s, LOCATION_SZONE) && card::get_sequence(d, s) < 5,
        None => false,
    }
}
/// `Duel.IsExistingMatchingCard(...,sc)`: `sc` as the excluded card (nil = none)
fn ex_of(sc: Option<CardId>) -> Exclude<'static> {
    sc.into()
}
/// `s.monspfilter(c,e,tp,sc)`
fn monspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, sc: Option<CardId>) -> bool {
    // return Duel.GetMZoneCount(tp,sc)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    mzone_count(d, tp, sc) > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.moneqfilter(c,tp,ft,sc)`
fn moneqfilter(d: &mut Duel, c: CardId, tp: u8, ft: i32, sc: Option<CardId>) -> bool {
    // return (ft>0 or (sc and sc:IsLocation(LOCATION_SZONE) and sc:GetSequence()<5)) and not c:IsForbidden()
    //   and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_INZEKTOR),tp,LOCATION_MZONE,0,1,sc)
    (ft > 0 || sc_in_szone_low(d, sc))
        && !card::is_forbidden(d, c)
        && duel::is_existing_matching_card(d, Some(&faceup_inzektor), tp, LOCATION_MZONE, 0, 1, ex_of(sc))
}
/// `aux.FaceupFilter(Card.IsSetCard,SET_INZEKTOR)`
fn faceup_inzektor(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_INZEKTOR as u16])
}
/// `s.eqspfilter(c,tp,ft,sc)`
fn eqspfilter(d: &mut Duel, c: CardId, tp: u8, ft: i32, sc: Option<CardId>) -> bool {
    // return c:IsSetCard(SET_INZEKTOR) and c:IsType(TYPE_EQUIP) and c:IsSpell()
    //   and (ft>0 or (sc and sc:IsLocation(LOCATION_SZONE) and sc:GetSequence()<5))
    //   and Duel.IsExistingMatchingCard(s.eqfilter,tp,LOCATION_MZONE,0,1,sc,c)
    card::is_set_card(d, c, &[SET_INZEKTOR as u16])
        && card::is_type(d, c, TYPE_EQUIP)
        && card::is_spell(d, c)
        && (ft > 0 || sc_in_szone_low(d, sc))
        && duel::is_existing_matching_card(d, Some(&|d, x| eqfilter(d, x, c)), tp, LOCATION_MZONE, 0, 1, ex_of(sc))
}
/// `s.eqfilter(c,ec)`
fn eqfilter(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_INZEKTOR) and ec:CheckEquipTarget(c)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_INZEKTOR as u16]) && card::check_equip_target(d, ec, c)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0;
    // if c:IsLocation(LOCATION_HAND) then ft=ft-1 end
    if card::is_location(d, c, LOCATION_HAND) {
        ft -= 1;
    }
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,c,e,tp,ft) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| cfilter(d, x, a.e, a.tp, ft)), a.tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,1,c,e,tp,ft)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| cfilter(d, x, a.e, a.tp, ft)), a.tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, 1, false, c.into()).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0;
    // local b1=Duel.IsExistingMatchingCard(s.monfilter,tp,LOCATION_DECK,0,1,nil,e,tp,ft)
    let b1 = duel::is_existing_matching_card(d, Some(&|d, x| monfilter(d, x, a.e, a.tp, ft, None)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    // local b2=Duel.IsExistingMatchingCard(s.eqspfilter,tp,LOCATION_DECK,0,1,nil,tp,ft)
    let b2 = duel::is_existing_matching_card(d, Some(&|d, x| eqspfilter(d, x, a.tp, ft, None)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,0)}, {b2,aux.Stringid(id,1)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 0)), (b2, aux::stringid(ID, 1))]).nil();
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_EQUIP)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON + CATEGORY_EQUIP);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
        duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_EQUIP,nil,1,tp,LOCATION_DECK)
        duel::set_possible_operation_info(d, 0, CATEGORY_EQUIP, None, 1, a.tp, LOCATION_DECK as i32);
    } else {
        // e:SetCategory(CATEGORY_EQUIP)
        effect::set_category(d, a.e, CATEGORY_EQUIP);
        // Duel.SetOperationInfo(0,CATEGORY_EQUIP,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, a.tp, LOCATION_DECK as i32);
    }
    true
}
/// `s.eqlimit`: `function(e,c) return c==e:GetLabelObject() end`
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    effect::get_label_object(d, e).as_card() == Some(c)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0;
    // Special Summon or equip 1 "Inzektor" monster from your Deck
    // if op==1 then
    if op == 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 2));
        // local sc=Duel.SelectMatchingCard(tp,s.monfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,ft):GetFirst()
        let sel = duel::select_matching_card(d, a.tp, Some(&|d, x| monfilter(d, x, a.e, a.tp, ft, None)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if not sc then return end
        let Some(sc) = group::get_first(d, &sel) else {
            return;
        };
        // local sp=s.monspfilter(sc,e,tp)
        let sp = monspfilter(d, sc, a.e, a.tp, None);
        // local eq=s.moneqfilter(sc,tp,ft)
        let eq = moneqfilter(d, sc, a.tp, ft, None);
        // local op=Duel.SelectEffect(tp, {sp,aux.Stringid(id,3)}, {eq,aux.Stringid(id,4)})
        let op2 = duel::select_effect(d, a.tp, &[(sp, aux::stringid(ID, 3)), (eq, aux::stringid(ID, 4))]).nil();
        // if op==1 then
        if op2 == 1 {
            // Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        // elseif op==2 then
        } else if op2 == 2 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
            // local ec=Duel.SelectMatchingCard(tp,aux.FaceupFilter(Card.IsSetCard,SET_INZEKTOR),tp,LOCATION_MZONE,0,1,1,nil):GetFirst()
            let sel2 = duel::select_matching_card(d, a.tp, Some(&faceup_inzektor), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
            // if not ec then return end
            let Some(ec) = group::get_first(d, &sel2) else {
                return;
            };
            // Duel.HintSelection(ec,true)
            duel::hint_selection(d, ec, true);
            // if not Duel.Equip(tp,sc,ec,true) then return end
            if !duel::equip(d, a.tp, sc, ec, true, false) {
                return;
            }
            // local e1=Effect.CreateEffect(e:GetHandler())
            let c = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_EQUIP_LIMIT)
            effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // e1:SetValue(function(e,c) return c==e:GetLabelObject() end)
            effect::set_value_fn(d, e1, effect_card_fn(eqlimit));
            // e1:SetLabelObject(ec)
            effect::set_label_object(d, e1, Value::Card(ec));
            // sc:RegisterEffect(e1)
            card::register_effect(d, sc, e1, false);
        }
    // elseif op==2 and ft>0 then
    } else if op == 2 && ft > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.eqspfilter,tp,LOCATION_DECK,0,1,1,nil,tp,ft):GetFirst()
        let sel = duel::select_matching_card(d, a.tp, Some(&|d, x| eqspfilter(d, x, a.tp, ft, None)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if not sc then return end
        let Some(sc) = group::get_first(d, &sel) else {
            return;
        };
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
        // local ec=Duel.SelectMatchingCard(tp,s.eqfilter,tp,LOCATION_MZONE,0,1,1,nil,sc):GetFirst()
        let sel2 = duel::select_matching_card(d, a.tp, Some(&|d, x| eqfilter(d, x, sc)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        let ec = group::get_first(d, &sel2);
        // Duel.HintSelection(ec,true)
        if let Some(ec) = ec {
            duel::hint_selection(d, ec, true);
        }
        // if not ec then return end
        let Some(ec) = ec else {
            return;
        };
        // Duel.Equip(tp,sc,ec)
        duel::equip(d, a.tp, sc, ec, true, false);
    }
}
