//! Cupsy★Yummy Way
//! scripted by Zedja
use super::prelude::*;

pub const ID: u32 = 31603289;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_YUMMY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Tuner + 1 non-Tuner monster
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(aux.NOT(Card.IsLinkMonster)),1,1,aux.FilterBoolFunction(Card.IsLink,1))
    let nt: CardFilter = std::sync::Arc::new(|d: &mut Duel, c: CardId| !card::is_link_monster(d, c));
    synchro::add_procedure(
        d,
        c,
        None,
        1,
        1,
        Some(synchro::non_tuner(Some(nt))),
        1,
        1,
        Some(mat_filter(|d, c, _sc, _st, _tp| card::is_link(d, c, &[1]))),
        None,
        None,
        None,
        None,
    );
    // For this card's Synchro Summon, you can treat 1 Link-1 monster you control as a Level 1 Tuner
    // local e0a=Effect.CreateEffect(c)
    let e0a = effect::create_effect(d, c);
    // e0a:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0a, EFFECT_TYPE_FIELD);
    // e0a:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e0a, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e0a:SetCode(EFFECT_SYNCHRO_LEVEL)
    effect::set_code(d, e0a, EFFECT_SYNCHRO_LEVEL);
    // e0a:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e0a, LOCATION_EXTRA);
    // e0a:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e0a, LOCATION_MZONE, LOCATION_MZONE);
    // e0a:SetTarget(function(e,c) return c:IsLink(1) end)
    effect::set_target(d, e0a, Some(effect_card_fn(|d, _e, c| card::is_link(d, c, &[1]))));
    // e0a:SetValue(function(e,sc) return sc:IsSetCard(SET_YUMMY) and 1 or -1 end)
    effect::set_value_fn(
        d,
        e0a,
        effect_card_value(|d, _e, sc| if card::is_set_card(d, sc, &[SET_YUMMY as u16]) { 1 } else { -1 }),
    );
    // c:RegisterEffect(e0a)
    card::register_effect(d, c, e0a, false);
    // local e0b=e0a:Clone()
    let e0b = effect::clone(d, e0a);
    // e0b:SetCode(EFFECT_CANNOT_BE_SYNCHRO_MATERIAL)
    effect::set_code(d, e0b, EFFECT_CANNOT_BE_SYNCHRO_MATERIAL);
    // e0b:SetValue(function(e,sc) return sc and not sc:IsSetCard(SET_YUMMY) end)
    effect::set_value_fn(
        d,
        e0b,
        lua_fn(|d, a: Args| match a.card(1) {
            Some(sc) => !card::is_set_card(d, sc, &[SET_YUMMY as u16]),
            None => false,
        }),
    );
    // c:RegisterEffect(e0b)
    card::register_effect(d, c, e0b, false);
    // Add 2 "Yummy" monsters from your Deck to your hand, then discard 1 card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND+CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_SEARCH + CATEGORY_TOHAND + CATEGORY_HANDES);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsSynchroSummoned() end)
    effect::set_condition(d, e1, Some(condition(synchro_summoned_cond)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon up to 2 "Yummy" monsters from your GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp end)
    effect::set_condition(d, e2, Some(condition(opponent_chain_cond)));
    // e2:SetCost(Cost.SelfToExtra)
    effect::set_cost(d, e2, Some(cost::self_to_extra()));
    // e2:SetTarget(s.gysptg)
    effect::set_target(d, e2, Some(cost_target(gysptg)));
    // e2:SetOperation(s.gyspop)
    effect::set_operation(d, e2, Some(operation(gyspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e) return e:GetHandler():IsSynchroSummoned() end`
fn synchro_summoned_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_synchro_summoned(d, h)
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp end`
fn opponent_chain_cond(_d: &mut Duel, a: &EffArgs) -> bool {
    a.rp == 1 - a.tp
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_YUMMY) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_YUMMY as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,2,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 2, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 2, a.tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, 1);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,2,2,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 2, 2, false, NO_EX).nil();
    // if #g==2 and Duel.SendtoHand(g,nil,REASON_EFFECT)>0 then
    if g.len() == 2 && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
        // local dg=Duel.SelectMatchingCard(tp,Card.IsDiscardable,tp,LOCATION_HAND,0,1,1,nil,REASON_EFFECT)
        let dg = duel::select_matching_card(
            d,
            tp,
            Some(&|d, c| card::is_discardable(d, c, Some(REASON_EFFECT), None)),
            tp,
            LOCATION_HAND,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // if #dg>0 then
        if !dg.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoGrave(dg,REASON_DISCARD|REASON_EFFECT)
            duel::send_to_grave(d, dg, REASON_DISCARD | REASON_EFFECT, None, None);
        }
    }
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_YUMMY) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_YUMMY as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.gysptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn gysptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetMZoneCount(tp,e:GetHandler())>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let e = a.e;
        return duel::get_m_zone_count(d, tp, h.into(), None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.gyspop(e,tp,eg,ep,ev,re,r,rp)`
fn gyspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // ft=math.min(ft,2)
    ft = ft.min(2);
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,ft,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, ft as u16, false, NO_EX).nil();
    // if #g>0 then Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP) end
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
