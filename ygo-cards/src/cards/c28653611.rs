//! ヘル・パニッシャー / Infernal Punisher
use super::prelude::*;

pub const ID: u32 = 28653611;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.effcost)
    effect::set_cost(d, e1, Some(cost_target(effcost)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WATER) and c:IsRace(RACE_DINOSAUR) and c:IsAbleToHand()
    card::is_attribute(d, c, ATTRIBUTE_WATER) && card::is_race(d, c, RACE_DINOSAUR) && card::is_able_to_hand(d, c, None)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelAbove(6) and c:IsAttribute(ATTRIBUTE_FIRE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_above(d, c, 6) && card::is_attribute(d, c, ATTRIBUTE_FIRE) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.effcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(-100)
    effect::set_label(d, a.e, &[-100]);
    // local b1=not Duel.HasFlagEffect(tp,id)
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
    let b1 = !duel::has_flag_effect(d, a.tp, ID, None) && duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    // local b2=not Duel.HasFlagEffect(tp,id+1)
    //     and Duel.CheckReleaseGroupCost(tp,nil,1,false,aux.ReleaseCheckMMZ,nil)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp)
    let b2 = !duel::has_flag_effect(d, a.tp, ID + 1, None)
        && duel::check_release_group_cost(d, a.tp, None, 1, 1, false, Some(&aux::release_check_mmz), NO_EX)
        && duel::is_existing_matching_card(
            d,
            Some(&|d, c| spfilter(d, c, a.e, a.tp)),
            a.tp,
            LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
            0,
            1,
            NO_EX,
        );
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).unwrap_or(0);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    // if op==2 then
    if op == 2 {
        // local g=Duel.SelectReleaseGroupCost(tp,nil,1,1,false,aux.ReleaseCheckMMZ,nil)
        let g = duel::select_release_group_cost(d, a.tp, None, 1, 1, false, Some(&aux::release_check_mmz), NO_EX);
        // Duel.Release(g,REASON_COST)
        duel::release(d, g, REASON_COST, None);
    }
    true
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // local cost_skip=e:GetLabel()~=-100
    let mut cost_skip = effect::get_label(d, a.e) != -100;
    // local b1=(cost_skip or not Duel.HasFlagEffect(tp,id))
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
    let b1 = (cost_skip || !duel::has_flag_effect(d, a.tp, ID, None))
        && duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    // local b2=(cost_skip or not Duel.HasFlagEffect(tp,id+1))
    //     and (not cost_skip or Duel.GetLocationCount(tp,LOCATION_MZONE)>0)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp)
    let b2 = (cost_skip || !duel::has_flag_effect(d, a.tp, ID + 1, None))
        && (!cost_skip || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0)
        && duel::is_existing_matching_card(
            d,
            Some(&|d, c| spfilter(d, c, a.e, a.tp)),
            a.tp,
            LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
            0,
            1,
            NO_EX,
        );
    // if chk==0 then e:SetLabel(0) return b1 or b2 end
    if a.chk == Some(0) {
        effect::set_label(d, a.e, &[0]);
        return b1 || b2;
    }
    // local op=e:GetLabel()
    let mut op = effect::get_label(d, a.e);
    // if op==0 then
    if op == 0 {
        // cost_skip=true
        cost_skip = true;
        // op=Duel.SelectEffect(tp,
        //     {b1,aux.Stringid(id,1)},
        //     {b2,aux.Stringid(id,2)})
        op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).unwrap_or(0) as i64;
    } else {
        // cost_skip=false
        cost_skip = false;
    }
    // e:SetLabel(0)
    effect::set_label(d, a.e, &[0]);
    // Duel.SetTargetParam(op)
    duel::set_target_param(d, op as u32);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
        // if not cost_skip then Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1) end
        if !cost_skip {
            duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
        duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // if not cost_skip then Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1) end
        if !cost_skip {
            duel::register_flag_effect(d, a.tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local op=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let op = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    // if op==1 then
    if op == 1 {
        // Add 1 WATER Dinosaur monster from your Deck to your hand, then if your opponent controls a monster, you can Special Summon the added monster
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
        let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if sc and Duel.SendtoHand(sc,nil,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_HAND) then
        if let Some(sc) = group::get_first(d, &g) {
            if duel::send_to_hand(d, sc, None, REASON_EFFECT, None) > 0 && card::is_location(d, sc, LOCATION_HAND) {
                // Duel.ConfirmCards(1-tp,sc)
                duel::confirm_cards(d, 1 - a.tp, sc);
                // if Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
                //     and sc:IsCanBeSpecialSummoned(e,0,tp,false,false) and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
                if duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE) > 0
                    && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                    && card::is_can_be_special_summoned(d, sc, a.e, 0, a.tp, false, false, None, None, None)
                    && duel::select_yes_no(d, a.tp, aux::stringid(ID, 3))
                {
                    // Duel.BreakEffect()
                    duel::break_effect(d);
                    // Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)
                    duel::special_summon(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
                }
            }
        }
    // elseif op==2 then
    } else if op == 2 {
        // Special Summon 1 Level 6 or higher FIRE monster from your hand, Deck, or GY, but if you Special Summoned a non-Fiend monster by this effect, negate its effects
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp):GetFirst()
        let nv = aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp));
        let g = duel::select_matching_card(d, a.tp, Some(&nv), a.tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if sc and Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)>0 and not sc:IsRace(RACE_FIEND) then
        if let Some(sc) = group::get_first(d, &g) {
            if duel::special_summon(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 && !card::is_race(d, sc, RACE_FIEND) {
                // sc:NegateEffects(e:GetHandler())
                let h = effect::get_handler(d, a.e).nil();
                card::negate_effects(d, sc, h, None, false, None);
            }
        }
    }
}
