//! Rainbow Bridge of Salvation
use super::prelude::*;

pub const ID: u32 = 5611760;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_CRYSTAL_BEAST}
    listed_series: &[SET_CRYSTAL_BEAST],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetCondition(s.tdcon)
    effect::set_condition(d, e1, Some(condition(tdcon)));
    // e1:SetTarget(s.tdtg)
    effect::set_target(d, e1, Some(cost_target(tdtg)));
    // e1:SetOperation(s.tdop)
    effect::set_operation(d, e1, Some(operation(tdop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Search 1 "Crystal Beast" and 1 Field Spell
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_SEARCH + CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_END_PHASE));
    // e2:SetCondition(function(_,tp) return Duel.GetFlagEffect(tp,id+1)==0 end)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.tdcon(e,tp,...)`
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsLevelAbove,10),tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_level_above(d, c, 10))),
        tp,
        LOCATION_MZONE,
        LOCATION_MZONE,
        NO_EX,
    );
    // return #g>1 and g:GetClassCount(Card.GetRace)>1 and Duel.GetFlagEffect(tp,id)==0
    g.len() > 1
        && group::get_class_count(d, &g, &|d, c| card::get_race(d, c) as i64) > 1
        && duel::get_flag_effect(d, tp, ID) == 0
}
/// `s.tdtg(e,tp,...,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local loc=LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE
    let loc = LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE;
    // local g=Duel.GetMatchingGroup(Card.IsAbleToDeck,tp,loc,loc,nil,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&card::is_able_to_deck), tp, loc, loc, h.into());
    // if chk==0 then return #g>0 and Duel.IsPlayerCanDraw(tp,5) and Duel.IsPlayerCanDraw(1-tp,5) end
    if a.chk == Some(0) {
        return !g.is_empty() && duel::is_player_can_draw(d, tp, 5) && duel::is_player_can_draw(d, 1 - tp, 5);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,PLAYER_ALL,5)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, PLAYER_ALL as u8, 5);
    true
}
/// `s.tdop(e,tp,...)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetFlagEffect(tp,id)~=0 then return end
    if duel::get_flag_effect(d, tp, ID) != 0 {
        return;
    }
    // Duel.RegisterFlagEffect(tp,id,0,0,0)
    duel::register_flag_effect(d, tp, ID, 0, 0, 0, 0);
    // local loc=LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE
    let loc = LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE;
    // local g=Duel.GetMatchingGroup(Card.IsAbleToDeck,tp,loc,loc,nil,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&card::is_able_to_deck), tp, loc, loc, h.into());
    // if #g>0 and Duel.SendtoDeck(g,nil,SEQ_DECKTOP,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::send_to_deck(d, g, None, SEQ_DECKTOP as i32, REASON_EFFECT, None) > 0 {
        // local og=Duel.GetOperatedGroup()
        let og = duel::get_operated_group(d);
        // if not og:IsExists(Card.IsLocation,1,nil,LOCATION_DECK|LOCATION_EXTRA) then return end
        if !group::is_exists(d, &og, &|d, c| card::is_location(d, c, LOCATION_DECK | LOCATION_EXTRA), 1, NO_EX) {
            return;
        }
        // local dg=og:Filter(Card.IsLocation,nil,LOCATION_DECK)
        let dg = group::filter(d, &og, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX);
        // local ct=dg:FilterCount(Card.IsControler,nil,tp)
        let ct = group::filter_count(d, &dg, &|d, c| card::is_controler(d, c, tp), NO_EX);
        // if ct>0 then Duel.ShuffleDeck(tp) end
        if ct > 0 {
            duel::shuffle_deck(d, tp);
        }
        // if #dg>ct then Duel.ShuffleDeck(1-tp) end
        if dg.len() > ct {
            duel::shuffle_deck(d, 1 - tp);
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,5,REASON_EFFECT)
        duel::draw(d, tp, 5, REASON_EFFECT);
        // Duel.Draw(1-tp,5,REASON_EFFECT)
        duel::draw(d, 1 - tp, 5, REASON_EFFECT);
    }
}
/// `s.cbfilter(c,tp)`
fn cbfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_CRYSTAL_BEAST) and c:IsAbleToHand()
    //     and Duel.IsExistingMatchingCard(s.fsfilter,tp,LOCATION_DECK,0,1,c)
    card::is_monster(d, c)
        && card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16])
        && card::is_able_to_hand(d, c, None)
        && duel::is_existing_matching_card(d, Some(&fsfilter), tp, LOCATION_DECK, 0, 1, c.into())
}
/// `s.fsfilter(c)`
fn fsfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_FIELD) and c:IsSpell() and c:IsAbleToHand()
    card::is_type(d, c, TYPE_FIELD) && card::is_spell(d, c) && card::is_able_to_hand(d, c, None)
}
/// The condition of the search effect: `function(_,tp) return Duel.GetFlagEffect(tp,id+1)==0 end`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_flag_effect(d, a.tp, ID + 1) == 0
}
/// `s.thtg(e,tp,...,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cbfilter,tp,LOCATION_DECK,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| cbfilter(d, c, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 2, tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,...)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetFlagEffect(tp,id+1)~=0 then return end
    if duel::get_flag_effect(d, tp, ID + 1) != 0 {
        return;
    }
    // Duel.RegisterFlagEffect(tp,id+1,0,0,0)
    duel::register_flag_effect(d, tp, ID + 1, 0, 0, 0, 0);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cbfilter,tp,LOCATION_DECK,0,1,1,nil,tp)
    let mut g = duel::select_matching_card(d, tp, Some(&|d, c| cbfilter(d, c, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g<1 then return end
    if g.is_empty() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // g=g+Duel.SelectMatchingCard(tp,s.fsfilter,tp,LOCATION_DECK,0,1,1,g)
    let second = duel::select_matching_card(d, tp, Some(&fsfilter), tp, LOCATION_DECK, 0, 1, 1, false, (&g).into()).nil();
    group::add_card(&mut g, &second);
    // if #g<2 then return end
    if g.len() < 2 {
        return;
    }
    // Duel.SendtoHand(g,nil,REASON_EFFECT)
    duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, g);
}
