//! Gouki Machine Suprex
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 29099860;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_GOUKI],
    listed_names: &[ID],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon both this card and 1 "Gouki" monster in your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 "Gouki" card from your Deck to your hand, except "Gouki Mashin Suprex"
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e) return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD) end)
    effect::set_condition(d, e2, Some(lua_fn(e2_cond)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e) return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD) end`
fn e2_cond(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_previous_location(d, h, LOCATION_ONFIELD)
}
/// `s.spcostfilter(c,e,tp)`
fn spcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GOUKI) and not c:IsPublic() and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_GOUKI as u16])
        && !card::is_public(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:IsPublic() and Duel.IsExistingMatchingCard(s.spcostfilter,tp,LOCATION_HAND,0,1,c,e,tp) end
    if a.chk == Some(0) {
        return !card::is_public(d, c)
            && duel::is_existing_matching_card(d, Some(&|d: &mut Duel, x| spcostfilter(d, x, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
    // local rc=Duel.SelectMatchingCard(tp,s.spcostfilter,tp,LOCATION_HAND,0,1,1,c,e,tp):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&|d: &mut Duel, x| spcostfilter(d, x, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, 1, false, c.into()).nil();
    let rc = group::get_first(d, &sel).nil();
    // e:SetLabelObject(rc)
    effect::set_label_object(d, a.e, Value::Card(rc));
    // rc:CreateEffectRelation(e)
    card::create_effect_relation(d, rc, a.e);
    // Duel.ConfirmCards(1-tp,Group.FromCards(c,rc))
    duel::confirm_cards(d, 1 - a.tp, from_cards(c, Some(rc)));
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, a.tp);
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and Duel.GetLocationCount(tp,LOCATION_MZONE)>=2
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT as u32).is_none()
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 >= 2
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,Group.FromCards(c,e:GetLabelObject()),2,tp,0)
    let mut g = CardSet::from_card(c);
    if let Some(rc) = effect::get_label_object(d, a.e).as_card() {
        group::add_card(&mut g, &CardSet::from_card(rc));
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 2, a.tp, 0);
    true
}
/// `s.spfilter(c,e,tp)` (the one of `spop`: relate to the effect and can be summoned)
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRelateToEffect(e) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_relate_to_effect(d, c, e) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `function(e) return e:GetHandler():IsControler(e:GetOwnerPlayer()) end`
fn e1_cond(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let h = effect::get_handler(d, e).nil();
    let owner = effect::get_owner_player(d, e);
    card::is_controler(d, h, owner)
}
/// `function(e,c) return not c:IsSetCard(SET_GOUKI) end`
fn e1_tg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    !card::is_set_card(d, c, &[SET_GOUKI as u16])
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<2 or Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 2
        || duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT as u32).is_some()
    {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local sg=Group.FromCards(c,e:GetLabelObject())
    let mut sg = CardSet::from_card(c);
    if let Some(rc) = effect::get_label_object(d, a.e).as_card() {
        group::add_card(&mut sg, &CardSet::from_card(rc));
    }
    // if sg:FilterCount(s.spfilter,nil,e,tp)==2 and Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)>0 then
    if group::filter_count(d, &sg, &|d: &mut Duel, x| spfilter2(d, x, a.e, a.tp), NO_EX) == 2
        && duel::special_summon(d, sg.clone(), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0
    {
        // for sc in sg:Iter() do
        for sc in group::iter(d, &sg) {
            // sc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,2))
            card::register_flag_effect(d, sc, ID, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 2));
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
            effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
            // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
            effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
            // e1:SetRange(LOCATION_MZONE)
            effect::set_range(d, e1, LOCATION_MZONE);
            // e1:SetAbsoluteRange(tp,1,0)
            effect::set_absolute_range(d, e1, a.tp, 1, 0);
            // e1:SetCondition(function(e) return e:GetHandler():IsControler(e:GetOwnerPlayer()) end)
            effect::set_condition(d, e1, Some(lua_fn(e1_cond)));
            // e1:SetTarget(function(e,c) return not c:IsSetCard(SET_GOUKI) end)
            effect::set_target(d, e1, Some(effect_card_fn(e1_tg)));
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // sc:RegisterEffect(e1,true)
            card::register_effect(d, sc, e1, true);
        }
    }
}
/// `Group.FromCards(a, b)` (a nil card is left out)
fn from_cards(a: CardId, b: Option<CardId>) -> CardSet {
    let mut g = CardSet::from_card(a);
    if let Some(b) = b {
        group::add_card(&mut g, &CardSet::from_card(b));
    }
    g
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GOUKI) and not c:IsCode(id) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_GOUKI as u16]) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
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
