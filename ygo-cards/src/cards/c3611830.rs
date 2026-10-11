//! Endymion, the Mighty Master of Magic
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 3611830;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_SPELL as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_SPELL)
    card::enable_counter_permit(d, c, COUNTER_SPELL as u16, None, None);
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Special Summon from the pendulum zone
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DESTROY+CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY + CATEGORY_COUNTER);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
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
    // Negate Spell/Trap
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY+CATEGORY_COUNTER+CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_NEGATE + CATEGORY_DESTROY + CATEGORY_COUNTER + CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.negcon)
    effect::set_condition(d, e2, Some(condition(negcon)));
    // e2:SetTarget(s.negtg)
    effect::set_target(d, e2, Some(cost_target(negtg)));
    // e2:SetOperation(s.negop)
    effect::set_operation(d, e2, Some(operation(negop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Cannot be targeted
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e3, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.indcon)
    effect::set_condition(d, e3, Some(lua_fn(indcon)));
    // e3:SetValue(aux.tgoval)
    effect::set_value_fn(d, e3, aux::tgoval());
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Cannot be destroyed
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_EFFECT);
    // e4:SetValue(s.indval)
    effect::set_value_fn(d, e4, lua_fn(indval));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Register before leaving
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e5:SetCode(EVENT_LEAVE_FIELD_P)
    effect::set_code(d, e5, EVENT_LEAVE_FIELD_P);
    // e5:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e5, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e5:SetOperation(s.regop)
    effect::set_operation(d, e5, Some(operation(regop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // Search
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e6, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e6:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e6:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e6, EVENT_BATTLE_DESTROYED);
    // e6:SetLabelObject(e5)
    effect::set_label_object(d, e6, Value::Effect(e5));
    // e6:SetCondition(s.thcon)
    effect::set_condition(d, e6, Some(condition(thcon)));
    // e6:SetTarget(s.thtg)
    effect::set_target(d, e6, Some(cost_target(thtg)));
    // e6:SetOperation(s.thop)
    effect::set_operation(d, e6, Some(operation(thop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // aux.DoubleSnareValidity(c,LOCATION_MZONE)
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsCanRemoveCounter(tp,1,0,COUNTER_SPELL,6,REASON_COST) end
    if a.chk == Some(0) {
        return duel::is_can_remove_counter(d, a.tp, 1, 0, COUNTER_SPELL as u16, 6, REASON_COST);
    }
    // Duel.RemoveCounter(tp,1,0,COUNTER_SPELL,6,REASON_COST)
    duel::remove_counter(d, a.tp, 1, 0, COUNTER_SPELL as u16, 6, REASON_COST);
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and c:IsCanAddCounter(COUNTER_SPELL,1,false,LOCATION_MZONE) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)
            && card::is_can_add_counter(d, c, COUNTER_SPELL as u16, 1, false, LOCATION_MZONE);
    }
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)~=0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) != 0 {
        // local dc=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsCanAddCounter,COUNTER_SPELL,1,false,LOCATION_ONFIELD),tp,LOCATION_ONFIELD,0,nil)
        let dc = duel::get_matching_group_count(
            d,
            Some(&|d, x| card::is_faceup(d, x) && card::is_can_add_counter(d, x, COUNTER_SPELL as u16, 1, false, LOCATION_ONFIELD)),
            tp,
            LOCATION_ONFIELD,
            0,
            NO_EX,
        );
        // if dc==0 then return end
        if dc == 0 {
            return;
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,dc,nil)
        let g = duel::select_matching_card(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, dc as u16, false, NO_EX).nil();
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // local oc=Duel.Destroy(g,REASON_EFFECT)
        let oc = duel::destroy(d, g, REASON_EFFECT, None, None);
        // if oc>0 then c:AddCounter(COUNTER_SPELL,oc) end
        if oc > 0 {
            card::add_counter(d, c, COUNTER_SPELL as u16, oc as u16, false);
        }
    }
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED) and re:IsSpellTrapEffect() and Duel.IsChainNegatable(ev)
    let h = effect::get_handler(d, a.e).nil();
    !card::is_status(d, h, STATUS_BATTLE_DESTROYED)
        && effect::is_spell_trap_effect(d, a.re.nil())
        && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.negfilter(c)`
fn negfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetCounter(COUNTER_SPELL)>0 and c:IsAbleToHand()
    card::get_counter(d, c, COUNTER_SPELL as u16) > 0 && card::is_able_to_hand(d, c, None)
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.negfilter,tp,LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&negfilter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX);
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
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local hc=Duel.SelectMatchingCard(tp,s.negfilter,tp,LOCATION_ONFIELD,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&negfilter), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // if not hc then return end
    let Some(hc) = group::get_first(d, &g) else {
        return;
    };
    // local ct=hc:GetCounter(COUNTER_SPELL)
    let ct = card::get_counter(d, hc, COUNTER_SPELL as u16);
    // if Duel.SendtoHand(hc,nil,REASON_EFFECT)>0 and hc:IsLocation(LOCATION_HAND)
    //     and Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::send_to_hand(d, hc, None, REASON_EFFECT, None) > 0
        && card::is_location(d, hc, LOCATION_HAND)
        && duel::negate_activation(d, a.ev as u8)
        && {
            let re = a.re.nil();
            let rh = effect::get_handler(d, re).nil();
            card::is_relate_to_effect(d, rh, re)
        }
    {
        // if Duel.Destroy(eg,REASON_EFFECT)>0 and c:IsRelateToEffect(e) and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if duel::destroy(d, a.eg(), REASON_EFFECT, None, None) > 0
            && card::is_relate_to_effect(d, c, a.e)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // c:AddCounter(COUNTER_SPELL,ct)
            card::add_counter(d, c, COUNTER_SPELL as u16, ct as u16, false);
        }
    }
}
/// `s.indcon(e)`
fn indcon(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler():GetCounter(COUNTER_SPELL)>0
    let h = effect::get_handler(d, a.e(0)).nil();
    card::get_counter(d, h, COUNTER_SPELL as u16) > 0
}
/// `s.indval(e,re,tp)`
fn indval(d: &mut Duel, a: Args) -> bool {
    // return tp==1-e:GetHandlerPlayer()
    let hp = effect::get_handler_player(d, a.e(0));
    a.p(2) == 1 - hp
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=c:GetCounter(COUNTER_SPELL)
    let ct = card::get_counter(d, c, COUNTER_SPELL as u16);
    // e:SetLabel(ct)
    effect::set_label(d, a.e, &[ct as i64]);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsNormalSpell() and c:IsAbleToHand()
    card::is_normal_spell(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsReason(REASON_BATTLE) and e:GetLabelObject():GetLabel()>0
    if !card::is_reason(d, c, REASON_BATTLE) {
        return false;
    }
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    effect::get_label(d, lo) > 0
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
