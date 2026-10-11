//! D/D Defense Soldier
use super::prelude::*;

pub const ID: u32 = 93317313;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DD, SET_DDD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 "D/D" Pendulum Monster from your face-up Extra Deck or GY to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_DD) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_DD as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.limcostfilter(c)`
fn limcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_DD) and c:IsMonster() and c:IsAbleToRemoveAsCost()
    card::is_set_card(d, c, &[SET_DD as u16]) && card::is_monster(d, c) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_PZONE) and chkc:IsControler(tp) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_PZONE) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, a.e, tp);
    }
    // local b1=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_PZONE,0,1,nil,e,tp)
    let b1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_PZONE, 0, 1, NO_EX);
    // local b2=not Duel.HasFlagEffect(tp,id)
    //     and Duel.IsExistingMatchingCard(s.limcostfilter,tp,LOCATION_GRAVE,0,1,nil)
    let b2 = !duel::has_flag_effect(d, tp, ID, None)
        && duel::is_existing_matching_card(d, Some(&limcostfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,2)}, {b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
    let op = op.map_or(0, |x| x as i64);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_PZONE,0,1,1,nil,e,tp)
        let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_PZONE, 0, 1, 1, false, NO_EX).nil();
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, tp, 0);
    } else if op == 2 {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
        // e:SetProperty(0)
        effect::set_property(d, a.e, 0, 0);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.limcostfilter,tp,LOCATION_GRAVE,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&limcostfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // Duel.Remove(g,POS_FACEUP,REASON_COST)
        duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // Special Summon 1 "D/D" card in your Pendulum Zone
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if tc:IsRelateToEffect(e) then
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, CardSet::from_card(tc), 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    } else if op == 2 {
        // if Duel.HasFlagEffect(tp,id) then return end
        if duel::has_flag_effect(d, tp, ID, None) {
            return;
        }
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID, RESET_PHASE + PHASE_END, 0, 1, 0);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(aux.Stringid(id,4))
        effect::set_description(d, e1, aux::stringid(ID, 4));
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
        effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetTargetRange(0,1)
        effect::set_target_range(d, e1, 0, 1);
        // e1:SetCondition(s.actcon)
        effect::set_condition(d, e1, Some(lua_fn(actcon)));
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE + PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
}
/// `s.actcon(e)`
fn actcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // local tc=Duel.GetAttacker()
    // return tc and tc:IsSetCard(SET_DDD) and tc:IsControler(e:GetHandlerPlayer())
    match duel::get_attacker(d) {
        Some(tc) => card::is_set_card(d, tc, &[SET_DDD as u16]) && card::is_controler(d, tc, effect::get_handler_player(d, e)),
        None => false,
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_DD) and c:IsType(TYPE_PENDULUM) and c:IsFaceup() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_DD as u16])
        && card::is_type(d, c, TYPE_PENDULUM)
        && card::is_faceup(d, c)
        && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_EXTRA|LOCATION_GRAVE,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_EXTRA | LOCATION_GRAVE, 0, 1, h.into());
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_EXTRA|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, (LOCATION_EXTRA | LOCATION_GRAVE) as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_EXTRA|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(thfilter)),
        tp,
        LOCATION_EXTRA | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
