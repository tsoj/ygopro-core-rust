//! Stars Align Above the Shrine
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 15306543;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_SHINOBIRD, 20417688], listed_card_types: &[TYPE_SPIRIT], ..ScriptInfo::DEFAULT };

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
    // Ritual Summon 1 WIND monster from the Deck by tributing Spirit monsters and or "Shinobird Tokens" you control
    // local e2=Ritual.CreateProc({handler=c,lvtype=RITPROC_GREATER,filter=aux.FilterBoolFunction(Card.IsAttribute,ATTRIBUTE_WIND),
    //     desc=aux.Stringid(id,0), matfilter=s.matfilter,location=LOCATION_DECK})
    let e2 = ritual::create_proc(
        d,
        c,
        ritual::RitualProc {
            lvtype: RITPROC_GREATER,
            filter: Some(std::sync::Arc::new(|d: &mut Duel, x: CardId| card::is_attribute(d, x, ATTRIBUTE_WIND))),
            desc: Some(aux::stringid(ID, 0)),
            matfilter: Some(ritual::MatFilt::F(std::sync::Arc::new(|d: &mut Duel, x: CardId, _e: EffectId, _tp: u8| matfilter(d, x)))),
            location: Some(LOCATION_DECK),
            ..Default::default()
        },
    );
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Activate 1 of these effects
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_TO_HAND)
    effect::set_code(d, e3, EVENT_TO_HAND);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(2)
    effect::set_count_limit(d, e3, 2, 0, 0, 0);
    // e3:SetCondition(s.effcon)
    effect::set_condition(d, e3, Some(condition(effcon)));
    // e3:SetTarget(s.efftg)
    effect::set_target(d, e3, Some(cost_target(efftg)));
    // e3:SetOperation(s.effop)
    effect::set_operation(d, e3, Some(operation(effop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.matfilter(c)
fn matfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_MZONE) and (c:IsType(TYPE_SPIRIT) or c:IsCode(TOKEN_SHINOBIRD))
    card::is_location(d, c, LOCATION_MZONE) && (card::is_type(d, c, TYPE_SPIRIT) || card::is_code(d, c, &[TOKEN_SHINOBIRD]))
}
// function s.cfilter(c,tp)
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousPosition(POS_FACEUP) and c:IsPreviousLocation(LOCATION_MZONE)
    //     and (c:GetPreviousAttributeOnField()&ATTRIBUTE_WIND)>0 and c:IsPreviousControler(tp)
    card::is_previous_position(d, c, POS_FACEUP)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && (card::get_previous_attribute_on_field(d, c) & ATTRIBUTE_WIND) > 0
        && card::is_previous_controler(d, c, tp)
}
// function s.effcon(e,tp,eg,ep,ev,re,r,rp)
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, tp), 1, NO_EX)
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsType(TYPE_SPIRIT) or c:IsRitualSpell()) and c:IsFaceup() and c:IsAbleToHand()
    (card::is_type(d, c, TYPE_SPIRIT) || card::is_ritual_spell(d, c)) && card::is_faceup(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.setfilter(c)
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(20417688) and c:IsSSetable()
    card::is_code(d, c, &[20417688]) && card::is_s_setable(d, c, false, None)
}
// function s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local b1=Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil)
    let b1 = duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    // local b2=Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil)
    let b2 = duel::is_existing_matching_card(d, Some(&setfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,2)},{b2,aux.Stringid(id,3)})
    // (not cancelable: a nil choice leaves the label nil, which does neither effect, as 0 does)
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]).unwrap_or(0) as i64;
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_TOHAND)
        effect::set_category(d, a.e, CATEGORY_TOHAND);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE|LOCATION_REMOVED)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    // elseif op==2 then
    } else if op == 2 {
        // e:SetCategory(CATEGORY_SET)
        effect::set_category(d, a.e, CATEGORY_SET);
    }
    true
}
// function s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // Add 1 of your Spirit monsters or Ritual Spells that is banished or in your GY to your hand
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil)
        let nf = aux::necro_valley_filter(|d, c| thfilter(d, c));
        let g = duel::select_matching_card(d, tp, Some(&nf), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, &g);
        }
    // elseif op==2 then
    } else if op == 2 {
        // Set 1 "Stars Align across the Milky Way" from your Deck
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
        // local sg=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,1,1,nil)
        let sg = duel::select_matching_card(d, tp, Some(&setfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #sg>0 then Duel.SSet(tp,sg) end
        if !sg.is_empty() {
            duel::s_set(d, tp, &sg, None, true);
        }
    }
}
