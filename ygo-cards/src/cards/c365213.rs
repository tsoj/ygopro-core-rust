//! Arrive in Light
use super::prelude::*;

pub const ID: u32 = 365213;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_STARDUST_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // cannot to extra
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_TO_DECK)
    effect::set_code(d, e2, EFFECT_CANNOT_TO_DECK);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_ONFIELD,LOCATION_ONFIELD)
    effect::set_target_range(d, e2, LOCATION_ONFIELD, LOCATION_ONFIELD);
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(effect_card_fn(tdtg)));
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // draw or special summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_DRAW+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_DRAW + CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCondition(s.applycon)
    effect::set_condition(d, e3, Some(condition(applycon)));
    // e3:SetTarget(s.applytg)
    effect::set_target(d, e3, Some(cost_target(applytg)));
    // e3:SetOperation(s.applyop)
    effect::set_operation(d, e3, Some(operation(applyop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function s.tdfilter(c) return c:IsLevel(1) and c:IsRace(RACE_DRAGON) and (c:IsLocation(LOCATION_DECK) or c:IsAbleToDeck()) end`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_level(d, c, &[1])
        && card::is_race(d, c, RACE_DRAGON)
        && (card::is_location(d, c, LOCATION_DECK) || card::is_able_to_deck(d, c))
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // return Duel.IsExistingMatchingCard(s.tdfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil)
        return duel::is_existing_matching_card(d, Some(&tdfilter), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `function(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local g1=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_HAND,0,nil)
    let g1 = duel::get_matching_group(d, Some(&tdfilter), tp, LOCATION_HAND, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_DECK,0,nil)
    let g2 = duel::get_matching_group(d, Some(&tdfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if not (#g1>0 or #g2>0) then return end
    if g1.is_empty() && g2.is_empty() {
        return;
    }
    // local op=-1
    let mut op: i32 = -1;
    // if #g1>0 and #g2>0 then
    if !g1.is_empty() && !g2.is_empty() {
        // op=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1))
        op = duel::select_option(d, tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)]);
    // elseif #g1>0 then
    } else if !g1.is_empty() {
        // op=0
        op = 0;
    // elseif #g2>0 then
    } else if !g2.is_empty() {
        // op=1
        op = 1;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    if op == 0 {
        // local tc=g1:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g1, tp, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &sel).nil();
        // Duel.SendtoDeck(tc,nil,SEQ_DECKTOP,REASON_EFFECT)
        duel::send_to_deck(d, tc, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    } else if op == 1 {
        // local tc=g2:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g2, tp, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &sel).nil();
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, tp);
        // Duel.MoveSequence(tc,SEQ_DECKTOP)
        duel::move_sequence(d, tc, SEQ_DECKTOP as u8, None);
    }
    // Duel.ConfirmDecktop(tp,1)
    duel::confirm_decktop(d, tp, 1);
}
/// `function s.tdtg(e,c) return c:IsFaceup() and ((c:IsCode(CARD_STARDUST_DRAGON) and (c:IsType(TYPE_EXTRA) or c:IsLocation(LOCATION_SZONE)))
///     or (c:IsType(TYPE_SYNCHRO) and c:ListsCode(CARD_STARDUST_DRAGON))) end`
fn tdtg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_faceup(d, c)
        && ((card::is_code(d, c, &[CARD_STARDUST_DRAGON])
            && (card::is_type(d, c, TYPE_EXTRA) || card::is_location(d, c, LOCATION_SZONE)))
            || (card::is_type(d, c, TYPE_SYNCHRO) && card::lists_code(d, c, &[CARD_STARDUST_DRAGON])))
}
/// `function(e,tp,eg,ep,ev,re,r,rp)`
fn applycon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(aux.FaceupFilter(Card.IsType,TYPE_SYNCHRO),1,nil)
    let eg = a.eg.clone().nil();
    let f = aux::faceup_filter(|d, c| card::is_type(d, c, TYPE_SYNCHRO));
    group::is_exists(d, &eg, &f, 1, NO_EX)
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk)`
fn applytg(d: &mut Duel, a: &EffArgs) -> bool {
    // local draw=s.drtg(e,tp,eg,ep,ev,re,r,rp,0)
    let draw = drtg(d, a.tp);
    // local spsummon=s.sptg(e,tp,eg,ep,ev,re,r,rp,0)
    let spsummon = sptg(d, a.e, a.tp);
    // if chk==0 then return draw or spsummon end
    if a.chk == Some(0) {
        return draw || spsummon;
    }
    true
}
/// `function(e,tp,eg,ep,ev,re,r,rp)`
fn applyop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local draw=s.drtg(e,tp,eg,ep,ev,re,r,rp,0)
    let draw = drtg(d, tp);
    // local spsummon=s.sptg(e,tp,eg,ep,ev,re,r,rp,0)
    let spsummon = sptg(d, a.e, tp);
    // local op=-1
    let mut op: i32 = -1;
    // if draw and spsummon then
    if draw && spsummon {
        // op=Duel.SelectOption(tp,aux.Stringid(id,2),aux.Stringid(id,3))
        op = duel::select_option(d, tp, true, &[aux::stringid(ID, 2), aux::stringid(ID, 3)]);
    // elseif draw then
    } else if draw {
        // op=0
        op = 0;
    // elseif spsummon then
    } else if spsummon {
        // op=1
        op = 1;
    }
    // if op==0 then
    if op == 0 {
        // s.drop(e,tp,eg,ep,ev,re,r,rp)
        drop_op(d, a);
    // elseif op==1 then
    } else if op == 1 {
        // s.spop(e,tp,eg,ep,ev,re,r,rp)
        spop(d, a);
    }
}
/// `s.drtg(...)` with chk == 0 (only called from applytg and applyop).
/// `Duel.IsPlayerCanDraw(tp,1) and Duel.GetFlagEffect(tp,id)==0`
fn drtg(d: &mut Duel, tp: u8) -> bool {
    duel::is_player_can_draw(d, tp, 1) && duel::get_flag_effect(d, tp, ID) == 0
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetFlagEffect(tp,id)>0 then return end
    if duel::get_flag_effect(d, tp, ID) > 0 {
        return;
    }
    // Duel.Draw(tp,1,REASON_EFFECT)
    duel::draw(d, tp, 1, REASON_EFFECT);
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
}
/// `function s.spfilter(c,e,tp) return c:IsCanBeSpecialSummoned(e,0,tp,false,false) and c:IsType(TYPE_TUNER) end`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None) && card::is_type(d, c, TYPE_TUNER)
}
/// `s.sptg(...)` with chk == 0 (only called from applytg and applyop).
/// `Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.GetFlagEffect(tp,id+1)==0
///     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp)`
fn sptg(d: &mut Duel, e: EffectId, tp: u8) -> bool {
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::get_flag_effect(d, tp, ID + 1) == 0
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX)
}
/// `function s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if Duel.GetFlagEffect(tp,id+1)>0 then return end
    if duel::get_flag_effect(d, tp, ID + 1) > 0 {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
    // Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
}
