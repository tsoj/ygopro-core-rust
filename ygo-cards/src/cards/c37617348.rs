//! Rescue-ACE Hydrant
use super::prelude::*;

pub const ID: u32 = 37617348;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RESCUE_ACE], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot be targeted for attacks
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_BATTLE_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_BATTLE_TARGET);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.con)
    effect::set_condition(d, e1, Some(lua_fn(con)));
    // e1:SetValue(aux.imval2)
    effect::set_value_fn(d, e1, aux::imval2());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot be targeted by the opponent's effects
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetValue(aux.tgoval)
    effect::set_value_fn(d, e2, aux::tgoval());
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Can activate Spells/Traps the turn they're Set
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e3, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e3:SetCode(EFFECT_TRAP_ACT_IN_SET_TURN)
    effect::set_code(d, e3, EFFECT_TRAP_ACT_IN_SET_TURN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(LOCATION_SZONE,0)
    effect::set_target_range(d, e3, LOCATION_SZONE, 0);
    // e3:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTarget(function(_,c) return c:HasFlagEffect(id) end)
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::has_flag_effect(d, c, ID, None))));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_QP_ACT_IN_SET_TURN)
    effect::set_code(d, e4, EFFECT_QP_ACT_IN_SET_TURN);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Search 1 "Rescue-ACE" monster
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e5, aux::stringid(ID, 0));
    // e5:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e5, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e5:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e5, 1, ID, 1, 0);
    // e5:SetTarget(s.thtg)
    effect::set_target(d, e5, Some(cost_target(thtg)));
    // e5:SetOperation(s.thop)
    effect::set_operation(d, e5, Some(operation(thop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // Check for Spells/Traps Set by the effect of a "Rescue-ACE" card
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, move |d| {
        // local ge=Effect.CreateEffect(c)
        let ge = effect::create_effect(d, c);
        // ge:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge:SetCode(EVENT_SSET)
        effect::set_code(d, ge, EVENT_SSET);
        // ge:SetOperation(s.checkop)
        effect::set_operation(d, ge, Some(operation(checkop)));
        // Duel.RegisterEffect(ge,0)
        duel::register_effect(d, ge, 0);
    });
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // if not (re and re:GetHandler():IsSetCard(SET_RESCUE_ACE)) then return end
    let Some(re) = a.re else { return };
    let rh = effect::get_handler(d, re).nil();
    if !card::is_set_card(d, rh, &[SET_RESCUE_ACE as u16]) {
        return;
    }
    // local sg=eg:Filter(Card.IsType,nil,TYPE_QUICKPLAY|TYPE_TRAP)
    let eg = a.eg.clone().nil();
    let sg = group::filter(d, &eg, &|d, c| card::is_type(d, c, (TYPE_QUICKPLAY | TYPE_TRAP) as u32), NO_EX);
    // for ec in sg:Iter() do
    for ec in group::iter(d, &sg) {
        // ec:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        card::register_flag_effect(d, ec, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    }
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_RESCUE_ACE) and not c:IsCode(id)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_RESCUE_ACE as u16]) && !card::is_code(d, c, &[ID])
}
/// `s.con(e)`
fn con(d: &mut Duel, a: Args) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    let p = effect::get_handler_player(d, a.e(0));
    duel::is_existing_matching_card(d, Some(&cfilter), p, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_RESCUE_ACE) and c:IsMonster() and not c:IsCode(id) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_RESCUE_ACE as u16]) && card::is_monster(d, c) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
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
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g.clone());
    }
}
