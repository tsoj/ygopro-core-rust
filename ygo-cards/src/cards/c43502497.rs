//! Pendulum Witch
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 43502497;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Add 1 Pendulum Monster from the Deck to the Extra Deck face-up
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOEXTRA)
    effect::set_category(d, e1, CATEGORY_TOEXTRA);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e1:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e1, EVENT_DESTROYED);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.tetg)
    effect::set_target(d, e1, Some(cost_target(tetg)));
    // e1:SetOperation(s.teop)
    effect::set_operation(d, e1, Some(operation(teop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy this card and 1 card in the Pendulum Zone
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e2a=e2:Clone()
    let e2a = effect::clone(d, e2);
    // e2a:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2a, EVENT_SPSUMMON_SUCCESS);
    // e2a:SetCondition(function(e) return e:GetHandler():IsPendulumSummoned() end)
    effect::set_condition(d, e2a, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_pendulum_summoned(d, h)
    })));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // Place this card in the Pendulum Zone
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetCondition(s.pencon)
    effect::set_condition(d, e3, Some(condition(pencon)));
    // e3:SetTarget(s.pentg)
    effect::set_target(d, e3, Some(cost_target(pentg)));
    // e3:SetOperation(s.penop)
    effect::set_operation(d, e3, Some(operation(penop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.teconfilter(c,tp)`
fn teconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsType(TYPE_FUSION|TYPE_SYNCHRO|TYPE_XYZ) and c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousPosition(POS_FACEUP)
    //     and c:IsPreviousControler(tp) and (c:IsReason(REASON_BATTLE) or (c:IsReason(REASON_EFFECT) and c:GetReasonPlayer()==1-tp))
    card::is_type(d, c, TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_position(d, c, POS_FACEUP)
        && card::is_previous_controler(d, c, tp)
        && (card::is_reason(d, c, REASON_BATTLE) || (card::is_reason(d, c, REASON_EFFECT) && card::get_reason_player(d, c) == 1 - tp))
}
/// `s.tefilter(c,rc)`
fn tefilter(d: &mut Duel, c: CardId, rc: u64) -> bool {
    // return c:IsType(TYPE_PENDULUM) and c:IsOriginalRace(rc) and not c:IsForbidden()
    card::is_type(d, c, TYPE_PENDULUM) && card::is_original_race(d, c, rc) && !card::is_forbidden(d, c)
}
/// `s.tetg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tetg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=eg:Filter(s.teconfilter,nil,tp)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| teconfilter(d, c, tp), NO_EX);
    // local rc=g:GetBitwiseOr(Card.GetOriginalRace)
    let mut rc: u64 = 0;
    for tc in group::iter(d, &g) {
        rc |= card::get_original_race(d, tc);
    }
    // if chk==0 then return #g>0 and rc>0 and Duel.IsExistingMatchingCard(s.tefilter,tp,LOCATION_DECK,0,1,nil,rc) end
    if a.chk == Some(0) {
        return !g.is_empty() && rc > 0 && duel::is_existing_matching_card(d, Some(&|d, c| tefilter(d, c, rc)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetTargetParam(rc)
    duel::set_target_param(d, rc as u32);
    // Duel.SetOperationInfo(0,CATEGORY_TOEXTRA,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOEXTRA, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.teop(e,tp,...)`
fn teop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local rc=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let rc = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int() as u64;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tefilter,tp,LOCATION_DECK,0,1,1,nil,rc)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| tefilter(d, c, rc)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoExtraP(g,tp,REASON_EFFECT)
        duel::send_to_extra_p(d, g, None, REASON_EFFECT, None);
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelBelow(4) and c:IsType(TYPE_PENDULUM) and c:IsAbleToHand()
    card::is_level_below(d, c, 4) && card::is_type(d, c, TYPE_PENDULUM) && card::is_able_to_hand(d, c, None)
}
/// `s.destg(e,tp,...,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_PZONE) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_PZONE);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,LOCATION_PZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, LOCATION_PZONE, 0, 1, NO_EX) && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local dg=Duel.SelectTarget(tp,nil,tp,LOCATION_PZONE,0,1,1,nil)
    let dg = duel::select_target(d, tp, None, tp, LOCATION_PZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg+e:GetHandler(),2,0,0)
    let h = effect::get_handler(d, a.e).nil();
    let mut dgh = dg;
    group::add_card(&mut dgh, &CardSet::from_card(h));
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dgh), 2, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.desop(e,tp,...)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) and Duel.Destroy(Group.FromCards(c,tc),REASON_EFFECT)==2 then
    if card::is_relate_to_effect(d, c, e) && card::is_relate_to_effect(d, tc.nil(), e) && duel::destroy(d, CardSet::from_iter([c, tc.nil()]), REASON_EFFECT, None, None) == 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
/// `s.pencon(e,tp,...)`
fn pencon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsFaceup()
    card::is_previous_location(d, c, LOCATION_MZONE) && card::is_faceup(d, c)
}
/// `s.pentg(e,tp,...,chk)`
fn pentg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckPendulumZones(tp) end
    if a.chk == Some(0) {
        return duel::check_pendulum_zones(d, a.tp);
    }
    true
}
/// `s.penop(e,tp,...)`
fn penop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not Duel.CheckPendulumZones(tp) then return end
    if !duel::check_pendulum_zones(d, tp) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    //     Duel.MoveToField(c,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
    // end
    if card::is_relate_to_effect(d, c, a.e) {
        duel::move_to_field(d, c, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
