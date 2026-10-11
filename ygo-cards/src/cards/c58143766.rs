//! Volcanic Emission
//! ヴォルカニック・エミッション
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 58143766;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VOLCANIC], ..ScriptInfo::DEFAULT };

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
// function s.thspfilter(c,e,tp,ft)
fn thspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    // return c:IsSetCard(SET_VOLCANIC) and c:IsMonster() and (c:IsAbleToHand() or (ft>0 and c:IsCanBeSpecialSummoned(e,0,tp,true,false)))
    card::is_set_card(d, c, &[SET_VOLCANIC as u16])
        && card::is_monster(d, c)
        && (card::is_able_to_hand(d, c, None) || (ft > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)))
}
// function s.damfilter(c)
fn damfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_PYRO) and c:IsFaceup() and c:GetBaseAttack()>0
    card::is_race(d, c, RACE_PYRO) && card::is_faceup(d, c) && card::get_base_attack(d, c) > 0
}
// function s.effcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // e:SetLabel(-100)
    effect::set_label(d, a.e, &[-100]);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // local b1=not Duel.HasFlagEffect(tp,id)
    //     and Duel.IsExistingMatchingCard(s.thspfilter,tp,LOCATION_DECK,0,1,nil,e,tp,ft)
    let b1 = !duel::has_flag_effect(d, tp, ID, None)
        && duel::is_existing_matching_card(d, Some(&|d, c| thspfilter(d, c, a.e, tp, ft)), tp, LOCATION_DECK, 0, 1, NO_EX);
    // local b2=not Duel.HasFlagEffect(tp,id+1)
    //     and Duel.IsExistingTarget(s.damfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    let b2 = !duel::has_flag_effect(d, tp, ID + 1, None)
        && duel::is_existing_target(d, Some(&damfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    true
}
// function s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.damfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && damfilter(d, chkc);
    }
    // local cost_skip=e:GetLabel()~=-100
    let cost_skip = effect::get_label(d, a.e) != -100;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // local b1=(cost_skip or not Duel.HasFlagEffect(tp,id))
    //     and Duel.IsExistingMatchingCard(s.thspfilter,tp,LOCATION_DECK,0,1,nil,e,tp,ft)
    let b1 = (cost_skip || !duel::has_flag_effect(d, tp, ID, None))
        && duel::is_existing_matching_card(d, Some(&|d, c| thspfilter(d, c, a.e, tp, ft)), tp, LOCATION_DECK, 0, 1, NO_EX);
    // local b2=(cost_skip or not Duel.HasFlagEffect(tp,id+1))
    //     and Duel.IsExistingTarget(s.damfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    let b2 = (cost_skip || !duel::has_flag_effect(d, tp, ID + 1, None))
        && duel::is_existing_target(d, Some(&damfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    // if chk==0 then e:SetLabel(0) return b1 or b2 end
    if a.chk == Some(0) {
        effect::set_label(d, a.e, &[0]);
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)})
    // (not cancelable: a nil choice leaves the label nil, which matches neither branch, as 0 does)
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).unwrap_or(0) as i64;
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
        // e:SetProperty(0)
        effect::set_property(d, a.e, 0, 0);
        // if not cost_skip then Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1) end
        if !cost_skip {
            duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
        // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
        duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    // elseif op==2 then
    } else if op == 2 {
        // e:SetCategory(CATEGORY_DAMAGE)
        effect::set_category(d, a.e, CATEGORY_DAMAGE);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
        // local tc=Duel.SelectTarget(tp,s.damfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil):GetFirst()
        let sel = duel::select_target(d, tp, Some(&damfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &sel).nil();
        // local dam=tc:GetBaseAttack()
        let mut dam = card::get_base_attack(d, tc) as i64;
        // local ctrl=tc:GetControler()
        let ctrl = card::get_controler(d, tc);
        // if ctrl==tp then dam=dam//2 end
        if ctrl == tp {
            dam = dam.div_euclid(2);
        }
        // Duel.SetTargetParam(ctrl)
        duel::set_target_param(d, ctrl as u32);
        // if not cost_skip then Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1) end
        if !cost_skip {
            duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
        // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,dam)
        duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, dam as i32);
    }
    true
}
// function s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // Search or Special Summon 1 "Volcanic" monster from your Deck
        // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
        let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,3))
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 3));
        // local sc=Duel.SelectMatchingCard(tp,s.thspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,ft):GetFirst()
        let sel = duel::select_matching_card(d, tp, Some(&|d, c| thspfilter(d, c, a.e, tp, ft)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX);
        let sc = match sel {
            Some(g) => group::get_first(d, &g),
            None => None,
        };
        // if not sc then return end
        let Some(sc) = sc else {
            return;
        };
        // aux.ToHandOrElse(sc,tp,
        //     function(c)
        //         return ft>0 and sc:IsCanBeSpecialSummoned(e,0,tp,true,false)
        //     end,
        //     function(c)
        //         if Duel.SpecialSummonStep(sc,0,tp,tp,true,false,POS_FACEUP) then
        //             --Return it to the hand during the End Phase
        //             aux.DelayedOperation(sc,PHASE_END,id,e,tp,function(ag) Duel.SendtoHand(ag,nil,REASON_EFFECT) end,nil,0,0,aux.Stringid(id,5))
        //         end
        //         Duel.SpecialSummonComplete()
        //     end,
        //     aux.Stringid(id,4)
        // )
        aux::to_hand_or_else(
            d,
            sc,
            tp,
            Some(&|d, _c| ft > 0 && card::is_can_be_special_summoned(d, sc, a.e, 0, tp, true, false, None, None, None)),
            Some(&|d, _g| {
                if duel::special_summon_step(d, sc, 0, tp, tp, true, false, POS_FACEUP as u8, None) {
                    // Return it to the hand during the End Phase
                    aux::delayed_operation(
                        d,
                        sc,
                        PHASE_END,
                        ID,
                        a.e,
                        tp,
                        Some(std::sync::Arc::new(|d: &mut Duel, ag: &CardSet, _a: &EffArgs| {
                            duel::send_to_hand(d, ag.clone(), None, REASON_EFFECT, None);
                            true
                        })),
                        None,
                        Some(0),
                        Some(0),
                        Some(aux::stringid(ID, 5)),
                        None,
                    );
                }
                duel::special_summon_complete(d);
                0
            }),
            Some(aux::stringid(ID, 4)),
        );
    // elseif op==2 then
    } else if op == 2 {
        // Inflict damage to your opponent equal to that monster's original ATK
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
            // local dam=tc:GetBaseAttack()
            let mut dam = card::get_base_attack(d, tc) as i64;
            // local ctrl=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
            let ctrl = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
            // if ctrl==tp then dam=dam//2 end
            if ctrl == tp as i64 {
                dam = dam.div_euclid(2);
            }
            // Duel.Damage(1-tp,dam,REASON_EFFECT)
            duel::damage(d, 1 - tp, dam, REASON_EFFECT, false, None);
        }
    }
}
