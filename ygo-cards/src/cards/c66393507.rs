//! X-Krawler Neurogos
use super::prelude::*;

pub const ID: u32 = 66393507;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_KRAWLER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2 Insect monsters
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_INSECT),2,2)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_INSECT, sc, st, Some(tp)))),
        2,
        Some(2),
        None,
        None,
        None,
    );
    // "Krawler" monsters this card points to cannot be destroyed by battle
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1a, EFFECT_TYPE_FIELD);
    // e1a:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1a, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1a, LOCATION_MZONE);
    // e1a:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1a, LOCATION_MZONE, LOCATION_MZONE);
    // e1a:SetTarget(function(e,c) return c:IsSetCard(SET_KRAWLER) and e:GetHandler():GetLinkedGroup():IsContains(c) end)
    effect::set_target(d, e1a, Some(effect_card_fn(e1a_target)));
    // e1a:SetValue(1)
    effect::set_value(d, e1a, 1);
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // Gain 300 ATK/DEF
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1b, EFFECT_UPDATE_ATTACK);
    // e1b:SetValue(300)
    effect::set_value(d, e1b, 300);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // local e1c=e1b:Clone()
    let e1c = effect::clone(d, e1b);
    // e1c:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e1c, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e1c)
    card::register_effect(d, c, e1c, false);
    // If they battle your opponent's monster, any battle damage they inflict to your opponent is doubled
    // local e1d=e1a:Clone()
    let e1d = effect::clone(d, e1a);
    // e1d:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1d, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1d:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e1d, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e1d:SetTarget(s.doubledamtg)
    effect::set_target(d, e1d, Some(effect_card_fn(doubledamtg)));
    // e1d:SetValue(aux.ChangeBattleDamage(1,DOUBLE_DAMAGE))
    effect::set_value_fn(d, e1d, aux::change_battle_damage(1, DOUBLE_DAMAGE as i64));
    // c:RegisterEffect(e1d)
    card::register_effect(d, c, e1d, false);
    // Special Summon 2 "Krawler" monsters with different names from your GY in face-down Defense Position
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,c) return c:IsSetCard(SET_KRAWLER) and e:GetHandler():GetLinkedGroup():IsContains(c) end`
fn e1a_target(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_KRAWLER as u16]) && linked_contains(d, e, c)
}
/// `e:GetHandler():GetLinkedGroup():IsContains(c)`
fn linked_contains(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let h = effect::get_handler(d, e).nil();
    let linked = card::get_linked_group(d, h);
    group::is_contains(&linked, c)
}
/// `s.doubledamtg(e,c)`
fn doubledamtg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // return c:IsSetCard(SET_KRAWLER) and bc and bc:IsControler(1-e:GetHandlerPlayer()) and e:GetHandler():GetLinkedGroup():IsContains(c)
    if !card::is_set_card(d, c, &[SET_KRAWLER as u16]) {
        return false;
    }
    let Some(bc) = bc else { return false };
    let hp = effect::get_handler_player(d, e);
    card::is_controler(d, bc, 1 - hp) && linked_contains(d, e, c)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return (c:IsReason(REASON_BATTLE) or (rp==1-tp and c:IsReason(REASON_EFFECT) and c:IsPreviousControler(tp)))
    //     and c:IsPreviousPosition(POS_FACEUP)
    (card::is_reason(d, c, REASON_BATTLE)
        || (a.rp == 1 - a.tp && card::is_reason(d, c, REASON_EFFECT) && card::is_previous_controler(d, c, a.tp)))
        && card::is_previous_position(d, c, POS_FACEUP)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_KRAWLER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEDOWN_DEFENSE)
    card::is_set_card(d, c, &[SET_KRAWLER as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_GRAVE,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>=2
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and g:GetClassCount(Card.GetCode)>=2 end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 >= 2
            && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 2;
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dncheck,1,tp,HINTMSG_SPSUMMON)
    let tg = aux::select_unselect_group(
        d,
        &g,
        Some(a.e),
        a.tp,
        Some(2),
        Some(2),
        Some(&aux::dncheck()),
        1,
        a.tp,
        Some(HINTMSG_SPSUMMON as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,tg,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(tg), 2, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let mut tg = duel::get_target_cards(d, a.e).nil();
    // if #tg==0 or (#tg>1 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)) then return end
    if tg.is_empty() || (tg.len() > 1 && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some()) {
        return;
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft>0 and ft<#tg then
    if ft > 0 && ft < tg.len() as i32 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // tg=tg:Select(tp,ft,ft,nil)
        tg = group::select(d, &tg, a.tp, ft as u16, ft as u16, false, NO_EX).nil();
    }
    // if #tg>0 and Duel.SpecialSummon(tg,0,tp,tp,false,false,POS_FACEDOWN_DEFENSE)>0 then
    if !tg.is_empty() && duel::special_summon(d, tg.clone(), 0, a.tp, a.tp, false, false, POS_FACEDOWN_DEFENSE as u8, None) > 0 {
        // Duel.ConfirmCards(1-tp,tg)
        duel::confirm_cards(d, 1 - a.tp, tg);
    }
}
