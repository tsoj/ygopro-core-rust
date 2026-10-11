//! Loka Samsara
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 2116237;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_VISAS_STARFROST],
    listed_series: &[SET_HEART],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Heart" monster with 3000 ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some((TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END) as u32));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add it to the hand if the opponent Special Summons from the Extra Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spcostfilter(c,e,tp)`
fn spcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsCode(CARD_VISAS_STARFROST) and c:IsAbleToRemoveAsCost()
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c)
    card::is_faceup(d, c)
        && card::is_code(d, c, &[CARD_VISAS_STARFROST])
        && card::is_able_to_remove_as_cost(d, c, None)
        && duel::is_existing_matching_card(d, Some(&|d, cc| spfilter(d, cc, e, tp, Some(c))), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,rmvc)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rmvc: Option<CardId>) -> bool {
    // return c:IsSetCard(SET_HEART) and c:IsAttack(3000) and Duel.GetLocationCountFromEx(tp,tp,rmvc,c)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_set_card(d, c, &[SET_HEART as u16])
        && card::is_attack(d, c, &[3000])
        && duel::get_location_count_from_ex(d, tp, Some(tp), rmvc.map_or(NO_EX, |r| r.into()), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spcostfilter,tp,LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, cc| spcostfilter(d, cc, a.e, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spcostfilter,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&|d, cc| spcostfilter(d, cc, a.e, a.tp)),
        a.tp,
        LOCATION_MZONE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // aux.RemoveUntil(g,nil,REASON_COST,PHASE_END,id,e,tp,aux.DefaultFieldReturnOp,nil,nil,nil,nil,aux.Stringid(id,2))
    aux::remove_until(
        d,
        g,
        None,
        REASON_COST,
        PHASE_END,
        ID,
        a.e,
        a.tp,
        Some(aux::default_field_return()),
        None,
        None,
        None,
        None,
        Some(aux::stringid(ID, 2)),
    );
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, cc| spfilter(d, cc, a.e, a.tp, None)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_MZONE as i32);
    true
}
/// `function(ag) Duel.Remove(ag,POS_FACEDOWN,REASON_EFFECT) end`
fn delayed_remove(d: &mut Duel, ag: &CardSet, _a: &EffArgs) -> bool {
    duel::remove(d, ag.clone(), POS_FACEDOWN as u8, REASON_EFFECT, None, None);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&|d, cc| spfilter(d, cc, a.e, a.tp, None)),
        a.tp,
        LOCATION_EXTRA,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let sc = group::get_first(d, &g);
    // if sc and Duel.SpecialSummonStep(sc,0,tp,tp,true,false,POS_FACEUP) then
    if let Some(sc) = sc {
        if duel::special_summon_step(d, sc, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None) {
            // local c=e:GetHandler()
            let c = effect::get_handler(d, a.e).nil();
            // --Can only activate its effects once
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e1:SetCode(EVENT_CHAINING)
            effect::set_code(d, e1, EVENT_CHAINING);
            // e1:SetRange(LOCATION_MZONE)
            effect::set_range(d, e1, LOCATION_MZONE);
            // e1:SetOperation(s.aclimit)
            effect::set_operation(d, e1, Some(operation(aclimit)));
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // sc:RegisterEffect(e1)
            card::register_effect(d, sc, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EVENT_CHAIN_NEGATED)
            effect::set_code(d, e2, EVENT_CHAIN_NEGATED);
            // sc:RegisterEffect(e2)
            card::register_effect(d, sc, e2, false);
            // local e3=Effect.CreateEffect(e:GetHandler())
            let e3 = effect::create_effect(d, c);
            // e3:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e3:SetCode(EFFECT_CANNOT_TRIGGER)
            effect::set_code(d, e3, EFFECT_CANNOT_TRIGGER);
            // e3:SetCondition(function(e) return e:GetHandler():HasFlagEffect(id+1) end)
            effect::set_condition(d, e3, Some(lua_fn(e3_cond)));
            // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
            // sc:RegisterEffect(e3)
            card::register_effect(d, sc, e3, false);
            // aux.DelayedOperation(sc,PHASE_END,id,e,tp,function(ag) Duel.Remove(ag,POS_FACEDOWN,REASON_EFFECT) end,nil,0,0,aux.Stringid(id,3),aux.Stringid(id,4))
            aux::delayed_operation(
                d,
                sc,
                PHASE_END,
                ID,
                a.e,
                a.tp,
                Some(std::sync::Arc::new(delayed_remove)),
                None,
                Some(0),
                Some(0),
                Some(aux::stringid(ID, 3)),
                Some(aux::stringid(ID, 4)),
            );
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.aclimit(e,tp,eg,ep,ev,re,r,rp)`
fn aclimit(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if re:GetHandler()~=c then return end
    let rh = effect::get_handler(d, a.re.nil()).nil();
    if rh != c {
        return;
    }
    // if e:GetCode()==EVENT_CHAINING then
    if effect::get_code(d, a.e) == EVENT_CHAINING {
        // c:RegisterFlagEffect(id+1,RESET_EVENT|RESETS_STANDARD,0,1)
        let _ = card::register_flag_effect(d, c, ID + 1, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
    } else {
        // c:ResetFlagEffect(id+1)
        card::reset_flag_effect(d, c, ID + 1);
    }
}
/// `function(e) return e:GetHandler():HasFlagEffect(id+1) end`
fn e3_cond(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::has_flag_effect(d, h, ID + 1, None)
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSummonPlayer(1-tp) and c:IsSummonLocation(LOCATION_EXTRA)
    card::is_summon_player(d, c, 1 - tp) && card::is_summon_location(d, c, LOCATION_EXTRA)
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, cc| cfilter(d, cc, tp), 1, NO_EX)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,c)
        duel::confirm_cards(d, 1 - a.tp, c);
    }
}
