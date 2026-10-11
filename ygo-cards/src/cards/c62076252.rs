//! Test Bear
use super::prelude::*;

pub const ID: u32 = 62076252;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GLADIATOR_BEAST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card (from your hand)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.selfspcon)
    effect::set_condition(d, e1, Some(lua_fn(selfspcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon up to 2 "Gladiator Beast" monsters from your Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(s.deckspcost)
    effect::set_cost(d, e2, Some(cost_target(deckspcost)));
    // e2:SetTarget(s.decksptg)
    effect::set_target(d, e2, Some(cost_target(decksptg)));
    // e2:SetOperation(s.deckspop)
    effect::set_operation(d, e2, Some(operation(deckspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.selfspconfilter(c)
fn selfspconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GLADIATOR_BEAST) and c:IsSummonLocation(LOCATION_DECK|LOCATION_EXTRA) and c:IsFaceup()
    card::is_set_card(d, c, &[SET_GLADIATOR_BEAST as u16])
        && card::is_summon_location(d, c, LOCATION_DECK | LOCATION_EXTRA)
        && card::is_faceup(d, c)
}
// function s.selfspcon(e,c)
fn selfspcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.selfspconfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&selfspconfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
// function s.deckspcostfilter(c,tp,mzone_chk)
fn deckspcostfilter(d: &mut Duel, c: CardId, tp: u8, mzone_chk: bool) -> bool {
    // return c:IsSetCard(SET_GLADIATOR_BEAST) and c:IsMonster() and (c:IsLocation(LOCATION_HAND) or c:IsFaceup())
    //     and c:IsAbleToDeckOrExtraAsCost() and (mzone_chk or Duel.GetMZoneCount(tp,c)>0)
    card::is_set_card(d, c, &[SET_GLADIATOR_BEAST as u16])
        && card::is_monster(d, c)
        && (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
        && card::is_able_to_deck_or_extra_as_cost(d, c)
        && (mzone_chk || duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0)
}
// function s.deckspcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn deckspcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local mzone_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let mzone_chk = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // if chk==0 then return c:IsReleasable() and Duel.IsExistingMatchingCard(s.deckspcostfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil,tp,mzone_chk) end
    if a.chk == Some(0) {
        return card::is_releasable(d, c, None, None)
            && duel::is_existing_matching_card(
                d,
                Some(&|d, x| deckspcostfilter(d, x, a.tp, mzone_chk)),
                a.tp,
                LOCATION_HAND | LOCATION_MZONE,
                0,
                1,
                NO_EX,
            );
    }
    // Duel.Release(c,REASON_COST)
    duel::release(d, CardSet::from_card(c), REASON_COST, None);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.deckspcostfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil,tp,mzone_chk):GetFirst()
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&|d, x| deckspcostfilter(d, x, a.tp, mzone_chk)),
        a.tp,
        LOCATION_HAND | LOCATION_MZONE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let sc = group::get_first(d, &g).nil();
    // if sc:IsLocation(LOCATION_HAND) then Duel.ConfirmCards(1-tp,sc)
    if card::is_location(d, sc, LOCATION_HAND) {
        duel::confirm_cards(d, 1 - a.tp, CardSet::from_card(sc));
    } else {
        // else Duel.HintSelection(sc) end
        duel::hint_selection(d, CardSet::from_card(sc), true);
    }
    // Duel.SendtoDeck(sc,nil,SEQ_DECKSHUFFLE,REASON_COST)
    duel::send_to_deck(d, CardSet::from_card(sc), None, SEQ_DECKSHUFFLE as i32, REASON_COST, None);
    true
}
// function s.deckspfilter(c,e,tp)
fn deckspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GLADIATOR_BEAST) and c:IsCanBeSpecialSummoned(e,100,tp,false,false)
    card::is_set_card(d, c, &[SET_GLADIATOR_BEAST as u16]) && card::is_can_be_special_summoned(d, c, e, 100, tp, false, false, None, None, None)
}
// function s.decksptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn decksptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.deckspfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| deckspfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.deckspop(e,tp,eg,ep,ev,re,r,rp)
fn deckspop(d: &mut Duel, a: &EffArgs) {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // ft=math.min(ft,2)
    ft = ft.min(2);
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.deckspfilter,tp,LOCATION_DECK,0,1,ft,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| deckspfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, ft as u16, false, NO_EX).nil();
    // if #g>0 and Duel.SpecialSummon(g,100,tp,tp,false,false,POS_FACEUP)>0 then
    if !g.is_empty() && duel::special_summon(d, g.clone(), 100, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        // for sc in g:Iter() do
        for sc in group::iter(d, &g) {
            // sc:RegisterFlagEffect(sc:GetOriginalCode(),RESET_EVENT|RESETS_STANDARD_DISABLE,0,1)
            let code = card::get_original_code(d, sc);
            card::register_flag_effect(d, sc, code, RESET_EVENT | RESETS_STANDARD_DISABLE, 0, 1, 0, 0);
        }
    }
}
