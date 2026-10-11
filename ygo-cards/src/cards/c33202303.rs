//! Earthbound Servant Geo Gremlina
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 33202303;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_EARTHBOUND], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Material
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_EARTHBOUND),aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_DARK))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_set_card_ctx(d, x, &[SET_EARTHBOUND as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_attribute_ctx(d, x, ATTRIBUTE_DARK, Some(fc), st, Some(tp)))),
        ],
    );
    // Search 1 "Earthbound" monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // 1 DARK Synchro Monster you control can attack directly this turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function() return Duel.IsAbleToEnterBP() end)
    effect::set_condition(d, e2, Some(lua_fn(|d, _a: Args| duel::is_able_to_enter_bp(d))));
    // e2:SetTarget(s.datg)
    effect::set_target(d, e2, Some(cost_target(datg)));
    // e2:SetOperation(s.daop)
    effect::set_operation(d, e2, Some(operation(daop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Inflict damage to your opponent
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetTarget(s.damtg)
    effect::set_target(d, e3, Some(cost_target(damtg)));
    // e3:SetOperation(s.damop)
    effect::set_operation(d, e3, Some(operation(damop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_EARTHBOUND) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_EARTHBOUND as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
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
/// `s.dafilter(c)`
fn dafilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_SYNCHRO) and c:IsAttribute(ATTRIBUTE_DARK) and c:IsFaceup() and not c:IsHasEffect(EFFECT_DIRECT_ATTACK)
    card::is_type(d, c, TYPE_SYNCHRO) && card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_faceup(d, c) && card::is_has_effect(d, c, EFFECT_DIRECT_ATTACK, None).is_empty()
}
/// `s.datg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn datg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.dafilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && dafilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.dafilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&dafilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.dafilter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, a.tp, Some(&dafilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.daop(e,tp,eg,ep,ev,re,r,rp)`
fn daop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // Can attack directly this turn
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(3205)
        effect::set_description(d, e1, 3205);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_DIRECT_ATTACK)
        effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.damfilter(c,e,tp)`
fn damfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local rc=c:GetReasonEffect():GetHandler()
    let re = card::get_reason_effect(d, c).nil();
    let rc = effect::get_handler(d, re).nil();
    // return rc:IsSetCard(SET_EARTHBOUND) and c:IsMonster() and c:IsPreviousLocation(LOCATION_MZONE)
    //     and c:IsPreviousControler(1-tp) and c:IsLocation(LOCATION_GRAVE|LOCATION_REMOVED)
    //     and c:IsReason(REASON_EFFECT) and c:IsCanBeEffectTarget(e) and c:GetTextAttack()>0
    card::is_set_card(d, rc, &[SET_EARTHBOUND as u16])
        && card::is_monster(d, c)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_controler(d, c, 1 - tp)
        && card::is_location(d, c, LOCATION_GRAVE | LOCATION_REMOVED)
        && card::is_reason(d, c, REASON_EFFECT)
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::get_text_attack(d, c) > 0
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // if chkc then return eg:IsContains(chkc) and s.damfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        let eg = a.eg.clone().nil();
        return group::is_contains(&eg, chkc) && damfilter(d, chkc, e, tp);
    }
    // if chk==0 then return r&REASON_EFFECT==REASON_EFFECT and eg:IsExists(s.damfilter,1,nil,e,tp) end
    if a.chk == Some(0) {
        if (a.r & REASON_EFFECT) != REASON_EFFECT {
            return false;
        }
        let eg = a.eg.clone().nil();
        return group::is_exists(d, &eg, &|d, x| damfilter(d, x, e, tp), 1, NO_EX);
    }
    // local g=eg:Filter(s.damfilter,nil,e,tp)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, x| damfilter(d, x, e, tp), NO_EX);
    // local tc=nil
    // if #g>1 then
    let tc = if g.len() > 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
        // tc=g:Select(tp,1,1,nil):GetFirst()
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        group::get_first(d, &sg)
    } else {
        // tc=g:GetFirst()
        group::get_first(d, &g)
    };
    // Duel.SetTargetCard(tc)
    let tc = tc.nil();
    duel::set_target_card(d, tc);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,tc:GetTextAttack())
    let atk = card::get_text_attack(d, tc);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, atk as i32);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Damage(1-tp,tc:GetTextAttack(),REASON_EFFECT)
        let atk = card::get_text_attack(d, tc) as i64;
        duel::damage(d, 1 - a.tp, atk, REASON_EFFECT, false, None);
    }
}
