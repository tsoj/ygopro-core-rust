//! Artmage Varnish -Alteration-
use super::prelude::*;

pub const ID: u32 = 74011784;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ARTMAGE], listed_names: &[CARD_MEDIUS_THE_PURE, 74733322, ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Place 1 "Artmage Academic Arcane Arts Acropolis" from your Deck or GY face-up in your Field Zone
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate an attack targeting your "Artmage" monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e2, EVENT_BE_BATTLE_TARGET);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.negcon)
    effect::set_condition(d, e2, Some(condition(negcon)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.negtg)
    effect::set_target(d, e2, Some(cost_target(negtg)));
    // e2:SetOperation(s.negop)
    effect::set_operation(d, e2, Some(operation(negop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.plthfilter(c,tohand_chk)`
fn plthfilter(d: &mut Duel, c: CardId, tohand_chk: bool) -> bool {
    // return (c:IsCode(74733322) and not c:IsForbidden())
    //     or (tohand_chk and c:IsSetCard(SET_ARTMAGE) and c:IsAbleToHand() and c:IsLocation(LOCATION_DECK) and not c:IsCode(id))
    (card::is_code(d, c, &[74733322]) && !card::is_forbidden(d, c))
        || (tohand_chk
            && card::is_set_card(d, c, &[SET_ARTMAGE as u16])
            && card::is_able_to_hand(d, c, None)
            && card::is_location(d, c, LOCATION_DECK)
            && !card::is_code(d, c, &[ID]))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        // local tohand_chk=Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,74733322),tp,LOCATION_ONFIELD,0,1,nil)
        let tohand_chk = duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[74733322])), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
        // return Duel.IsExistingMatchingCard(s.plthfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,tohand_chk)
        return duel::is_existing_matching_card(d, Some(&|d, c| plthfilter(d, c, tohand_chk)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tohand_chk=Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,74733322),tp,LOCATION_ONFIELD,0,1,nil)
    let tohand_chk = duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[74733322])), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    // local hint_desc=tohand_chk and aux.Stringid(id,2) or HINTMSG_TOFIELD
    let hint_desc = if tohand_chk { aux::stringid(ID, 2) } else { HINTMSG_TOFIELD as u64 };
    // Duel.Hint(HINT_SELECTMSG,tp,hint_desc)
    duel::hint(d, HINT_SELECTMSG as u8, tp, hint_desc);
    // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.plthfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,tohand_chk):GetFirst()
    let f = aux::necro_valley_filter(|d, c| plthfilter(d, c, tohand_chk));
    let g = duel::select_matching_card(d, tp, Some(&f), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if not sc then return end
    let Some(sc) = group::get_first(d, &g) else {
        return;
    };
    // if sc:IsCode(74733322) then
    if card::is_code(d, sc, &[74733322]) {
        // if not tohand_chk then
        if !tohand_chk {
            // Duel.MoveToField(sc,tp,tp,LOCATION_FZONE,POS_FACEUP,true)
            duel::move_to_field(d, sc, tp, tp, LOCATION_FZONE as u16, POS_FACEUP as u8, true, None);
        } else {
            // aux.ToHandOrElse(sc,tp,
            //     function() return tohand_chk and not sc:IsForbidden() end,
            //     function() Duel.MoveToField(sc,tp,tp,LOCATION_FZONE,POS_FACEUP,true) end,
            //     aux.Stringid(id,3))
            aux::to_hand_or_else(
                d,
                sc,
                tp,
                Some(&|d, _c| tohand_chk && !card::is_forbidden(d, sc)),
                Some(&|d, _g| {
                    duel::move_to_field(d, sc, tp, tp, LOCATION_FZONE as u16, POS_FACEUP as u8, true, None);
                    0
                }),
                Some(aux::stringid(ID, 3)),
            );
        }
    // else
    } else {
        // Duel.SendtoHand(sc,nil,REASON_EFFECT)
        duel::send_to_hand(d, sc, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sc)
        duel::confirm_cards(d, 1 - tp, sc);
    }
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=Duel.GetAttackTarget()
    // return bc and bc:IsSetCard(SET_ARTMAGE) and bc:IsControler(tp) and bc:IsFaceup()
    let Some(bc) = duel::get_attack_target(d) else {
        return false;
    };
    card::is_set_card(d, bc, &[SET_ARTMAGE as u16]) && card::is_controler(d, bc, a.tp) && card::is_faceup(d, bc)
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_MEDIUS_THE_PURE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[CARD_MEDIUS_THE_PURE]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.NegateAttack() and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,4)) then
    if duel::negate_attack(d)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 4))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
