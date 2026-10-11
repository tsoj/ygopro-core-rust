//! Magikey Mechmusket - Batosbuster
use super::prelude::*;

pub const ID: u32 = 19489718;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_MAGIKEY],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --Add 1 "Magikey" card from your Deck to hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsRitualSummoned() end)
    effect::set_condition(d, e1, Some(condition(cond1)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Place cards on bottom of Deck to negate effects of attacking monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_DISABLE+CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_DISABLE + CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(cost_target(atktg)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e) return e:GetHandler():IsRitualSummoned() end`
fn cond1(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_ritual_summoned(d, h)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MAGIKEY) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_MAGIKEY as u16]) && card::is_able_to_hand(d, c, None)
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
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and (c:IsType(TYPE_NORMAL) or c:IsSetCard(SET_MAGIKEY))
    card::is_monster(d, c) && (card::is_type(d, c, TYPE_NORMAL) || card::is_set_card(d, c, &[SET_MAGIKEY as u16]))
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.atkfilter,tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&atkfilter), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    // if bc and bc:IsFaceup() then
    if let Some(bc) = card::get_battle_target(d, c) {
        if card::is_faceup(d, bc) {
            // local att=0
            let mut att = 0u32;
            // for gc in g:Iter() do att=att|gc:GetAttribute() end
            for gc in group::iter(d, &g) {
                att |= card::get_attribute(d, gc);
            }
            // return not bc:IsStatus(STATUS_DISABLED) and bc:GetAttribute()&att~=0
            return !card::is_status(d, bc, STATUS_DISABLED) && card::get_attribute(d, bc) & att != 0;
        }
    }
    false
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&card::is_able_to_deck), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // local tc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_battle_target(d, h);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,tc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, tc.map(CardSet::from_card), 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 0);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=c:GetBattleTarget()
    let tc = card::get_battle_target(d, c);
    // local hg=Duel.GetFieldGroup(tp,LOCATION_HAND,0)
    let hg = duel::get_field_group(d, a.tp, LOCATION_HAND, 0);
    // if #hg==0 then return end
    if hg.is_empty() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToDeck,tp,LOCATION_HAND,0,1,#hg,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&card::is_able_to_deck), a.tp, LOCATION_HAND, 0, 1, hg.len() as u16, false, NO_EX).nil();
    // if #g>0 and Duel.SendtoDeck(g,tp,SEQ_DECKBOTTOM,REASON_EFFECT)>0 and g:FilterCount(Card.IsLocation,nil,LOCATION_DECK)==#g
    //     and tc and tc:IsRelateToBattle() and tc:IsFaceup() and tc:IsControler(1-tp)
    //     and not tc:IsImmuneToEffect(e) and not tc:IsStatus(STATUS_DISABLED) then
    if !g.is_empty()
        && duel::send_to_deck(d, g.clone(), Some(a.tp), SEQ_DECKBOTTOM as i32, REASON_EFFECT, None) > 0
        && group::filter_count(d, &g, &|d, x| card::is_location(d, x, LOCATION_DECK), NO_EX) == g.len()
    {
        if let Some(tc) = tc {
            if card::is_relate_to_battle(d, tc)
                && card::is_faceup(d, tc)
                && card::is_controler(d, tc, 1 - a.tp)
                && !card::is_immune_to_effect(d, tc, a.e)
                && !card::is_status(d, tc, STATUS_DISABLED)
            {
                // --Negate its effects, then draw cards equal to the number of cards placed under the Deck
                // tc:NegateEffects(c,RESETS_STANDARD_PHASE_END)
                card::negate_effects(d, tc, c, Some(RESETS_STANDARD_PHASE_END), false, None);
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Draw(tp,#g,REASON_EFFECT)
                duel::draw(d, a.tp, g.len() as u32, REASON_EFFECT);
            }
        }
    }
}
