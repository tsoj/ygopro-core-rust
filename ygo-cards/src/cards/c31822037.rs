//! Purification Power Patron
use super::prelude::*;

pub const ID: u32 = 31822037;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_POWER_PATRON as u32, SET_THEOREALIZE as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2 non-Link Monsters
    // Link.AddProcedure(c,s.matfilter,2)
    link::add_procedure(d, c, Some(mat_filter(matfilter)), 2, None, None, None, None);
    // If this card is Link Summoned and you have a "Power Patron" monster in your GY: You can add 1 "Theorealize" card from your Deck to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsLinkSummoned() end)
    effect::set_condition(
        d,
        e1,
        Some(lua_fn(|d, a: Args| {
            let h = effect::get_handler(d, a.e(0)).nil();
            card::is_link_summoned(d, h)
        })),
    );
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During the Main Phase, if you control another DARK Fairy monster, you can (Quick Effect): Immediately after this effect resolves, Link Summon using materials you control, including this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.linkcon)
    effect::set_condition(d, e2, Some(condition(linkcon)));
    // e2:SetTarget(s.linktg)
    effect::set_target(d, e2, Some(cost_target(linktg)));
    // e2:SetOperation(s.linkop)
    effect::set_operation(d, e2, Some(operation(linkop)));
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.matfilter(c,lc,sumtyp,sumpl)`
fn matfilter(d: &mut Duel, c: CardId, lc: Option<CardId>, sumtyp: u64, sumpl: u8) -> bool {
    // return not c:IsType(TYPE_LINK,lc,sumtyp,sumpl)
    !card::is_type_ctx(d, c, TYPE_LINK, lc, sumtyp, Some(sumpl))
}
/// `s.thconfilter(c)`
fn thconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_POWER_PATRON) and c:IsMonster()
    card::is_set_card(d, c, &[SET_POWER_PATRON as u16]) && card::is_monster(d, c)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_THEOREALIZE) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_THEOREALIZE as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,...,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thconfilter,tp,LOCATION_GRAVE,0,1,nil)
    //   and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thconfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,...)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
/// `s.linkconfilter(c)`
fn linkconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK) and c:IsRace(RACE_FAIRY) and c:IsFaceup()
    card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_race(d, c, RACE_FAIRY) && card::is_faceup(d, c)
}
/// `s.linkcon(e,tp,...)`
fn linkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsMainPhase() and Duel.IsExistingMatchingCard(s.linkconfilter,tp,LOCATION_MZONE,0,1,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    duel::is_main_phase(d, None) && duel::is_existing_matching_card(d, Some(&linkconfilter), a.tp, LOCATION_MZONE, 0, 1, h.into())
}
/// `s.linktg(e,tp,...,chk)`
fn linktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsLinkSummonable,tp,LOCATION_EXTRA,0,1,nil,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&|d, x| card::is_link_summonable(d, x, Some(CardSet::from_card(c)), None, 0, 0)),
            tp,
            LOCATION_EXTRA,
            0,
            1,
            NO_EX,
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.linkop(e,tp,...)`
fn linkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsControler(1-tp) then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_controler(d, c, 1 - tp) {
        return;
    }
    // local g=Duel.GetMatchingGroup(Card.IsLinkSummonable,tp,LOCATION_EXTRA,0,nil,c)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_link_summonable(d, x, Some(CardSet::from_card(c)), None, 0, 0)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // Duel.LinkSummon(tp,sg:GetFirst(),c)
        let sc = group::get_first(d, &sg).nil();
        duel::link_summon(d, tp, sc, Some(CardSet::from_card(c)), None, 0, 0);
    }
}
