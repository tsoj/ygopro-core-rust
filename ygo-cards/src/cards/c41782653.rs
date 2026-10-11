//! Overtex Qoatlus
use super::prelude::*;

pub const ID: u32 = 41782653;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_EVOLUTION_PILL],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Cannot special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special summon procedure
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(lua_fn(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(lua_fn(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy 1 monster and negate Spell/Trap activation
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.negcon)
    effect::set_condition(d, e3, Some(condition(negcon)));
    // e3:SetTarget(s.negtg)
    effect::set_target(d, e3, Some(cost_target(negtg)));
    // e3:SetOperation(s.negop)
    effect::set_operation(d, e3, Some(operation(negop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Addd 1 "Evolution Pill" card to the hand
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e4, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e4, EVENT_TO_GRAVE);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetCondition(s.thcon)
    effect::set_condition(d, e4, Some(condition(thcon)));
    // e4:SetTarget(s.thtg)
    effect::set_target(d, e4, Some(cost_target(thtg)));
    // e4:SetOperation(s.thop)
    effect::set_operation(d, e4, Some(operation(thop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // aux.DoubleSnareValidity(c,LOCATION_MZONE)
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
}
/// `s.spfilter(c)`
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_DINOSAUR) and c:IsAbleToDeckAsCost()
    card::is_faceup(d, c) && card::is_race(d, c, RACE_DINOSAUR) && card::is_able_to_deck_as_cost(d, c)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    if a.card(1).is_none() {
        return true;
    }
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // local rg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_REMOVED,0,nil)
    let rg = duel::get_matching_group(d, Some(&spfilter), tp, LOCATION_REMOVED, 0, NO_EX);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and #rg>4 and aux.SelectUnselectGroup(rg,e,tp,5,5,nil,0)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && rg.len() > 4
        && matches!(aux::select_unselect_group(d, &rg, Some(e), tp, Some(5), Some(5), None, 0, tp, None, None, None, false), Err(true))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local c=e:GetHandler()
    // local g=nil
    // local rg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_REMOVED,0,nil)
    let rg = duel::get_matching_group(d, Some(&spfilter), tp, LOCATION_REMOVED, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(rg,e,tp,5,5,nil,1,tp,HINTMSG_TODECK,nil,nil,true)
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(5), Some(5), None, 1, tp, Some(HINTMSG_TODECK as u64), None, None, true).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else {
        return;
    };
    // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_COST)
    duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_COST, None);
    // g:DeleteGroup()
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // return not e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED) and ep~=tp
    //     and re:IsHasType(EFFECT_TYPE_ACTIVATE) and Duel.IsChainNegatable(ev)
    let h = effect::get_handler(d, e).nil();
    !card::is_status(d, h, STATUS_BATTLE_DESTROYED)
        && a.ep != tp
        && effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE)
        && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsRace(RACE_DINOSAUR) and (c:IsLocation(LOCATION_HAND) or c:IsFaceup())
    card::is_monster(d, c) && card::is_race(d, c, RACE_DINOSAUR) && (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
}
/// `s.pubfilter(c)`
fn pubfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPublic() and c:IsMonster() and c:IsRace(RACE_DINOSAUR)
    card::is_public(d, c) && card::is_monster(d, c) && card::is_race(d, c, RACE_DINOSAUR)
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.desfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&desfilter), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    // local hg=Duel.GetFieldGroup(tp,LOCATION_HAND,0)
    let hg = duel::get_field_group(d, tp, LOCATION_HAND, 0);
    // if #hg==0 or (hg:FilterCount(Card.IsPublic,nil)==#hg and hg:IsExists(s.pubfilter,1,nil)) then
    let hn = hg.len();
    if hn == 0
        || (group::filter_count(d, &hg, &|d, c| card::is_public(d, c), NO_EX) == hn && group::is_exists(d, &hg, &pubfilter, 1, NO_EX))
    {
        // local dg=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_MZONE,0,nil)
        let dg = duel::get_matching_group(d, Some(&desfilter), tp, LOCATION_MZONE, 0, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,1,tp,LOCATION_HAND|LOCATION_MZONE)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), 1, tp, (LOCATION_HAND | LOCATION_MZONE) as i32);
    } else {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,nil,1,tp,LOCATION_HAND|LOCATION_MZONE)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, None, 1, tp, (LOCATION_HAND | LOCATION_MZONE) as i32);
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsDestructable() and re:GetHandler():IsRelateToEffect(re) then
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    if card::is_destructable(d, rh, None) && card::is_relate_to_effect(d, rh, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g1=Duel.SelectMatchingCard(tp,s.desfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil)
    let g1 = duel::select_matching_card(d, tp, Some(&desfilter), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if #g1>0 and Duel.Destroy(g1,REASON_EFFECT)~=0 then
    if !g1.is_empty() && duel::destroy(d, g1, REASON_EFFECT, None, None) != 0 {
        // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
        let re = a.re.nil();
        let rh = effect::get_handler(d, re).nil();
        if duel::negate_activation(d, a.ev as u8) && card::is_relate_to_effect(d, rh, re) {
            // Duel.Destroy(eg,REASON_EFFECT)
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsReason(REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    card::is_reason(d, h, REASON_EFFECT)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_EVOLUTION_PILL) and c:IsSpell() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_EVOLUTION_PILL as u16]) && card::is_spell(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
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
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
