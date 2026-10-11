//! Barian's Hope
use super::prelude::*;

pub const ID: u32 = 75518330;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_UMBRAL_HORROR, SET_MASQUERADE, SET_NUMBER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Umbral Horror" monster or "Masquerade" Xyz Monster from your hand or GY, then you can make the Levels of all "Umbral Horror" monsters you control become the Level of 1 "Umbral Horror" monster in your field or GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_LVCHANGE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_LVCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You can banish this card from your GY, then target 1 Xyz Monster you control; attach 1 "Number" Xyz Monster from your Extra Deck or GY to it
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_LEAVE_GRAVE)
    effect::set_category(d, e2, CATEGORY_LEAVE_GRAVE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.attachtg)
    effect::set_target(d, e2, Some(cost_target(attachtg)));
    // e2:SetOperation(s.attachop)
    effect::set_operation(d, e2, Some(operation(attachop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    (card::is_set_card(d, c, &[SET_UMBRAL_HORROR as u16]) || (card::is_set_card(d, c, &[SET_MASQUERADE as u16]) && card::is_xyz_monster(d, c)))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_LVCHANGE,nil,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_LVCHANGE, None, 1, a.tp, 0);
    true
}
/// `s.umbralfilter(c,umbral_group)`
fn umbralfilter(d: &mut Duel, c: CardId, umbral_group: &CardSet) -> bool {
    card::is_set_card(d, c, &[SET_UMBRAL_HORROR as u16])
        && card::has_level(d, c)
        && card::is_faceup(d, c)
        && {
            // umbral_group:IsExists(aux.NOT(Card.IsLevel),1,c,c:GetLevel())
            let lv = card::get_level(d, c);
            group::is_exists(d, umbral_group, &|d, x| !card::is_level(d, x, &[lv]), 1, c.into())
        }
}
/// `s.spop`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp))),
        a.tp,
        LOCATION_HAND | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)>0 then
    if !g.is_empty() && duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        // local umbral_group=Duel.GetMatchingGroup(aux.AND(Card.IsSetCard,Card.HasLevel,Card.IsFaceup),tp,LOCATION_MZONE,0,nil,SET_UMBRAL_HORROR)
        let umbral_group = duel::get_matching_group(
            d,
            Some(&|d, c| card::is_set_card(d, c, &[SET_UMBRAL_HORROR as u16]) && card::has_level(d, c) && card::is_faceup(d, c)),
            a.tp,
            LOCATION_MZONE,
            0,
            NO_EX,
        );
        // if #umbral_group>0 and Duel.IsExistingMatchingCard(s.umbralfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil,umbral_group)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if !umbral_group.is_empty()
            && duel::is_existing_matching_card(d, Some(&|d, c| umbralfilter(d, c, &umbral_group)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
        {
            // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,3))
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 3));
            // local sc=Duel.SelectMatchingCard(tp,s.umbralfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil,umbral_group):GetFirst()
            let sg = duel::select_matching_card(d, a.tp, Some(&|d, c| umbralfilter(d, c, &umbral_group)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
            // if not sc then return end
            let sc = match group::get_first(d, &sg) {
                Some(sc) => sc,
                None => return,
            };
            // Duel.HintSelection(sc)
            duel::hint_selection(d, sc, true);
            // local c=e:GetHandler()
            let c = effect::get_handler(d, a.e).nil();
            // local lv=sc:GetLevel()
            let lv = card::get_level(d, sc);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // for tc in umbral_group:Iter() do
            for tc in group::iter(d, &umbral_group) {
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
                // e1:SetCode(EFFECT_CHANGE_LEVEL_FINAL)
                effect::set_code(d, e1, EFFECT_CHANGE_LEVEL_FINAL);
                // e1:SetValue(lv)
                effect::set_value(d, e1, lv as i64);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // tc:RegisterEffect(e1)
                card::register_effect(d, tc, e1, false);
            }
        }
    }
}
/// `s.xyzfilter(c,tp)`
fn xyzfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_xyz_monster(d, c) && card::is_faceup(d, c) && duel::is_existing_matching_card(d, Some(&|d, x| attachfilter(d, x, tp, c)), tp, LOCATION_EXTRA | LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.attachfilter(c,tp,xyzc)`
fn attachfilter(d: &mut Duel, c: CardId, tp: u8, xyzc: CardId) -> bool {
    card::is_set_card(d, c, &[SET_NUMBER as u16]) && card::is_xyz_monster(d, c) && card::is_can_be_xyz_material(d, c, Some(xyzc), Some(tp), Some(REASON_EFFECT))
}
/// `s.attachtg`
fn attachtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.xyzfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && xyzfilter(d, chkc, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.xyzfilter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| xyzfilter(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.xyzfilter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    duel::select_target(d, a.tp, Some(&|d, c| xyzfilter(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, a.tp, 0);
    true
}
/// `s.attachop`
fn attachop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsXyzMonster() and not tc:IsImmuneToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_xyz_monster(d, tc) && !card::is_immune_to_effect(d, tc, a.e) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_XMATERIAL as u64);
        // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.attachfilter),tp,LOCATION_EXTRA|LOCATION_GRAVE,0,1,1,nil,tp,tc):GetFirst()
        let sg = duel::select_matching_card(
            d,
            a.tp,
            Some(&aux::necro_valley_filter(|d, c| attachfilter(d, c, a.tp, tc))),
            a.tp,
            LOCATION_EXTRA | LOCATION_GRAVE,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // if not sc then return end
        let sc = match group::get_first(d, &sg) {
            Some(sc) => sc,
            None => return,
        };
        // if sc:IsLocation(LOCATION_EXTRA) then
        if card::is_location(d, sc, LOCATION_EXTRA) {
            // Duel.ConfirmCards(1-tp,sc)
            duel::confirm_cards(d, 1 - a.tp, CardSet::from_card(sc));
        } else {
            // Duel.HintSelection(sc)
            duel::hint_selection(d, sc, true);
        }
        // Duel.Overlay(tc,sc)
        duel::overlay(d, tc, CardSet::from_card(sc), false);
    }
}
