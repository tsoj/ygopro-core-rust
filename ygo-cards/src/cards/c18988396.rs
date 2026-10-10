//! Miracle Raven
//! ミラクル・レイヴン－供物の儀式－
use super::prelude::*;

pub const ID: u32 = 18988396;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // c:AddMustBeRitualSummoned()
    card::add_must_be_ritual_summoned(d, c);
    // Ritual Summon this card, by Tributing monsters from your hand or field whose total Levels equal or exceed 1
    // local e1=Ritual.CreateProc({handler=c,lvtype=RITPROC_GREATER,filter=function(rc) return rc==c end,lv=1,location=LOCATION_PZONE,desc=aux.Stringid(id,0),self=true})
    let e1 = ritual::create_proc(
        d,
        c,
        ritual::RitualProc {
            lvtype: RITPROC_GREATER,
            filter: Some(std::sync::Arc::new(move |_d, rc| rc == c)),
            lv: Some(ritual::Lv::N(1)),
            location: Some(LOCATION_PZONE),
            desc: Some(aux::stringid(ID, 0)),
            self_: true,
            ..Default::default()
        },
    );
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // This Ritual Summoned card is unaffected by your opponent's activated effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e2, EFFECT_IMMUNE_EFFECT);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(function(e) return e:GetHandler():IsRitualSummoned() end)
    effect::set_condition(d, e2, Some(condition(immune_cond)));
    // e2:SetValue(function(e,te) return te:IsActivated() and te:GetOwnerPlayer()==1-e:GetHandlerPlayer() end)
    effect::set_value_fn(d, e2, lua_fn(immune_value));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // If you Ritual Summon exactly 1 Ritual Monster with a card effect that requires use of monsters, this card you control can be used as the entire Tribute
    // local e3=Ritual.AddWholeLevelTribute(c,aux.TRUE)
    let e3 = ritual::add_whole_level_tribute(d, c, std::sync::Arc::new(|_d, _c, _e| true));
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // Add 1 Ritual Monster from your Deck to your hand
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
    // e4:SetCode(EVENT_RELEASE)
    effect::set_code(d, e4, EVENT_RELEASE);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetCondition(function(e) return e:GetHandler():IsReason(REASON_RITUAL) end)
    effect::set_condition(d, e4, Some(condition(e4_cond)));
    // e4:SetTarget(s.thtg)
    effect::set_target(d, e4, Some(cost_target(thtg)));
    // e4:SetOperation(s.thop)
    effect::set_operation(d, e4, Some(operation(thop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `function(e) return e:GetHandler():IsRitualSummoned() end`
fn immune_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_ritual_summoned(d, h)
}
/// `function(e,te) return te:IsActivated() and te:GetOwnerPlayer()==1-e:GetHandlerPlayer() end`
fn immune_value(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let te = a.e(1);
    effect::is_activated(d, te) && effect::get_owner_player(d, te) == 1 - effect::get_handler_player(d, e)
}
/// `function(e) return e:GetHandler():IsReason(REASON_RITUAL) end`
fn e4_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_reason(d, h, REASON_RITUAL)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRitualMonster() and c:IsAbleToHand()
    card::is_ritual_monster(d, c) && card::is_able_to_hand(d, c, None)
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
