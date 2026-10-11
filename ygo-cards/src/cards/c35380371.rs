//! Frozen Domain of the Ice Barrier
//! 氷結界に至る晶域
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 35380371;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ICE_BARRIER],
    ..ScriptInfo::DEFAULT
};

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
    // Your opponent cannot activate effects in response to the effects of your "Ice Barrier" monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetOperation(s.chainop)
    effect::set_operation(d, e2, Some(operation(chainop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Return 1 "Ice Barrier" monster the hand/Deck and can place 1 card on the field/GY on the bottom of the Deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_TODECK)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_TODECK);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(function(e,tp,eg) return eg:IsExists(Card.IsSummonLocation,1,nil,LOCATION_EXTRA) end)
    effect::set_condition(
        d,
        e3,
        Some(condition(|d, a| {
            let eg = a.eg.clone().nil();
            group::is_exists(
                d,
                &eg,
                &|d, c| card::is_summon_location(d, c, LOCATION_EXTRA),
                1,
                NO_EX,
            )
        })),
    );
    // e3:SetTarget(s.rettg)
    effect::set_target(d, e3, Some(cost_target(rettg)));
    // e3:SetOperation(s.retop)
    effect::set_operation(d, e3, Some(operation(retop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Reveal 3 "Ice Barrier" monsters or destroy this card
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_DESTROY);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e4, EVENT_PHASE + PHASE_END);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCondition(function(e,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e4, Some(condition(|d, a| duel::is_turn_player(d, a.tp))));
    // e4:SetTarget(s.selfdestg)
    effect::set_target(d, e4, Some(cost_target(selfdestg)));
    // e4:SetOperation(s.selfdesop)
    effect::set_operation(d, e4, Some(operation(selfdesop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.chfilter(c,tp) -- unused by the script's effects
// function s.chainop(e,tp,eg,ep,ev,re,r,rp)
fn chainop(d: &mut Duel, a: &EffArgs) {
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // if ep==tp and re:IsMonsterEffect() and rc:IsSetCard(SET_ICE_BARRIER) then
    if a.ep == a.tp
        && effect::is_monster_effect(d, re)
        && card::is_set_card(d, rc, &[SET_ICE_BARRIER as u16])
    {
        // Duel.SetChainLimit(function(_e,_rp,_tp) return _tp==_rp end)
        duel::set_chain_limit(d, lua_fn(|_d, a| a.p(2) == a.p(1)));
    }
}
// function s.icefilter(c)
fn icefilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_ICE_BARRIER) and (c:IsAbleToHand() or c:IsAbleToDeck())
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_ICE_BARRIER as u16])
        && (card::is_able_to_hand(d, c, None) || card::is_able_to_deck(d, c))
}
// function s.rettg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rettg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.icefilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&icefilter),
            a.tp,
            LOCATION_MZONE,
            0,
            1,
            NO_EX,
        );
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_MZONE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,nil,1,PLAYER_EITHER,LOCATION_ONFIELD|LOCATION_GRAVE)
    duel::set_possible_operation_info(
        d,
        0,
        CATEGORY_TODECK,
        None,
        1,
        PLAYER_EITHER as u8,
        (LOCATION_ONFIELD | LOCATION_GRAVE) as i32,
    );
    true
}
// function s.retop(e,tp,eg,ep,ev,re,r,rp)
fn retop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    // local sc=Duel.SelectMatchingCard(tp,s.icefilter,tp,LOCATION_MZONE,0,1,1,nil):GetFirst()
    let sg = duel::select_matching_card(
        d,
        tp,
        Some(&icefilter),
        tp,
        LOCATION_MZONE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let Some(sc) = group::get_first(d, &sg) else {
        // if not sc then return end
        return;
    };
    // Duel.HintSelection(sc,true)
    duel::hint_selection(d, CardSet::from_card(sc), true);
    // local b1=sc:IsAbleToHand()
    let b1 = card::is_able_to_hand(d, sc, None);
    // local b2=sc:IsAbleToDeck()
    let b2 = card::is_able_to_deck(d, sc);
    // local op=0
    // if sc:IsAbleToExtra() then
    let op: Option<i64> = if card::is_able_to_extra(d, sc) {
        // op=2
        Some(2)
    } else {
        // op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,3)},{b2,aux.Stringid(id,4)})
        duel::select_effect(
            d,
            tp,
            &[(b1, aux::stringid(ID, 3)), (b2, aux::stringid(ID, 4))],
        )
        .map(|x| x as i64)
    };
    // local func=nil
    // local params={}
    // local shuffle_func=nil
    // if op==1 then
    //     func=Duel.SendtoHand
    //     params={nil,REASON_EFFECT}
    //     shuffle_func=Duel.ShuffleHand
    // elseif op==2 then
    //     func=Duel.SendtoDeck
    //     params={nil,SEQ_DECKSHUFFLE,REASON_EFFECT}
    //     shuffle_func=Duel.ShuffleDeck
    // end
    // `func(sc,table.unpack(params))` is called right after: a nil `func` is a Lua error
    let moved = match op {
        Some(1) => duel::send_to_hand(d, CardSet::from_card(sc), None, REASON_EFFECT, None),
        Some(2) => duel::send_to_deck(
            d,
            CardSet::from_card(sc),
            None,
            SEQ_DECKSHUFFLE as i32,
            REASON_EFFECT,
            None,
        ),
        _ => crate::lua_error("attempt to call a nil value"),
    };
    // if func(sc,table.unpack(params))==0 or not sc:IsLocation(LOCATION_HAND|LOCATION_DECK|LOCATION_EXTRA) then return end
    if moved == 0 || !card::is_location(d, sc, LOCATION_HAND | LOCATION_DECK | LOCATION_EXTRA) {
        return;
    }
    // if not sc:IsLocation(LOCATION_EXTRA) then shuffle_func(tp) end
    if !card::is_location(d, sc, LOCATION_EXTRA) {
        if op == Some(1) {
            duel::shuffle_hand(d, tp);
        } else {
            duel::shuffle_deck(d, tp);
        }
    }
    // local rg=Duel.GetMatchingGroup(aux.NecroValleyFilter(Card.IsAbleToDeck),tp,LOCATION_ONFIELD|LOCATION_GRAVE,LOCATION_ONFIELD|LOCATION_GRAVE,nil)
    let rg = duel::get_matching_group(
        d,
        Some(&aux::necro_valley_filter(|d, c| {
            card::is_able_to_deck(d, c)
        })),
        tp,
        LOCATION_ONFIELD | LOCATION_GRAVE,
        LOCATION_ONFIELD | LOCATION_GRAVE,
        NO_EX,
    );
    // if #rg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,5)) then
    if !rg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 5)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
        // local g=rg:Select(tp,1,1,nil)
        let g = group::select(d, &rg, tp, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g,true)
            duel::hint_selection(d, g.clone(), true);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoDeck(g,nil,SEQ_DECKBOTTOM,REASON_EFFECT)
            duel::send_to_deck(d, g, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
        }
    }
}
// function s.selfdestg(e,tp,eg,ep,ev,re,r,rp,chk)
fn selfdestg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,e:GetHandler(),1,tp,LOCATION_SZONE)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_possible_operation_info(
        d,
        0,
        CATEGORY_DESTROY,
        Some(CardSet::from_card(h)),
        1,
        a.tp,
        LOCATION_SZONE as i32,
    );
    true
}
// function s.rvlfilter(c)
fn rvlfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_ICE_BARRIER) and not c:IsPublic()
    card::is_monster(d, c)
        && card::is_set_card(d, c, &[SET_ICE_BARRIER as u16])
        && !card::is_public(d, c)
}
// function s.selfdesop(e,tp,eg,ep,ev,re,r,rp)
fn selfdesop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.rvlfilter,tp,LOCATION_EXTRA,0,nil)
    let g = duel::get_matching_group(d, Some(&rvlfilter), tp, LOCATION_EXTRA, 0, NO_EX);
    // if #g>=3 and aux.SelectUnselectGroup(g,e,tp,3,3,aux.dncheck,0) and Duel.SelectYesNo(tp,aux.Stringid(id,6)) then
    let can_select = g.len() >= 3
        && matches!(
            aux::select_unselect_group(
                d,
                &g,
                Some(a.e),
                tp,
                Some(3),
                Some(3),
                Some(&aux::dncheck()),
                0,
                tp,
                None,
                None,
                None,
                false
            ),
            Err(true)
        );
    if can_select && duel::select_yes_no(d, tp, aux::stringid(ID, 6)) {
        // local rvg=aux.SelectUnselectGroup(g,e,tp,3,3,aux.dncheck,1,tp,HINTMSG_CONFIRM)
        let rvg = aux::select_unselect_group(
            d,
            &g,
            Some(a.e),
            tp,
            Some(3),
            Some(3),
            Some(&aux::dncheck()),
            1,
            tp,
            Some(HINTMSG_CONFIRM as u64),
            None,
            None,
            false,
        )
        .unwrap_or_default();
        // Duel.ConfirmCards(1-tp,rvg)
        duel::confirm_cards(d, 1 - tp, &rvg);
        // Duel.ShuffleExtra(tp)
        duel::shuffle_extra(d, tp);
    } else {
        // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
        let h = effect::get_handler(d, a.e).nil();
        duel::destroy(d, h, REASON_EFFECT, None, None);
    }
}
