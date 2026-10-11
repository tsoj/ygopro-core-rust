//! Spirit Poacher
use super::prelude::*;

pub const ID: u32 = 85150300;

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
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE));
    // e1:SetCondition(function(e,tp) return Duel.IsPhase(PHASE_STANDBY) and Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 Level 4 or lower monster from your Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetCost(s.spcost)
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsPhase(PHASE_STANDBY) and Duel.IsTurnPlayer(1-tp)
    duel::is_phase(d, PHASE_STANDBY) && duel::is_turn_player(d, 1 - a.tp)
}
fn plfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // c:IsMonster() and not c:IsForbidden() and c:CheckUniqueOnField(tp)
    card::is_monster(d, c) && !card::is_forbidden(d, c) && card::check_unique_on_field(d, c, tp, None, None)
}
fn stspfilter(d: &mut Duel, c: CardId, e: EffectId, opp: u8, opp_lp: i32) -> bool {
    // c:IsMonsterCard() and c:GetBaseAttack()<opp_lp and c:IsCanBeSpecialSummoned(e,0,opp,false,false)
    card::is_monster_card(d, c)
        && card::get_base_attack(d, c) < opp_lp
        && card::is_can_be_special_summoned(d, c, e, 0, opp, false, false, None, None, None)
}
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local b1=Duel.GetLocationCount(tp,LOCATION_SZONE)>0 and Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_DECK,0,1,nil,tp)
    let b1 = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| plfilter(d, c, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    // local opp=1-tp
    let opp = 1 - tp;
    // local b2=Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.stspfilter,tp,LOCATION_STZONE,0,1,nil,e,opp,Duel.GetLP(opp))
    let b2 = duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && {
        let opp_lp = duel::get_lp(d, opp);
        duel::is_existing_matching_card(d, Some(&|d, c| stspfilter(d, c, a.e, opp, opp_lp)), tp, LOCATION_STZONE, 0, 1, NO_EX)
    };
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,2)}, {b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]).nil();
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    if op == 1 {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_STZONE)
        duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_STZONE as i32);
    }
    true
}
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    if op == 1 {
        // Place 1 monster from your Deck in your Spell & Trap Zone as a face-up Continuous Spell
        // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
        if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.plfilter,tp,LOCATION_DECK,0,1,1,nil,tp):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&|d, c| plfilter(d, c, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX);
        let sc = g.and_then(|g| group::get_first(d, &g));
        // if sc and Duel.MoveToField(sc,tp,tp,LOCATION_SZONE,POS_FACEUP,true) then
        if let Some(sc) = sc {
            if duel::move_to_field(d, sc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None) {
                // Treated as a Continuous Spell
                // local e1=Effect.CreateEffect(e:GetHandler())
                let h = effect::get_handler(d, a.e).nil();
                let e1 = effect::create_effect(d, h);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
                // e1:SetCode(EFFECT_CHANGE_TYPE)
                effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
                // e1:SetValue(TYPE_SPELL|TYPE_CONTINUOUS)
                effect::set_value(d, e1, (TYPE_SPELL | TYPE_CONTINUOUS) as i64);
                // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TURN_SET))
                effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
                // sc:RegisterEffect(e1)
                card::register_effect(d, sc, e1, false);
            }
        }
    } else if op == 2 {
        // Your opponent can Special Summon 1 Monster Card with less original ATK than their LP from your Spell & Trap Zone to their field, then they pay LP equal to its original ATK
        // local opp=1-tp
        let opp = 1 - tp;
        // local opp_lp=Duel.GetLP(opp)
        let opp_lp = duel::get_lp(d, opp);
        // if Duel.GetLocationCount(opp,LOCATION_MZONE)>0
        //   and Duel.IsExistingMatchingCard(s.stspfilter,tp,LOCATION_STZONE,0,1,nil,e,opp,opp_lp)
        //   and Duel.SelectYesNo(opp,aux.Stringid(id,4)) then
        if duel::get_location_count(d, opp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| stspfilter(d, c, a.e, opp, opp_lp)), tp, LOCATION_STZONE, 0, 1, NO_EX)
            && duel::select_yes_no(d, opp, aux::stringid(ID, 4))
        {
            // Duel.Hint(HINT_SELECTMSG,opp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, opp, HINTMSG_SPSUMMON as u64);
            // local sc=Duel.SelectMatchingCard(opp,s.stspfilter,tp,LOCATION_STZONE,0,1,1,nil,e,opp,opp_lp):GetFirst()
            let g = duel::select_matching_card(d, opp, Some(&|d, c| stspfilter(d, c, a.e, opp, opp_lp)), tp, LOCATION_STZONE, 0, 1, 1, false, NO_EX);
            let sc = g.and_then(|g| group::get_first(d, &g));
            // if sc and Duel.SpecialSummon(sc,0,opp,opp,false,false,POS_FACEUP)>0 then
            if let Some(sc) = sc {
                if duel::special_summon(d, sc, 0, opp, opp, false, false, POS_FACEUP as u8, None) > 0 {
                    // Duel.BreakEffect()
                    duel::break_effect(d);
                    // Duel.PayLPCost(opp,sc:GetBaseAttack())
                    let atk = card::get_base_attack(d, sc);
                    duel::pay_lp_cost(d, opp, atk as u32);
                }
            }
        }
    }
}
fn spconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // c:IsType(TYPE_EFFECT) and c:IsOwner(tp) and c:IsControler(1-tp) and c:IsFaceup()
    card::is_type(d, c, TYPE_EFFECT) && card::is_owner(d, c, tp) && card::is_controler(d, c, 1 - tp) && card::is_faceup(d, c)
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.spconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| spconfilter(d, c, tp), 1, NO_EX)
}
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToGraveAsCost() and c:IsStatus(STATUS_EFFECT_ENABLED) end
    if a.chk == Some(0) {
        return card::is_able_to_grave_as_cost(d, c) && card::is_status(d, c, STATUS_EFFECT_ENABLED);
    }
    // Duel.SendtoGrave(c,REASON_COST)
    duel::send_to_grave(d, c, REASON_COST, None, None);
    true
}
fn deckspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // c:IsLevelBelow(4) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_below(d, c, 4) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.deckspfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| deckspfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.deckspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| deckspfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
