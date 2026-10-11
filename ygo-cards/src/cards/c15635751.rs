//! Camelot, Realm of Noble Knights and Noble Arms
use super::prelude::*;

pub const ID: u32 = 15635751;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[55742055],
    listed_series: &[SET_NOBLE_KNIGHT, SET_NOBLE_ARMS, SET_ARTORIGUS],
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
    // Destruction replacement for "Noble Knight" cards
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.reptg)
    effect::set_target(d, e2, Some(cost_target(reptg)));
    // e2:SetValue(s.repval)
    effect::set_value_fn(d, e2, effect_card_fn(|d, e, c| {
        let hp = effect::get_handler_player(d, e);
        repfilter(d, c, hp)
    }));
    // e2:SetOperation(s.repop)
    effect::set_operation(d, e2, Some(operation(repop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Banish this card, place 1 "Noble Knights of the Round Table" in your Field Zone and Special summon or add to hand 1 "Artorigus"/"Noble Arms" card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON+CATEGORY_SEARCH+CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON + CATEGORY_SEARCH + CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTarget(s.tofieldtg)
    effect::set_target(d, e3, Some(cost_target(tofieldtg)));
    // e3:SetOperation(s.tofieldop)
    effect::set_operation(d, e3, Some(operation(tofieldop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsControler(tp) and c:IsLocation(LOCATION_ONFIELD) and c:IsSetCard(SET_NOBLE_KNIGHT)
    //     and c:IsReason(REASON_BATTLE|REASON_EFFECT) and not c:IsReason(REASON_REPLACE)
    card::is_faceup(d, c)
        && card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_ONFIELD)
        && card::is_set_card(d, c, &[SET_NOBLE_KNIGHT as u16])
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && !card::is_reason(d, c, REASON_REPLACE)
}
/// `s.desfilter(c,e)`
fn desfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_EQUIP) and c:IsDestructable(e)
    //     and not c:IsStatus(STATUS_DESTROY_CONFIRMED|STATUS_BATTLE_DESTROYED)
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_EQUIP)
        && card::is_destructable(d, c, Some(e))
        && !card::is_status(d, c, STATUS_DESTROY_CONFIRMED | STATUS_BATTLE_DESTROYED)
}
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return eg:IsExists(s.repfilter,1,nil,tp)
    //     and Duel.IsExistingMatchingCard(s.desfilter,tp,LOCATION_ONFIELD,0,1,nil,e) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return group::is_exists(d, &eg, &|d, c| repfilter(d, c, tp), 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| desfilter(d, c, a.e)), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // if Duel.SelectEffectYesNo(tp,e:GetHandler(),96) then
    let h = effect::get_handler(d, a.e).nil();
    if duel::select_effect_yes_no(d, tp, h, Some(96)) {
        // Duel.Hint(HINT_CARD,0,id)
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESREPLACE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESREPLACE as u64);
        // local tc=Duel.SelectMatchingCard(tp,s.desfilter,tp,LOCATION_ONFIELD,0,1,1,nil,e):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&|d, c| desfilter(d, c, a.e)), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &g).nil();
        // Duel.HintSelection(tc,true)
        duel::hint_selection(d, tc, true);
        // e:SetLabelObject(tc)
        effect::set_label_object(d, a.e, Value::Card(tc));
        // tc:SetStatus(STATUS_DESTROY_CONFIRMED,true)
        card::set_status(d, tc, STATUS_DESTROY_CONFIRMED, true);
        // return true
        return true;
    }
    // return false
    false
}
fn repop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // tc:SetStatus(STATUS_DESTROY_CONFIRMED,false)
    card::set_status(d, tc, STATUS_DESTROY_CONFIRMED, false);
    // Duel.Destroy(tc,REASON_EFFECT|REASON_REPLACE)
    duel::destroy(d, tc, REASON_EFFECT | REASON_REPLACE, None, None);
}
/// `s.tofieldfilter(c)`
fn tofieldfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(55742055) and not c:IsForbidden()
    card::is_code(d, c, &[55742055]) && !card::is_forbidden(d, c)
}
fn tofieldtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemove() and Duel.IsExistingMatchingCard(s.tofieldfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, c, None, None, None)
            && duel::is_existing_matching_card(d, Some(&tofieldfilter), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(c)), 1, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.cfilter(c,e,tp,ft)`
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    // return (c:IsSetCard(SET_NOBLE_ARMS) and c:IsAbleToHand())
    //     or (ft>0 and c:IsSetCard(SET_ARTORIGUS) and c:IsCanBeSpecialSummoned(e,0,tp,false,false))
    (card::is_set_card(d, c, &[SET_NOBLE_ARMS as u16]) && card::is_able_to_hand(d, c, None))
        || (ft > 0
            && card::is_set_card(d, c, &[SET_ARTORIGUS as u16])
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None))
}
/// `s.returnop(rg,e,tp,eg,ep,ev,re,r,rp)`: the delayed return of Camelot to the Field Zone
fn returnop() -> aux::DelayedFn {
    std::sync::Arc::new(|d, _rg, a| {
        let tp = a.tp;
        // local fc=Duel.GetFieldCard(tp,LOCATION_FZONE,0)
        // if fc then Duel.SendtoGrave(fc,REASON_RULE) Duel.BreakEffect() end
        if let Some(fc) = duel::get_field_card(d, tp, LOCATION_FZONE, 0) {
            duel::send_to_grave(d, fc, REASON_RULE, None, None);
            duel::break_effect(d);
        }
        // Duel.MoveToField(e:GetHandler(),tp,tp,LOCATION_FZONE,POS_FACEUP,true)
        let h = effect::get_handler(d, a.e).nil();
        duel::move_to_field(d, h, tp, tp, LOCATION_FZONE as u16, POS_FACEUP as u8, true, None);
        true
    })
}
fn tofieldop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and aux.RemoveUntil(c,nil,REASON_EFFECT,PHASE_STANDBY,id,e,tp,s.returnop) and c:IsLocation(LOCATION_REMOVED) then
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    if aux::remove_until(d, c, None, REASON_EFFECT, PHASE_STANDBY, ID, a.e, tp, Some(returnop()), None, None, None, None, None).is_none() {
        return;
    }
    if !card::is_location(d, c, LOCATION_REMOVED) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
    // local tc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.tofieldfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil):GetFirst()
    let nv = aux::necro_valley_filter(tofieldfilter);
    let g = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if not tc or not Duel.MoveToField(tc,tp,tp,LOCATION_FZONE,POS_FACEUP,true) then return end
    let Some(tc) = tc else { return };
    if !duel::move_to_field(d, tc, tp, tp, LOCATION_FZONE as u16, POS_FACEUP as u8, true, None) {
        return;
    }
    // Special Summon 1 "Artorigus" monster, or add to your hand 1 "Noble Arms" card, from your Deck or GY
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let (ft, _) = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None);
    // if Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.cfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp,ft)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    let nvc = aux::necro_valley_filter(|d, x| cfilter(d, x, a.e, tp, ft));
    if duel::is_existing_matching_card(d, Some(&nvc), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
        // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.cfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp,ft):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&nvc), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if not sc then return end
        let Some(sc) = group::get_first(d, &g) else { return };
        // local b1=ft>0 and sc:IsSetCard(SET_ARTORIGUS) and sc:IsCanBeSpecialSummoned(e,0,tp,false,false)
        let b1 = ft > 0
            && card::is_set_card(d, sc, &[SET_ARTORIGUS as u16])
            && card::is_can_be_special_summoned(d, sc, a.e, 0, tp, false, false, None, None, None);
        // local b2=sc:IsSetCard(SET_NOBLE_ARMS) and sc:IsAbleToHand()
        let b2 = card::is_set_card(d, sc, &[SET_NOBLE_ARMS as u16]) && card::is_able_to_hand(d, sc, None);
        // Duel.BreakEffect()
        duel::break_effect(d);
        if b1 && b2 {
            // aux.ToHandOrElse(sc,tp,
            //     function(sc) return ft>0 and sc:IsCanBeSpecialSummoned(e,0,tp,false,false) end,
            //     function(sc) Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP) end,
            //     aux.Stringid(id,3))
            let check = |d: &mut Duel, x: CardId| ft > 0 && card::is_can_be_special_summoned(d, x, a.e, 0, tp, false, false, None, None, None);
            let oper = |d: &mut Duel, g: &CardSet| {
                duel::special_summon(d, g.clone(), 0, tp, tp, false, false, POS_FACEUP as u8, None);
                0
            };
            aux::to_hand_or_else(d, sc, tp, Some(&check), Some(&oper), Some(aux::stringid(ID, 3)));
        } else if b1 {
            // Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        } else if b2 {
            // Duel.SendtoHand(sc,nil,REASON_EFFECT)
            duel::send_to_hand(d, sc, None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,sc)
            duel::confirm_cards(d, 1 - tp, sc);
        }
    }
}
