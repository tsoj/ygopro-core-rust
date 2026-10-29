//! Mayosenju Hitot
use super::prelude::*;

pub const ID: u32 = 21364070;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_YOSENJU], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum Summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Change its scale to 11
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sctg)
    effect::set_target(d, e1, Some(cost_target(sctg)));
    // e1:SetOperation(s.scop)
    effect::set_operation(d, e1, Some(operation(scop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return itself to the hand during the End Phase
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e2, Some(lua_fn(e2_cond)));
    // e2:SetTarget(s.sthtg)
    effect::set_target(d, e2, Some(cost_target(sthtg)));
    // e2:SetOperation(s.sthop)
    effect::set_operation(d, e2, Some(operation(sthop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Cannot be Special Summoned, except by Pendulum Summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e3:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e3, EFFECT_SPSUMMON_CONDITION);
    // e3:SetValue(aux.penlimit)
    effect::set_value_fn(d, e3, aux::penlimit());
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Return 1 card the opponent controls to the hand
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e4, CATEGORY_TOHAND);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e4:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SUMMON_SUCCESS);
    // e4:SetTarget(s.thtg)
    effect::set_target(d, e4, Some(cost_target(thtg)));
    // e4:SetOperation(s.thop)
    effect::set_operation(d, e4, Some(operation(thop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e5, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // Increase the ATK of "Yosenju" monsters by 500
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e6, aux::stringid(ID, 3));
    // e6:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e6, CATEGORY_ATKCHANGE);
    // e6:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e6:SetCode(EVENT_TO_HAND)
    effect::set_code(d, e6, EVENT_TO_HAND);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetCondition(s.atkcond)
    effect::set_condition(d, e6, Some(condition(atkcond)));
    // e6:SetOperation(s.atkop)
    effect::set_operation(d, e6, Some(operation(atkop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // local e7=e6:Clone()
    let e7 = effect::clone(d, e6);
    // e7:SetCode(EVENT_TO_DECK)
    effect::set_code(d, e7, EVENT_TO_DECK);
    // e7:SetCondition(s.atkcond2)
    effect::set_condition(d, e7, Some(condition(atkcond2)));
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
    // Return itself to the hand during the End Phase of the turn it is Special Summoned
    // local e8=Effect.CreateEffect(c)
    let e8 = effect::create_effect(d, c);
    // e8:SetDescription(aux.Stringid(id,4))
    effect::set_description(d, e8, aux::stringid(ID, 4));
    // e8:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e8, CATEGORY_TOHAND);
    // e8:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e8, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e8:SetRange(LOCATION_MZONE)
    effect::set_range(d, e8, LOCATION_MZONE);
    // e8:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e8, EVENT_PHASE + PHASE_END);
    // e8:SetCountLimit(1)
    effect::set_count_limit(d, e8, 1, 0, 0, 0);
    // e8:SetCondition(function(e) return e:GetHandler():HasFlagEffect(id) end)
    effect::set_condition(d, e8, Some(lua_fn(e8_cond)));
    // e8:SetTarget(s.rettg)
    effect::set_target(d, e8, Some(cost_target(rettg)));
    // e8:SetOperation(s.retop)
    effect::set_operation(d, e8, Some(operation(retop)));
    // c:RegisterEffect(e8)
    card::register_effect(d, c, e8, false);
    // aux.GlobalCheck(s,function()
    //     local ge1=Effect.CreateEffect(c)
    //     ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    //     ge1:SetCode(EVENT_SPSUMMON_SUCCESS)
    //     ge1:SetLabel(id)
    //     ge1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    //     ge1:SetOperation(aux.sumreg)
    //     Duel.RegisterEffect(ge1,0)
    // end)
    aux::global_check(d, ID, |d| {
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge1, EVENT_SPSUMMON_SUCCESS);
        effect::set_label(d, ge1, &[ID as i64]);
        effect::set_property(d, ge1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_operation(d, ge1, Some(aux::sumreg()));
        duel::register_effect(d, ge1, 0);
    });
}
/// `function(e,tp) return Duel.IsTurnPlayer(tp) end`
fn e2_cond(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    duel::is_turn_player(d, tp)
}
/// `function(e) return e:GetHandler():HasFlagEffect(id) end`
fn e8_cond(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::has_flag_effect(d, h, ID, None)
}
/// `s.sctg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sctg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_PZONE) and chkc:IsControler(tp) and chkc:IsSetCard(SET_YOSENJU) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_PZONE) && card::is_controler(d, chkc, a.tp) && card::is_set_card(d, chkc, &[SET_YOSENJU as u16]);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsSetCard,tp,LOCATION_PZONE,0,1,nil,SET_YOSENJU) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_set_card(d, c, &[SET_YOSENJU as u16])), a.tp, LOCATION_PZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,Card.IsSetCard,tp,LOCATION_PZONE,0,1,1,nil,SET_YOSENJU)
    duel::select_target(d, a.tp, Some(&|d, c| card::is_set_card(d, c, &[SET_YOSENJU as u16])), a.tp, LOCATION_PZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.scop(e,tp,eg,ep,ev,re,r,rp)`
fn scop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetDescription(aux.Stringid(id,4))
    effect::set_description(d, e0, aux::stringid(ID, 4));
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e0, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e0:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e0, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e0:SetTargetRange(1,0)
    effect::set_target_range(d, e0, 1, 0);
    // e0:SetTarget(s.splimit)
    effect::set_target(d, e0, Some(effect_card_fn(splimit)));
    // e0:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e0, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e0,tp)
    duel::register_effect(d, e0, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_LSCALE)
        effect::set_code(d, e1, EFFECT_CHANGE_LSCALE);
        // e1:SetValue(11)
        effect::set_value(d, e1, 11);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_CHANGE_RSCALE)
        effect::set_code(d, e2, EFFECT_CHANGE_RSCALE);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
/// `s.splimit(e,c)`
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsSetCard(SET_YOSENJU)
    !card::is_set_card(d, c, &[SET_YOSENJU as u16])
}
/// `s.sthtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.sthop(e,tp,eg,ep,ev,re,r,rp)`
fn sthop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then Duel.SendtoHand(c,nil,REASON_EFFECT) end
    if card::is_relate_to_effect(d, c, a.e) {
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
    }
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsAbleToHand() and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_able_to_hand(d, chkc, None) && card::is_controler(d, chkc, 1 - a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToHand,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToHand,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,0,0)
    let n = group::get_count(&g) as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g>0 then Duel.SendtoHand(g,nil,REASON_EFFECT) end
    if !g.is_empty() {
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
/// `s.rtfilter(c,e,tp)`
fn rtfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousLocation(LOCATION_ONFIELD) and c:IsReason(REASON_EFFECT) and c:GetReasonPlayer()==tp
    card::is_previous_location(d, c, LOCATION_ONFIELD) && card::is_reason(d, c, REASON_EFFECT) && card::get_reason_player(d, c) == tp
}
/// `s.atkcond(e,tp,eg,ep,ev,re,r,rp)`
fn atkcond(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.rtfilter,1,nil,e,tp)  (e is not used by rtfilter)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| rtfilter(d, c, tp), 1, NO_EX)
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_YOSENJU),tp,LOCATION_MZONE,0,nil)
    let f = aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_YOSENJU as u16]));
    let g = duel::get_matching_group(d, Some(&f), a.tp, LOCATION_MZONE, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetValue(500)
        effect::set_value(d, e1, 500);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.rtfilter2(c,tp)`
fn rtfilter2(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousLocation(LOCATION_ONFIELD) and c:IsReason(REASON_EFFECT) and c:GetReasonPlayer()==tp and not c:IsLocation(LOCATION_EXTRA)
    card::is_previous_location(d, c, LOCATION_ONFIELD)
        && card::is_reason(d, c, REASON_EFFECT)
        && card::get_reason_player(d, c) == tp
        && !card::is_location(d, c, LOCATION_EXTRA)
}
/// `s.atkcond2(e,tp,eg,ep,ev,re,r,rp)`
fn atkcond2(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.rtfilter2,1,nil,tp) and not eg:IsExists(Card.IsLocation,1,nil,LOCATION_HAND)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| rtfilter2(d, c, tp), 1, NO_EX)
        && !group::is_exists(d, &eg, &|d, c| card::is_location(d, c, LOCATION_HAND), 1, NO_EX)
}
/// `s.rettg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rettg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then Duel.SendtoHand(c,nil,REASON_EFFECT) end
    if card::is_relate_to_effect(d, c, a.e) {
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
    }
}
