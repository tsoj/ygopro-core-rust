//! Ashtraja Susanoo
use super::prelude::*;

pub const ID: u32 = 17020474;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ASHTRA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // If this card is Synchro Summoned: You can banish any number of Traps from your GY; banish that many cards on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsSynchroSummoned() end)
    effect::set_condition(
        d,
        e1,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::is_synchro_summoned(d, h)
        })),
    );
    // e1:SetCost(s.bancost)
    effect::set_cost(d, e1, Some(cost_target(bancost)));
    // e1:SetTarget(s.bantg)
    effect::set_target(d, e1, Some(cost_target(bantg)));
    // e1:SetOperation(s.banop)
    effect::set_operation(d, e1, Some(operation(banop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During the End Phase: You can target up to 2 "Asutra" monsters and/or "Asutra" Traps in your GY and/or banishment; ...
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SET+CATEGORY_LEAVE_GRAVE+CATEGORY_TOEXTRA)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SET + CATEGORY_LEAVE_GRAVE + CATEGORY_TOEXTRA);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.thsettg)
    effect::set_target(d, e2, Some(cost_target(thsettg)));
    // e2:SetOperation(s.thsetop)
    effect::set_operation(d, e2, Some(operation(thsetop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.bancost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn bancost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.AND(Card.IsTrap,Card.IsAbleToRemoveAsCost),tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_trap(d, c) && card::is_able_to_remove_as_cost(d, c, None)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // local max_banish_count=Duel.GetMatchingGroupCount(Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let max_banish_count = duel::get_matching_group_count(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.AND(Card.IsTrap,Card.IsAbleToRemoveAsCost),tp,LOCATION_GRAVE,0,1,max_banish_count,nil)
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&|d, c| card::is_trap(d, c) && card::is_able_to_remove_as_cost(d, c, None)),
        tp,
        LOCATION_GRAVE,
        0,
        1,
        max_banish_count as u16,
        false,
        NO_EX,
    )
    .nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_COST, None, None);
    // e:GetChainData().banish_count=#g
    chain::set_effect_data(d, e, None, chain::field::CARD, Value::Int(g.len() as i64));
    true
}
/// `s.bantg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn bantg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,e:GetChainData().banish_count,PLAYER_EITHER,LOCATION_ONFIELD)
    let count = chain::effect_data(d, a.e, None, chain::field::CARD).int() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, count, PLAYER_EITHER as u8, LOCATION_ONFIELD as i32);
    true
}
/// `s.banop(e,tp,eg,ep,ev,re,r,rp)`
fn banop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local banish_count=e:GetChainData().banish_count
    let banish_count = chain::effect_data(d, e, None, chain::field::CARD).int();
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // if #g<banish_count then return end
    if (g.len() as i64) < banish_count {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local sg=g:Select(tp,banish_count,banish_count,nil)
    let sg = group::select(d, &g, tp, banish_count as u16, banish_count as u16, false, NO_EX).nil();
    // if #sg==banish_count then
    if sg.len() as i64 == banish_count {
        // Duel.HintSelection(sg)
        duel::hint_selection(d, sg.clone(), true);
        // Duel.Remove(sg,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, sg, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
/// `s.setfilter(c,e,tp,mmz_chk)`
fn setfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mmz_chk: bool) -> bool {
    // if not (c:IsSetCard(SET_ASHTRA) and c:IsFaceup()) then return end
    if !(card::is_set_card(d, c, &[SET_ASHTRA as u16]) && card::is_faceup(d, c)) {
        return false;
    }
    // if c:IsMonster() then
    if card::is_monster(d, c) {
        // return c:IsAbleToHand() or (mmz_chk and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEDOWN_DEFENSE))
        return card::is_able_to_hand(d, c, None)
            || (mmz_chk && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None));
    // elseif c:IsTrap() then
    } else if card::is_trap(d, c) {
        // return c:IsAbleToHand() or c:IsSSetable()
        return card::is_able_to_hand(d, c, None) || card::is_s_setable(d, c, false, None);
    }
    // return false
    false
}
/// `s.rescon(to_extra_chk,free_mzones,free_stzones)`: returns the `rescon` of `aux.SelectUnselectGroup`
fn rescon(to_extra_chk: bool, free_mzones: i32, free_stzones: i32) -> impl Fn(&mut Duel, &CardSet, Option<EffectId>, u8, &CardSet, Option<CardId>) -> (bool, bool) {
    move |d, sg, _e, _tp, _mg, _c| {
        // if #sg==2 and not to_extra_chk then return false end
        if sg.len() == 2 && !to_extra_chk {
            return (false, false);
        }
        // local cannot_add_g=sg:Filter(aux.NOT(Card.IsAbleToHand),nil)
        let cannot_add_g = group::filter(d, sg, &|d, c| !card::is_able_to_hand(d, c, None), NO_EX);
        // if #cannot_add_g<2 then return true end
        if cannot_add_g.len() < 2 {
            return (true, false);
        }
        // local monsters,traps=cannot_add_g:Split(Card.IsMonster,nil)
        let (monsters, traps) = group::split(d, &cannot_add_g, &|d, c| card::is_monster(d, c), NO_EX);
        // return #monsters<=free_mzones and #traps<=free_stzones
        (monsters.len() as i32 <= free_mzones && traps.len() as i32 <= free_stzones, false)
    }
}
/// `s.thsettg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thsettg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local mmz_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let mmz_chk = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE|LOCATION_REMOVED) and chkc:IsControler(tp) and s.setfilter(chkc,e,tp,mmz_chk) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE | LOCATION_REMOVED) && card::is_controler(d, chkc, tp) && setfilter(d, chkc, e, tp, mmz_chk);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.setfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil,e,tp,mmz_chk) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| setfilter(d, c, e, tp, mmz_chk)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local to_extra_chk=c:IsAbleToExtra()
    let to_extra_chk = card::is_able_to_extra(d, c);
    // local free_mzones=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let free_mzones = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // local free_stzones=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let free_stzones = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // local g=Duel.GetTargetGroup(s.setfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,nil,e,tp,mmz_chk)
    let g = duel::get_target_group(d, Some(&|d, x| setfilter(d, x, e, tp, mmz_chk)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, NO_EX);
    // local tg=aux.SelectUnselectGroup(g,e,tp,1,2,s.rescon(to_extra_chk,free_mzones,free_stzones),1,tp,aux.Stringid(id,2))
    let rc = rescon(to_extra_chk, free_mzones, free_stzones);
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(2), Some(&rc), 1, tp, Some(aux::stringid(ID, 2)), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SET,tg,#tg,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_SET, Some(tg.clone()), tg.len() as u8, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,tg,#tg,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, Some(tg.clone()), tg.len() as u8, tp, 0);
    // if tg:IsExists(Card.IsMonster,1,nil) then
    if group::is_exists(d, &tg, &|d, x| card::is_monster(d, x), 1, NO_EX) {
        // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,tg:Filter(Card.IsMonster,nil),1,tp,0)
        let mons = group::filter(d, &tg, &|d, x| card::is_monster(d, x), NO_EX);
        duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(mons), 1, tp, 0);
    }
    // if tg:IsExists(Card.IsLocation,1,nil,LOCATION_GRAVE) then
    if group::is_exists(d, &tg, &|d, x| card::is_location(d, x, LOCATION_GRAVE), 1, NO_EX) {
        // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,tg:Filter(Card.IsLocation,nil,LOCATION_GRAVE),1,tp,0)
        let gy = group::filter(d, &tg, &|d, x| card::is_location(d, x, LOCATION_GRAVE), NO_EX);
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(gy), 1, tp, 0);
    }
    // if #tg==2 then Duel.SetOperationInfo(0,CATEGORY_TOEXTRA,c,1,tp,0) end
    if tg.len() == 2 {
        duel::set_operation_info(d, 0, CATEGORY_TOEXTRA, Some(CardSet::from_card(c)), 1, tp, 0);
    }
    // e:GetChainData().number_of_targets=#tg
    chain::set_effect_data(d, e, None, chain::field::CARD + 1, Value::Int(tg.len() as i64));
    true
}
/// `s.thsetop(e,tp,eg,ep,ev,re,r,rp)`
fn thsetop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tg=Duel.GetTargetCards(e)
    // if #tg==0 then return end
    let mut tg = duel::get_target_cards(d, e).nil();
    if tg.is_empty() {
        return;
    }
    // local success=false
    let mut success = false;
    // for i=1,#tg do
    let count = tg.len();
    for i in 1..=count {
        // local tc=nil
        let tc: CardId;
        // if #tg==2 then
        if tg.len() == 2 {
            // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
            duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
            // tc=tg:Select(tp,1,1,nil):GetFirst()
            let sel = group::select(d, &tg, tp, 1, 1, false, NO_EX).nil();
            tc = group::get_first(d, &sel).nil();
            // Duel.HintSelection(tc)
            duel::hint_selection(d, tc, true);
            // tg:RemoveCard(tc)
            group::remove_card(&mut tg, &CardSet::from_card(tc));
        } else {
            // tc=tg:GetFirst()
            tc = group::get_first(d, &tg).nil();
            // if i==2 then Duel.HintSelection(tc) end
            if i == 2 {
                duel::hint_selection(d, tc, true);
            }
        }
        // local monster_set_chk=tc:IsMonster() and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and tc:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEDOWN_DEFENSE)
        let monster_set_chk = card::is_monster(d, tc)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, tc, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None);
        // local trap_set_chk=tc:IsTrap() and tc:IsSSetable()
        let trap_set_chk = card::is_trap(d, tc) && card::is_s_setable(d, tc, false, None);
        // local success_chk=aux.ToHandOrElse(tc,tp,
        //     function() return monster_set_chk or trap_set_chk end,
        //     function() ... end, aux.Stringid(id,3))
        let success_chk = aux::to_hand_or_else(
            d,
            tc,
            tp,
            Some(&|_d, _c| monster_set_chk || trap_set_chk),
            Some(&|d, _g| {
                if monster_set_chk {
                    if duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEDOWN_DEFENSE as u8, None) {
                        duel::confirm_cards(d, 1 - tp, tc);
                        return 1;
                    }
                } else if trap_set_chk {
                    return (duel::s_set(d, tp, tc, None, true) > 0) as i32;
                }
                0
            }),
            Some(aux::stringid(ID, 3)),
        ) > 0;
        // if tc:IsLocation(LOCATION_HAND) then Duel.ShuffleHand(tp) end
        if card::is_location(d, tc, LOCATION_HAND) {
            duel::shuffle_hand(d, tp);
        }
        // if not success and success_chk then success=true end
        if !success && success_chk {
            success = true;
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if success and c:IsRelateToEffect(e) and e:GetChainData().number_of_targets==2 then
    if success && card::is_relate_to_effect(d, c, e) && chain::effect_data(d, e, None, chain::field::CARD + 1).int() == 2 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SendtoDeck(c,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, c, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
