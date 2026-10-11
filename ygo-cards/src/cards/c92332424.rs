//! 竜剣士マジェスティＰ (Majesty Pegasus, the Dracoslayer)
use super::prelude::*;

pub const ID: u32 = 92332424;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DRACOSLAYER, SET_MAJESPECTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Search 1 "Dracoslayer" Pendulum Monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Protect "Dracoslayer" monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER));
    // e2:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e2, Some(cost::self_discard()));
    // e2:SetTarget(s.pttg)
    effect::set_target(d, e2, Some(cost_target(pttg)));
    // e2:SetOperation(s.ptop)
    effect::set_operation(d, e2, Some(operation(ptop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Search 1 Field Spell
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_HANDES)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_HANDES);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.fthcon)
    effect::set_condition(d, e3, Some(condition(fthcon)));
    // e3:SetTarget(s.fthtg)
    effect::set_target(d, e3, Some(cost_target(fthtg)));
    // e3:SetOperation(s.fthop)
    effect::set_operation(d, e3, Some(operation(fthop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.thfilter(c,code)`
fn thfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return c:IsSetCard(SET_DRACOSLAYER) and c:IsType(TYPE_PENDULUM) and c:IsAbleToHand() and not c:IsCode(code)
    card::is_set_card(d, c, &[SET_DRACOSLAYER as u16]) && card::is_type(d, c, TYPE_PENDULUM) && card::is_able_to_hand(d, c, None) && !card::is_code(d, c, &[code])
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        // local pc=Duel.GetFirstMatchingCard(nil,tp,LOCATION_PZONE,0,e:GetHandler())
        let handler = effect::get_handler(d, a.e).nil();
        let pc = duel::get_first_matching_card(d, None, tp, LOCATION_PZONE, 0, handler.into());
        // return pc and (pc:IsSetCard(SET_DRACOSLAYER) or pc:IsSetCard(SET_MAJESPECTER))
        //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,pc:GetOriginalCode())
        let Some(pc) = pc else { return false };
        if !(card::is_set_card(d, pc, &[SET_DRACOSLAYER as u16]) || card::is_set_card(d, pc, &[SET_MAJESPECTER as u16])) {
            return false;
        }
        let code = card::get_original_code(d, pc);
        return duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c, code)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,tp,LOCATION_PZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, tp, LOCATION_PZONE as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local pc=Duel.GetFirstMatchingCard(nil,tp,LOCATION_PZONE,0,e:GetHandler())
    let handler = effect::get_handler(d, a.e).nil();
    let pc = duel::get_first_matching_card(d, None, tp, LOCATION_PZONE, 0, handler.into());
    // if not pc then return end
    let Some(pc) = pc else { return };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,pc:GetOriginalCode())
    let code = card::get_original_code(d, pc);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| thfilter(d, c, code)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g<1 or Duel.SendtoHand(g,nil,REASON_EFFECT)<1 then return end
    if g.is_empty() || duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) < 1 {
        return;
    }
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, &g);
    // if not Duel.IsExistingMatchingCard(nil,tp,LOCATION_PZONE,0,1,nil)
    //     or not Duel.SelectYesNo(tp,aux.Stringid(id,3)) then return end
    if !duel::is_existing_matching_card(d, None, tp, LOCATION_PZONE, 0, 1, NO_EX) || !duel::select_yes_no(d, tp, aux::stringid(ID, 3)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local dg=Duel.SelectMatchingCard(tp,nil,tp,LOCATION_PZONE,0,1,1,nil)
    let dg = duel::select_matching_card(d, tp, None, tp, LOCATION_PZONE, 0, 1, 1, false, NO_EX).nil();
    // if #dg>0 then
    if !dg.is_empty() {
        // Duel.HintSelection(dg,true)
        duel::hint_selection(d, dg.clone(), true);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Destroy(dg,REASON_EFFECT)
        duel::destroy(d, dg, REASON_EFFECT, None, None);
    }
}
/// `s.pttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFlagEffect(tp,id)==0 end
    if a.chk == Some(0) {
        return duel::get_flag_effect(d, a.tp, ID) == 0;
    }
    true
}
/// `s.ptop(e,tp,eg,ep,ev,re,r,rp)`
fn ptop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // "Dracoslayer" monsters target protection
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_DRACOSLAYER))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_DRACOSLAYER as u16]))));
    // e1:SetValue(aux.tgoval)
    effect::set_value_fn(d, e1, aux::tgoval());
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // "Dracoslayer" monsters destruction protection
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // e2:SetProperty(0)
    effect::set_property(d, e2, 0, 0);
    // e2:SetValue(aux.indoval)
    effect::set_value_fn(d, e2, aux::indoval());
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    // aux.RegisterClientHint(c,0,tp,1,0,aux.Stringid(id,4),RESET_PHASE|PHASE_END,1)
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 4)), RESET_PHASE | PHASE_END, Some(1));
}
/// `s.fthcon(e,tp,eg,ep,ev,re,r,rp)`
fn fthcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return re and re:GetHandler():IsSetCard(SET_DRACOSLAYER) or e:GetHandler():IsPendulumSummoned()
    let re_ok = match a.re {
        Some(re) => {
            let h = effect::get_handler(d, re).nil();
            card::is_set_card(d, h, &[SET_DRACOSLAYER as u16])
        }
        None => false,
    };
    if re_ok {
        return true;
    }
    let handler = effect::get_handler(d, a.e).nil();
    card::is_pendulum_summoned(d, handler)
}
/// `s.fthfilter(c)`
fn fthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_FIELD) and c:IsSpell() and c:IsAbleToHand()
    card::is_type(d, c, TYPE_FIELD) && card::is_spell(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.fthtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn fthtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.fthfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&fthfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.fthop(e,tp,eg,ep,ev,re,r,rp)`
fn fthop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.fthfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&fthfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SendtoHand(g,nil,REASON_EFFECT)>0 and g:GetFirst():IsLocation(LOCATION_HAND) then
    if !g.is_empty() && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0 {
        let first = group::get_first(d, &g).nil();
        if card::is_location(d, first, LOCATION_HAND) {
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, &g);
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, tp);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.DiscardHand(tp,nil,1,1,REASON_EFFECT|REASON_DISCARD,nil)
            duel::discard_hand(d, tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
        }
    }
}
