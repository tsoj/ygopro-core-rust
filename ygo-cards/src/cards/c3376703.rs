//! Arcana Force V - The Hierophant
use super::prelude::*;

pub const ID: u32 = 3376703;

pub const INFO: ScriptInfo = ScriptInfo { toss_coin: true, listed_series: &[SET_ARCANA_FORCE as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Apply a "your opponent cannot activate cards or effects when an "Arcana Force" monster(s) is Summoned to your field" effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(function(e,tp) return not Duel.HasFlagEffect(tp,id) end)
    effect::set_condition(d, e1, Some(condition(|d, a| !duel::has_flag_effect(d, a.tp, ID, None))));
    // e1:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e1, Some(cost::self_discard()));
    // e1:SetOperation(s.limeffop)
    effect::set_operation(d, e1, Some(operation(limeffop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Toss a coin and apply the appropriate effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_COIN+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_COIN + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetTarget(s.cointg)
    effect::set_target(d, e2, Some(cost_target(cointg)));
    // e2:SetOperation(s.coinop)
    effect::set_operation(d, e2, Some(operation(coinop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e2:Clone()
    let e4 = effect::clone(d, e2);
    // e4:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_FLIP_SUMMON_SUCCESS);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.limeffop(e,tp,eg,ep,ev,re,r,rp)`
fn limeffop(d: &mut Duel, a: &EffArgs) {
    // if Duel.HasFlagEffect(tp,id) then return end
    if duel::has_flag_effect(d, a.tp, ID, None) {
        return;
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetOperation(s.limop1)
    effect::set_operation(d, e1, Some(operation(limop1)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_FLIP_SUMMON_SUCCESS);
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, a.tp);
    // local e4=e1:Clone()
    let e4 = effect::clone(d, e1);
    // e4:SetCode(EVENT_CHAIN_END)
    effect::set_code(d, e4, EVENT_CHAIN_END);
    // e4:SetOperation(s.limop2)
    effect::set_operation(d, e4, Some(operation(limop2)));
    // Duel.RegisterEffect(e4,tp)
    duel::register_effect(d, e4, a.tp);
    // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,2))
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
}
/// `s.limopfilter(c,tp)`
fn limopfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ARCANA_FORCE) and c:IsFaceup() and c:IsControler(tp)
    card::is_set_card(d, c, &[SET_ARCANA_FORCE as u16]) && card::is_faceup(d, c) && card::is_controler(d, c, tp)
}
/// `s.limop1(e,tp,eg,ep,ev,re,r,rp)`
fn limop1(d: &mut Duel, a: &EffArgs) {
    // if eg and eg:IsExists(s.limopfilter,1,nil,tp) then
    if let Some(eg) = a.eg.clone() {
        if group::is_exists(d, &eg, &|d, c| limopfilter(d, c, a.tp), 1, NO_EX) {
            // Duel.SetChainLimitTillChainEnd(function(re,rp,tp) return tp==rp end)
            duel::set_chain_limit_till_chain_end(d, lua_fn(|_d, b: Args| b.p(2) == b.p(1)));
        }
    }
}
/// `s.limop2(e,tp,eg,ep,ev,re,r,rp)`
fn limop2(d: &mut Duel, a: &EffArgs) {
    // local _,g=Duel.CheckEvent(EVENT_SPSUMMON_SUCCESS,true)
    let g = duel::check_event(d, EVENT_SPSUMMON_SUCCESS).and_then(|ev| ev.event_cards);
    // if g and g:IsExists(s.limopfilter,1,nil,tp) and Duel.CheckEvent(EVENT_SPSUMMON_SUCCESS) then
    if let Some(g) = g {
        if group::is_exists(d, &g, &|d, c| limopfilter(d, c, a.tp), 1, NO_EX) && duel::check_event(d, EVENT_SPSUMMON_SUCCESS).is_some() {
            // Duel.SetChainLimitTillChainEnd(function(re,rp,tp) return tp==rp end)
            duel::set_chain_limit_till_chain_end(d, lua_fn(|_d, b: Args| b.p(2) == b.p(1)));
        }
    }
}
/// `s.cointg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cointg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_COIN,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, a.tp, 1);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.samenamefilter(c,code)`
fn samenamefilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return c:IsCode(code) and c:IsFaceup() and c:IsMonster()
    card::is_code(d, c, &[code]) && card::is_faceup(d, c) && card::is_monster(d, c)
}
/// `s.headsspfilter(c,e,tp)`
fn headsspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(4) and c:IsSetCard(SET_ARCANA_FORCE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and not Duel.IsExistingMatchingCard(s.samenamefilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil,c:GetCode())
    card::is_level_below(d, c, 4)
        && card::is_set_card(d, c, &[SET_ARCANA_FORCE as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && !{
            let code = card::get_code(d, c)[0];
            duel::is_existing_matching_card(d, Some(&|d, x| samenamefilter(d, x, code)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX)
        }
}
/// `s.tailsspfilter(c,e,tp)`
fn tailsspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ARCANA_FORCE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,1-tp)
    card::is_set_card(d, c, &[SET_ARCANA_FORCE as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), None)
}
/// `s.coinop(e,tp,eg,ep,ev,re,r,rp)`
fn coinop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local coin=nil
    let coin: i32;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_LIGHT_BARRIER) then
    if duel::is_player_affected_by_effect(d, tp, CARD_LIGHT_BARRIER).is_some() {
        // local b1=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and Duel.IsExistingMatchingCard(s.headsspfilter,tp,LOCATION_DECK,0,1,nil,e,tp)
        let b1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| headsspfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
        // local b2=Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0
        //     and Duel.IsExistingMatchingCard(s.tailsspfilter,tp,LOCATION_DECK,0,1,nil,e,tp)
        let b2 = duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| tailsspfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
        // local op=Duel.SelectEffect(tp,
        //     {b1,aux.GetCoinEffectHintString(COIN_HEADS)},
        //     {b2,aux.GetCoinEffectHintString(COIN_TAILS)})
        let op = duel::select_effect(
            d,
            tp,
            &[
                (b1, aux::get_coin_effect_hint_string(COIN_HEADS as i32).nil()),
                (b2, aux::get_coin_effect_hint_string(COIN_TAILS as i32).nil()),
            ],
        );
        // if not op then return end
        let Some(op) = op else {
            return;
        };
        // coin=op==1 and COIN_HEADS or COIN_TAILS
        coin = if op == 1 { COIN_HEADS as i32 } else { COIN_TAILS as i32 };
    // else
    } else {
        // coin=Duel.TossCoin(tp,1)
        coin = duel::toss_coin(d, tp, 1)[0];
    }
    // if coin==COIN_HEADS then
    if coin == COIN_HEADS as i32 {
        // Heads: Special Summon 1 Level 4 or lower "Arcana Force" monster from your Deck with a different name from the monsters you control and in your GY
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.headsspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| headsspfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, &g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    // elseif coin==COIN_TAILS then
    } else if coin == COIN_TAILS as i32 {
        // Tails: Special Summon 1 "Arcana Force" monster from your Deck to your opponent's field
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.tailsspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| tailsspfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,1-tp,false,false,POS_FACEUP)
            duel::special_summon(d, &g, 0, tp, 1 - tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
