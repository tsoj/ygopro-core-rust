//! Arcana Force VII - The Chariot
use super::prelude::*;

pub const ID: u32 = 34568403;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ARCANA_FORCE], toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Toss a coin when this card is Summoned
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COIN)
    effect::set_category(d, e1, CATEGORY_COIN);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetTarget(s.cointg)
    effect::set_target(d, e1, Some(cost_target(cointg)));
    // e1:SetOperation(s.coinop)
    effect::set_operation(d, e1, Some(operation(coinop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_FLIP_SUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.cointg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cointg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_COIN,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, a.tp, 1);
    true
}
/// `s.coinop(e,tp,eg,ep,ev,re,r,rp)`
fn coinop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    // s.arcanareg(c,Arcana.TossCoin(c,tp))
    let coin = aux::arcana::toss_coin(d, c, a.tp);
    arcanareg(d, c, coin);
}
/// `s.arcanareg(c,coin)`
fn arcanareg(d: &mut Duel, c: CardId, coin: i32) {
    // Coin effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_ADJUST)
    effect::set_code(d, e2, EVENT_ADJUST);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetOperation(s.ctop)
    effect::set_operation(d, e2, Some(operation(ctop)));
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Arcana.RegisterCoinResult(c,coin)
    aux::arcana::register_coin_result(d, c, coin);
    // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,coin)
    card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, coin as i64, 0);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return Arcana.GetCoinResult(c)==COIN_HEADS and c:IsRelateToBattle() and c:IsStatus(STATUS_OPPO_BATTLE)
    aux::arcana::get_coin_result(d, c) == Some(COIN_HEADS as i64) && card::is_relate_to_battle(d, c) && card::is_status(d, c, STATUS_OPPO_BATTLE)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_battle_target(d, h);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and tc:IsLocation(LOCATION_GRAVE|LOCATION_REMOVED) and tc:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        let tc = tc.nil();
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_location(d, tc, LOCATION_GRAVE | LOCATION_REMOVED)
            && card::is_can_be_special_summoned(d, tc, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetTargetCard(tc)
    let tc = tc.nil();
    duel::set_target_card(d, tc);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(tc)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_battle_target(d, h);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Heads
    // if Arcana.GetCoinResult(c)==COIN_HEADS and c:GetFlagEffectLabel(id)==COIN_HEADS then
    if aux::arcana::get_coin_result(d, c) == Some(COIN_HEADS as i64) && card::get_flag_effect_label(d, c, ID).first().copied() == Some(COIN_HEADS as i64) {
        // c:SetFlagEffectLabel(id,COIN_TAILS)
        card::set_flag_effect_label(d, c, ID, COIN_TAILS as u32);
    }
    // Tails
    // if Arcana.GetCoinResult(c)==COIN_TAILS and c:GetFlagEffectLabel(id)==COIN_TAILS then
    if aux::arcana::get_coin_result(d, c) == Some(COIN_TAILS as i64) && card::get_flag_effect_label(d, c, ID).first().copied() == Some(COIN_TAILS as i64) {
        // c:SetFlagEffectLabel(id,COIN_HEADS)
        card::set_flag_effect_label(d, c, ID, COIN_HEADS as u32);
        // Duel.GetControl(c,1-tp,0,0)
        duel::get_control(d, c, 1 - a.tp, 0, 0, None, None);
    }
}
