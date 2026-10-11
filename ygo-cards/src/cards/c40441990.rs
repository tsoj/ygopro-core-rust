//! 星遺物－『星鍵』 (World Legacy - "World Key")
use super::prelude::*;

pub const ID: u32 = 40441990;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_WORLD_LEGACY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Apply a "you can Tribute Summon 1 monster during your Main Phase this turn, in addition to your Normal Summon/Set" effect
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
    // e1:SetCondition(function(e,tp) return Duel.IsPlayerCanSummon(tp) and Duel.IsPlayerCanAdditionalTributeSummon(tp) end)
    effect::set_condition(d, e1, Some(condition(summon_cond)));
    // e1:SetCost(Cost.Discard(function(c) return c:IsSetCard(SET_WORLD_LEGACY) end))
    effect::set_cost(
        d,
        e1,
        Some(cost::discard(Some(std::sync::Arc::new(|d, c, _e, _tp| card::is_set_card(d, c, &[SET_WORLD_LEGACY as u16]))), false, None, None, None)),
    );
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return 1 opponent's Link Monster that this card battles to the Extra Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e2, EVENT_BATTLE_START);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.rtexcon)
    effect::set_condition(d, e2, Some(condition(rtexcon)));
    // e2:SetTarget(s.rtextg)
    effect::set_target(d, e2, Some(cost_target(rtextg)));
    // e2:SetOperation(s.rtexop)
    effect::set_operation(d, e2, Some(operation(rtexop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,tp) return Duel.IsPlayerCanSummon(tp) and Duel.IsPlayerCanAdditionalTributeSummon(tp) end`
fn summon_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_player_can_summon(d, a.tp, None) && duel::is_player_can_additional_tribute_summon(d, a.tp)
}
/// `s.effop(e,tp,...)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // if not Duel.IsPlayerCanAdditionalTributeSummon(tp) then return end
    if !duel::is_player_can_additional_tribute_summon(d, a.tp) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,2))
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e1, aux::stringid(ID, 3));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_EXTRA_SUMMON_COUNT)
    effect::set_code(d, e1, EFFECT_EXTRA_SUMMON_COUNT);
    // e1:SetTargetRange(LOCATION_HAND,0)
    effect::set_target_range(d, e1, LOCATION_HAND, 0);
    // e1:SetValue(0x1)
    effect::set_value(d, e1, 0x1);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_EXTRA_SET_COUNT)
    effect::set_code(d, e2, EFFECT_EXTRA_SET_COUNT);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
}
/// `s.rtexcon(e,tp,...)`
fn rtexcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    // return bc and bc:IsRelateToBattle() and bc:IsControler(1-tp) and bc:IsLinkMonster()
    match card::get_battle_target(d, h) {
        Some(bc) => card::is_relate_to_battle(d, bc) && card::is_controler(d, bc, 1 - a.tp) && card::is_link_monster(d, bc),
        None => false,
    }
}
/// `s.rtextg(e,tp,...,chk)`
fn rtextg(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, h);
    // if chk==0 then return bc:IsAbleToExtra() end
    if a.chk == Some(0) {
        return card::is_able_to_extra(d, bc.nil());
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,bc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, bc.map(CardSet::from_card), 1, a.tp, 0);
    true
}
/// `s.rtexop(e,tp,...)`
fn rtexop(d: &mut Duel, a: &EffArgs) {
    // local bc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, h);
    // if bc and bc:IsRelateToBattle() then
    if let Some(bc) = bc {
        if card::is_relate_to_battle(d, bc) {
            // Duel.SendtoDeck(bc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
            duel::send_to_deck(d, bc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        }
    }
}
