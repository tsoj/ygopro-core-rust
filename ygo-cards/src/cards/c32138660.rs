//! Reptilianne Melusine
//! レプティレス・メルジーヌ
use super::prelude::*;

pub const ID: u32 = 32138660;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_REPTILE),1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_REPTILE, sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner(None)),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // Material check
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e0, EFFECT_MATERIAL_CHECK);
    // e0:SetValue(s.matcheck)
    effect::set_value_fn(d, e0, effect_card_value(matcheck));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Cannot be destroyed by battle
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.indcon)
    effect::set_condition(d, e1, Some(condition(indcon)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot be destroyed by card effects
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Change the ATK of 1 face-up monster the opponent controls to 0
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.atkcon)
    effect::set_condition(d, e3, Some(condition(atkcon)));
    // e3:SetTarget(s.atktg)
    effect::set_target(d, e3, Some(cost_target(atktg)));
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Search 1 Reptile monster
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
    // e4:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e4, EVENT_TO_GRAVE);
    // e4:SetCondition(s.thcon)
    effect::set_condition(d, e4, Some(condition(thcon)));
    // e4:SetTarget(s.thtg)
    effect::set_target(d, e4, Some(cost_target(thtg)));
    // e4:SetOperation(s.thop)
    effect::set_operation(d, e4, Some(operation(thop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.matcheck(e,c)`
fn matcheck(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // local mg=c:GetMaterial()
    let mg = card::get_material(d, c);
    // if #mg>0 and #mg==mg:FilterCount(Card.IsRace,nil,RACE_REPTILE) then
    let n = mg.len();
    if n > 0 && n == group::filter_count(d, &mg, &|d, cc| card::is_race(d, cc, RACE_REPTILE), NO_EX) {
        // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD&~(RESET_TOFIELD|RESET_LEAVE),EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,2))
        card::register_flag_effect(
            d,
            c,
            ID,
            RESET_EVENT | (RESETS_STANDARD & !(RESET_TOFIELD | RESET_LEAVE)),
            EFFECT_FLAG_CLIENT_HINT,
            1,
            0,
            aux::stringid(ID, 2),
        );
    }
    0
}
/// `s.indcon(e)`
fn indcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsSynchroSummoned() and c:HasFlagEffect(id)
    card::is_synchro_summoned(d, c) && card::has_flag_effect(d, c, ID, None)
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and re:IsMonsterEffect()
    a.rp == 1 - a.tp && effect::is_monster_effect(d, a.re.nil())
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:HasNonZeroAttack() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && card::has_non_zero_attack(d, chkc);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:GetFlagEffect(id+1)==0 and Duel.IsExistingTarget(Card.HasNonZeroAttack,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return card::get_flag_effect(d, c, ID + 1) == 0
            && duel::is_existing_target(d, Some(&card::has_non_zero_attack), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // c:RegisterFlagEffect(id+1,RESET_CHAIN,0,1)
    card::register_flag_effect(d, c, ID + 1, RESET_CHAIN, 0, 1, 0, 0);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,Card.HasNonZeroAttack,tp,0,LOCATION_MZONE,1,1,nil)
    let _ = duel::select_target(d, tp, Some(&card::has_non_zero_attack), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(0)
        effect::set_value(d, e1, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return rp==1-tp and c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_MZONE) and c:IsSynchroSummoned()
    a.rp == 1 - a.tp
        && card::is_previous_controler(d, c, a.tp)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_synchro_summoned(d, c)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_REPTILE) and c:IsAbleToHand()
    card::is_race(d, c, RACE_REPTILE) && card::is_able_to_hand(d, c, None)
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
