//! Yummy☆Surprise
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 29369059;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_YUMMY], ..ScriptInfo::DEFAULT };

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
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMING_BATTLE_START|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMING_BATTLE_START | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCost(s.effcost)
    effect::set_cost(d, e1, Some(cost_target(effcost)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.monoppthfilter(c,e,tp)`
fn monoppthfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return ((c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_BEAST) and c:IsControler(tp) and c:IsFaceup()) or c:IsControler(1-tp))
    //     and c:IsCanBeEffectTarget(e) and c:IsAbleToHand()
    ((card::is_attribute(d, c, ATTRIBUTE_LIGHT)
        && card::is_race(d, c, RACE_BEAST as u64)
        && card::is_controler(d, c, tp)
        && card::is_faceup(d, c))
        || card::is_controler(d, c, 1 - tp))
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_able_to_hand(d, c, None)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:FilterCount(Card.IsControler,nil,tp)==2
    (group::filter_count(d, sg, &|d, c| card::is_controler(d, c, tp), NO_EX) == 2, false)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_YUMMY) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_YUMMY as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.fieldspellthfilter(c)`
fn fieldspellthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFieldSpell() and c:IsFaceup() and c:IsAbleToHand()
    card::is_field_spell(d, c) && card::is_faceup(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.plfilter(c)`
fn plfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_YUMMY) and c:IsFieldSpell() and not c:IsForbidden()
    card::is_set_card(d, c, &[SET_YUMMY as u16]) && card::is_field_spell(d, c) && !card::is_forbidden(d, c)
}
/// The three options' availability, shared by the cost and the target (`b1, b2, b3`).
fn options(d: &mut Duel, e: EffectId, tp: u8) -> (bool, bool, bool) {
    // local g=Duel.GetMatchingGroup(s.monoppthfilter,tp,LOCATION_MZONE,LOCATION_ONFIELD,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| monoppthfilter(d, c, e, tp)), tp, LOCATION_MZONE, LOCATION_ONFIELD, NO_EX);
    // local b1=not Duel.HasFlagEffect(tp,id) and #g>=4
    //     and aux.SelectUnselectGroup(g,e,tp,4,4,s.rescon,0)
    let b1 = !duel::has_flag_effect(d, tp, ID, None)
        && g.len() >= 4
        && matches!(
            aux::select_unselect_group(d, &g, Some(e), tp, Some(4), Some(4), Some(&rescon), 0, tp, None, None, None, false),
            Err(true)
        );
    // local b2=not Duel.HasFlagEffect(tp,id+1) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,tp)
    let b2 = !duel::has_flag_effect(d, tp, ID + 1, None)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    // local b3=not Duel.HasFlagEffect(tp,id+2)
    //     and Duel.IsExistingMatchingCard(s.fieldspellthfilter,tp,LOCATION_ONFIELD|LOCATION_GRAVE,0,1,nil)
    let b3 = !duel::has_flag_effect(d, tp, ID + 2, None)
        && duel::is_existing_matching_card(d, Some(&fieldspellthfilter), tp, LOCATION_ONFIELD | LOCATION_GRAVE, 0, 1, NO_EX);
    (b1, b2, b3)
}
/// `s.effcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // e:SetLabel(-100)
    effect::set_label(d, e, &[-100]);
    let (b1, b2, b3) = options(d, e, tp);
    // if chk==0 then return b1 or b2 or b3 end
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    true
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    let e = a.e;
    let tp = a.tp;
    // local cost_skip=e:GetLabel()~=-100
    let cost_skip = effect::get_label(d, e) != -100;
    let (b1, b2, b3) = options(d, e, tp);
    // if chk==0 then e:SetLabel(0) return b1 or b2 or b3 end
    if a.chk == Some(0) {
        effect::set_label(d, e, &[0]);
        return b1 || b2 || b3;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,1)},{b2,aux.Stringid(id,2)},{b3,aux.Stringid(id,3)})
    let op = duel::select_effect(
        d,
        tp,
        &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3))],
    )
    .unwrap_or(0) as i64;
    // e:SetLabel(op)
    effect::set_label(d, e, &[op]);
    if op == 1 {
        // e:SetCategory(CATEGORY_TOHAND)
        effect::set_category(d, e, CATEGORY_TOHAND);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, e, EFFECT_FLAG_CARD_TARGET, 0);
        // if not cost_skip then Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1) end
        if !cost_skip {
            duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
        // local g=Duel.GetMatchingGroup(...) (same as above) -- recomputed for the selection
        let g = duel::get_matching_group(d, Some(&|d, c| monoppthfilter(d, c, e, tp)), tp, LOCATION_MZONE, LOCATION_ONFIELD, NO_EX);
        // local tg=aux.SelectUnselectGroup(g,e,tp,4,4,s.rescon,1,tp,HINTMSG_RTOHAND)
        let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(4), Some(4), Some(&rescon), 1, tp, Some(HINTMSG_RTOHAND as u64), None, None, false)
            .unwrap_or_default();
        // Duel.SetTargetCard(tg)
        duel::set_target_card(d, tg.clone());
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,tg,4,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(tg), 4, tp, 0);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON);
        // e:SetProperty(0)
        effect::set_property(d, e, 0, 0);
        // if not cost_skip then Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1) end
        if !cost_skip {
            duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    } else if op == 3 {
        // e:SetCategory(CATEGORY_TOHAND)
        effect::set_category(d, e, CATEGORY_TOHAND);
        // e:SetProperty(0)
        effect::set_property(d, e, 0, 0);
        // if not cost_skip then Duel.RegisterFlagEffect(tp,id+2,RESET_PHASE|PHASE_END,0,1) end
        if !cost_skip {
            duel::register_flag_effect(d, tp, ID + 2, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_ONFIELD|LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, (LOCATION_ONFIELD | LOCATION_GRAVE) as i32);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local op=e:GetLabel()
    let op = effect::get_label(d, e);
    if op == 1 {
        // Return 2 LIGHT Beast monsters you control and 2 cards your opponent controls to the hand
        // local tg=Duel.GetTargetCards(e)
        let tg = duel::get_target_cards(d, e).nil();
        // if #tg>0 then Duel.SendtoHand(tg,nil,REASON_EFFECT) end
        if !tg.is_empty() {
            duel::send_to_hand(d, tg, None, REASON_EFFECT, None);
        }
    } else if op == 2 {
        // Special Summon 1 "Yummy" monster from your hand or GY, but it cannot attack directly this turn
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,e,tp):GetFirst()
        let nv = aux::necro_valley_filter(|d, c| spfilter(d, c, e, tp));
        let g = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &g);
        // if sc and Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)>0 then
        if let Some(sc) = sc {
            if duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
                // It cannot attack directly this turn
                // local e1=Effect.CreateEffect(e:GetHandler())
                let h = effect::get_handler(d, e).nil();
                let e1 = effect::create_effect(d, h);
                // e1:SetDescription(3207)
                effect::set_description(d, e1, 3207);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
                // e1:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
                effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
                // e1:SetReset(RESETS_STANDARD_PHASE_END)
                effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
                // sc:RegisterEffect(e1)
                card::register_effect(d, sc, e1, false);
            }
        }
    } else if op == 3 {
        // Return 1 Field Spell from your face-up field or GY to the hand, then you can place 1 "Yummy" Field Spell from your hand face-up on your field
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.fieldspellthfilter),tp,LOCATION_ONFIELD|LOCATION_GRAVE,0,1,1,nil):GetFirst()
        let nv = aux::necro_valley_filter(|d, c| fieldspellthfilter(d, c));
        let g = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_ONFIELD | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &g);
        // if sc and Duel.SendtoHand(sc,nil,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_HAND) then
        if let Some(sc) = sc {
            if duel::send_to_hand(d, CardSet::from_card(sc), None, REASON_EFFECT, None) > 0 && card::is_location(d, sc, LOCATION_HAND) {
                // Duel.ConfirmCards(1-tp,sc)
                duel::confirm_cards(d, 1 - tp, CardSet::from_card(sc));
                // Duel.ShuffleHand(tp)
                duel::shuffle_hand(d, tp);
                // if Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_HAND,0,1,nil) and Duel.SelectYesNo(tp,aux.Stringid(id,4)) then
                if duel::is_existing_matching_card(d, Some(&plfilter), tp, LOCATION_HAND, 0, 1, NO_EX)
                    && duel::select_yes_no(d, tp, aux::stringid(ID, 4))
                {
                    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
                    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
                    // local plc=Duel.SelectMatchingCard(tp,s.plfilter,tp,LOCATION_HAND,0,1,1,nil):GetFirst()
                    let g2 = duel::select_matching_card(d, tp, Some(&plfilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
                    if let Some(plc) = group::get_first(d, &g2) {
                        // Duel.BreakEffect()
                        duel::break_effect(d);
                        // local fc=Duel.GetFieldCard(tp,LOCATION_FZONE,0)
                        // if fc then
                        if let Some(fc) = duel::get_field_card(d, tp, LOCATION_FZONE, 0) {
                            // Duel.SendtoGrave(fc,REASON_RULE)
                            duel::send_to_grave(d, CardSet::from_card(fc), REASON_RULE, None, None);
                            // Duel.BreakEffect()
                            duel::break_effect(d);
                        }
                        // Duel.MoveToField(plc,tp,tp,LOCATION_FZONE,POS_FACEUP,true)
                        duel::move_to_field(d, plc, tp, tp, LOCATION_FZONE as u16, POS_FACEUP as u8, true, None);
                    }
                }
            }
        }
    }
}
