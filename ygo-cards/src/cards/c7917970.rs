//! Dragunity Divine Wind
//! 神風のドラグニティ
use super::prelude::*;

pub const ID: u32 = 7917970;

/// `CARD_DRAGON_RAVINE` (local in the script)
const CARD_DRAGON_RAVINE: u32 = 62265044;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_DRAGON_RAVINE], listed_series: &[SET_DRAGUNITY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Apply 1 of these effects, or if "Dragon Ravine" is in your GY, you can apply any of them, in sequence
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON+CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON + CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1, LOCATION_FZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DRAGON|RACE_WINGEDBEAST) and c:IsAbleToHand()
    card::is_race(d, c, RACE_DRAGON | RACE_WINGEDBEAST) && card::is_able_to_hand(d, c, None)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_DRAGUNITY) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_DRAGUNITY as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local b1=Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_GRAVE,0,1,nil)
        let b1 = duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
        // local b2=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp)
        let b2 = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
        // return b1 or b2
        return b1 || b2;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_EQUIP,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_EQUIP, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.eqfilter(c,tp)`
fn eqfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRace(RACE_DRAGON) and c:IsSetCard(SET_DRAGUNITY) and c:CheckUniqueOnField(tp) and not c:IsForbidden()
    card::is_race(d, c, RACE_DRAGON) && card::is_set_card(d, c, &[SET_DRAGUNITY as u16]) && card::check_unique_on_field(d, c, tp, None, None) && !card::is_forbidden(d, c)
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local b1=Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.thfilter),tp,LOCATION_GRAVE,0,1,nil)
    let nv = aux::necro_valley_filter(thfilter);
    let b1 = duel::is_existing_matching_card(d, Some(&nv), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    // local b2=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp)
    let mut b2 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    // local ravine_chk=Duel.IsExistingMatchingCard(Card.IsCode,tp,LOCATION_GRAVE,0,1,nil,CARD_DRAGON_RAVINE)
    let ravine_chk = duel::is_existing_matching_card(d, Some(&|d, c| card::is_code(d, c, &[CARD_DRAGON_RAVINE])), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    // local op=nil
    let mut op = None;
    // if not ravine_chk then
    if !ravine_chk {
        // op=Duel.SelectEffect(tp,
        //   {b1,aux.Stringid(id,1)},
        //   {b2,aux.Stringid(id,2)})
        op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    }
    // local breakeffect=false
    let mut breakeffect = false;
    // if (op and op==1) or (ravine_chk and b1 and (not b2 or Duel.SelectYesNo(tp,aux.Stringid(id,3)))) then
    if op == Some(1) || (ravine_chk && b1 && (!b2 || duel::select_yes_no(d, tp, aux::stringid(ID, 3)))) {
        // Add 1 Dragon or Winged Beast monster from your GY to your hand
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_GRAVE,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g, None, REASON_EFFECT, None);
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, tp);
            // b2=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
            //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp)
            b2 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
        }
        // breakeffect=true
        breakeffect = true;
    }
    // if (op and op==2) or (ravine_chk and b2 and (not breakeffect or Duel.SelectYesNo(tp,aux.Stringid(id,4)))) then
    if op == Some(2) || (ravine_chk && b2 && (!breakeffect || duel::select_yes_no(d, tp, aux::stringid(ID, 4)))) {
        // Special Summon 1 "Dragunity" monster from your hand, then you can equip 1 Dragon "Dragunity" monster from your Deck to it as an Equip Spell
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp):GetFirst()
        let sel = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sel);
        // if sc then
        if let Some(sc) = sc {
            // if breakeffect then Duel.BreakEffect() end
            if breakeffect {
                duel::break_effect(d);
            }
            // if Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)>0 and Duel.GetLocationCount(tp,LOCATION_SZONE)>0
            //   and Duel.IsExistingMatchingCard(s.eqfilter,tp,LOCATION_DECK,0,1,nil,tp)
            //   and Duel.SelectYesNo(tp,aux.Stringid(id,5)) then
            if duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
                && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
                && duel::is_existing_matching_card(d, Some(&|d, c| eqfilter(d, c, tp)), tp, LOCATION_DECK, 0, 1, NO_EX)
                && duel::select_yes_no(d, tp, aux::stringid(ID, 5))
            {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
                // local ec=Duel.SelectMatchingCard(tp,s.eqfilter,tp,LOCATION_DECK,0,1,1,nil,tp):GetFirst()
                let sel2 = duel::select_matching_card(d, tp, Some(&|d, c| eqfilter(d, c, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
                let ec = group::get_first(d, &sel2);
                // if not ec then return end
                let Some(ec) = ec else { return };
                // Duel.BreakEffect()
                duel::break_effect(d);
                // if Duel.Equip(tp,ec,sc) then
                if duel::equip(d, tp, ec, sc, true, false) {
                    // Equip limit
                    // local e0=Effect.CreateEffect(e:GetHandler())
                    let h = effect::get_handler(d, e).nil();
                    let e0 = effect::create_effect(d, h);
                    // e0:SetType(EFFECT_TYPE_SINGLE)
                    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
                    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
                    // e0:SetCode(EFFECT_EQUIP_LIMIT)
                    effect::set_code(d, e0, EFFECT_EQUIP_LIMIT);
                    // e0:SetValue(function(e,c) return c==sc end)
                    effect::set_value_fn(d, e0, lua_fn(move |_d: &mut Duel, a: Args| -> bool { a.card(1) == Some(sc) }));
                    // e0:SetReset(RESET_EVENT|RESETS_STANDARD)
                    effect::set_reset(d, e0, RESET_EVENT | RESETS_STANDARD, 1);
                    // ec:RegisterEffect(e0)
                    card::register_effect(d, ec, e0, false);
                }
            }
        }
    }
}
