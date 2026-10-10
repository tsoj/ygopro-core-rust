//! Gravekeeper's Trap
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 98715423;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_EXCHANGE_SPIRIT], listed_series: &[SET_GRAVEKEEPERS as u32], ..ScriptInfo::DEFAULT };

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
    // Cannot activate effects or Special Summon from the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(0,1)
    effect::set_target_range(d, e2, 0, 1);
    // e2:SetCondition(s.limcon)
    effect::set_condition(d, e2, Some(condition(limcon)));
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e2:SetValue(function(_,re) return re:GetActivateLocation()==LOCATION_GRAVE end)
    effect::set_value_fn(d, e2, lua_fn(|d, a: Args| {
        let re = a.e(1);
        effect::get_activate_location(d, re) == LOCATION_GRAVE
    }));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // e3:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e3, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e3:SetTarget(function(_,c) return c:IsLocation(LOCATION_GRAVE) end)
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_GRAVE))));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Search 1 "Gravekeeper's" or EARTH Fairy monster
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND)
    effect::set_category(d, e4, CATEGORY_SEARCH + CATEGORY_TOHAND);
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e4, EVENT_FREE_CHAIN);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_MAIN_END)
    effect::set_hint_timing(d, e4, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    // e4:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e4, Some(condition(main_phase_cond)));
    // e4:SetCost(s.thcost)
    effect::set_cost(d, e4, Some(cost_target(thcost)));
    // e4:SetTarget(s.thtg)
    effect::set_target(d, e4, Some(cost_target(thtg)));
    // e4:SetOperation(s.thop)
    effect::set_operation(d, e4, Some(operation(thop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Declare card name
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e5, CATEGORY_TOGRAVE);
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e5:SetCode(EVENT_PREDRAW)
    effect::set_code(d, e5, EVENT_PREDRAW);
    // e5:SetRange(LOCATION_SZONE)
    effect::set_range(d, e5, LOCATION_SZONE);
    // e5:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e5, 1, ID, 1, 0);
    // e5:SetCondition(s.tgcon)
    effect::set_condition(d, e5, Some(condition(tgcon)));
    // e5:SetTarget(s.tgtg)
    effect::set_target(d, e5, Some(cost_target(tgtg)));
    // e5:SetOperation(s.tgop)
    effect::set_operation(d, e5, Some(operation(tgop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // Check if normal draw has happened
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_DRAW)
        effect::set_code(d, ge1, EVENT_DRAW);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
/// `s.limcon(e)`
fn limcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(Card.IsCode,e:GetHandlerPlayer(),LOCATION_GRAVE,0,1,nil,CARD_EXCHANGE_SPIRIT)
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_code(d, c, &[CARD_EXCHANGE_SPIRIT])), p, LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `function() return Duel.IsMainPhase() end`
fn main_phase_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_main_phase(d, None)
}
/// `s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD,nil)
    duel::discard_hand(d, a.tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToHand() and c:IsMonster()
    //     and (c:IsSetCard(SET_GRAVEKEEPERS) or (c:IsAttribute(ATTRIBUTE_EARTH) and c:IsRace(RACE_FAIRY)))
    card::is_able_to_hand(d, c, None)
        && card::is_monster(d, c)
        && (card::is_set_card(d, c, &[SET_GRAVEKEEPERS as u16]) || (card::is_attribute(d, c, ATTRIBUTE_EARTH) && card::is_race(d, c, RACE_FAIRY as u64)))
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
/// `s.tgcon(e,tp,eg,ep,ev,re,r,rp)`
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return Duel.IsTurnPlayer(1-tp) and not Duel.IsPlayerAffectedByEffect(1-tp,EFFECT_CANNOT_DRAW)
    //     and Duel.GetDrawCount(1-tp)>0 and Duel.GetFlagEffect(1-tp,id)==0
    duel::is_turn_player(d, 1 - tp)
        && duel::is_player_affected_by_effect(d, 1 - tp, EFFECT_CANNOT_DRAW).is_none()
        && duel::get_draw_count(d, 1 - tp) > 0
        && duel::get_flag_effect(d, 1 - tp, ID) == 0
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CODE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CODE as u64);
    // e:SetLabel(Duel.AnnounceCard(tp,TYPE_EXTRA,OPCODE_ISTYPE,OPCODE_NOT))
    let code = duel::announce_card(d, tp, &[TYPE_EXTRA as u64, OPCODE_ISTYPE, OPCODE_NOT]);
    effect::set_label(d, a.e, &[code as i64]);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or Duel.GetDrawCount(1-tp)<1 then return end
    if !card::is_relate_to_effect(d, c, a.e) || duel::get_draw_count(d, 1 - tp) < 1 {
        return;
    }
    // Look at the drawn card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_DRAW)
    effect::set_code(d, e1, EVENT_DRAW);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetOperation(s.drawcheck)
    effect::set_operation(d, e1, Some(operation(drawcheck)));
    // e1:SetLabel(e:GetLabel())
    let label = effect::get_label(d, a.e);
    effect::set_label(d, e1, &[label]);
    // e1:SetReset(RESET_PHASE|PHASE_DRAW)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_DRAW, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.drawcheck(e,tp,eg,ep,ev,re,r,rp)`
fn drawcheck(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.IsPhase(PHASE_DRAW) and (r&REASON_RULE)==REASON_RULE and #eg>0 then
    if duel::is_phase(d, PHASE_DRAW) && (a.r & REASON_RULE) == REASON_RULE && !a.eg().is_empty() {
        let eg = a.eg();
        // Duel.ConfirmCards(tp,eg)
        duel::confirm_cards(d, tp, &eg);
        // local dg=eg:Filter(Card.IsCode,nil,e:GetLabel())
        let label = effect::get_label(d, a.e) as u32;
        let dg = group::filter(d, &eg, &|d, c| card::is_code(d, c, &[label]), NO_EX);
        // if #dg>0 then
        if !dg.is_empty() {
            // Duel.SendtoGrave(dg,REASON_EFFECT)
            duel::send_to_grave(d, &dg, REASON_EFFECT, None, None);
        }
        // Duel.ShuffleHand(1-tp)
        duel::shuffle_hand(d, 1 - tp);
    }
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsPhase(PHASE_DRAW) and (r&REASON_RULE)==REASON_RULE then
    if duel::is_phase(d, PHASE_DRAW) && (a.r & REASON_RULE) == REASON_RULE {
        // Duel.RegisterFlagEffect(ep,id,RESET_PHASE|PHASE_STANDBY,0,1)
        duel::register_flag_effect(d, a.ep, ID, RESET_PHASE | PHASE_STANDBY, 0, 1, 0);
    }
}
