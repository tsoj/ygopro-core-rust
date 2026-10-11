//! Mitsurugi Mirror
use super::prelude::*;

pub const ID: u32 = 49721684;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[19899073, 55397172, 13332685], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual Summon any Reptile Ritual Monster from your hand or GY
    // local e1=Ritual.CreateProc({handler=c,lvtype=RITPROC_GREATER,filter=function(c) return c:IsRace(RACE_REPTILE) end,
    //     matfilter=function(c) return c:IsRace(RACE_REPTILE) end,location=LOCATION_HAND|LOCATION_GRAVE})
    let e1 = ritual::create_proc(
        d,
        c,
        ritual::RitualProc {
            lvtype: RITPROC_GREATER,
            filter: Some(std::sync::Arc::new(|d: &mut Duel, x: CardId| card::is_race(d, x, RACE_REPTILE))),
            matfilter: Some(ritual::MatFilt::F(std::sync::Arc::new(|d: &mut Duel, x: CardId, _e: EffectId, _tp: u8| card::is_race(d, x, RACE_REPTILE)))),
            location: Some(LOCATION_HAND | LOCATION_GRAVE),
            ..Default::default()
        },
    );
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Shuffle this card into the Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2:SetCode(EVENT_RELEASE)
    effect::set_code(d, e2, EVENT_RELEASE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.tdcon)
    effect::set_condition(d, e2, Some(condition(tdcon)));
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.tdconfilter(c,tp)`
fn tdconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousCodeOnField(19899073,55397172,13332685) and c:IsPreviousPosition(POS_FACEUP) and c:IsPreviousControler(tp)
    //     and c:IsPreviousLocation(LOCATION_ONFIELD)
    card::is_previous_code_on_field(d, c, &[19899073, 55397172, 13332685])
        && card::is_previous_position(d, c, POS_FACEUP)
        && card::is_previous_controler(d, c, tp)
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
}
/// `s.tdcon(e,tp,eg,ep,ev,re,r,rp)`
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.tdconfilter,1,nil,tp) and not eg:IsContains(e:GetHandler())
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, x| tdconfilter(d, x, a.tp), 1, NO_EX) && {
        let c = effect::get_handler(d, a.e).nil();
        !group::is_contains(&eg, c)
    }
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToDeck() end
    if a.chk == Some(0) {
        return card::is_able_to_deck(d, c);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.HintSelection(c)
        duel::hint_selection(d, CardSet::from_card(c), true);
        // Duel.SendtoDeck(c,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, CardSet::from_card(c), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
