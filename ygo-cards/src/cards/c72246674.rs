//! Gladiator Beast Dareios
use super::prelude::*;

pub const ID: u32 = 72246674;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GLADIATOR_BEAST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2 monsters, including a "Gladiator Beast" monster
    // Link.AddProcedure(c,nil,2,2,s.lcheck)
    link::add_procedure(d, c, None, 2, Some(2), Some(group_check(lcheck)), None, None);
    // Special Summon 1 Level 4 or lower "Gladiator Beast" monster from your hand or GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsLinkSummoned() end)
    effect::set_condition(d, e1, Some(condition(e1_cond)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // "Gladiator Beast" monsters you control cannot be destroyed by battle or card effects during your opponent's Battle Phase
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetCondition(function(e) return Duel.IsBattlePhase() and Duel.IsTurnPlayer(1-e:GetHandlerPlayer()) end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_GLADIATOR_BEAST))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_GLADIATOR_BEAST as u16]))));
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.lcheck(g,lc,sumtype,tp)
fn lcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(Card.IsSetCard,1,nil,SET_GLADIATOR_BEAST,lc,sumtype,tp)
    group::is_exists(d, g, &|d, c| card::is_set_card_ctx(d, c, &[SET_GLADIATOR_BEAST as u16], Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
// function s.spfilter(c,e,tp,opp_chk)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, opp_chk: bool) -> bool {
    // return c:IsSetCard(SET_GLADIATOR_BEAST) and c:IsCanBeSpecialSummoned(e,100,tp,false,false)
    //     and ((c:IsLevelBelow(4) and c:IsLocation(LOCATION_HAND|LOCATION_GRAVE))
    //     or (opp_chk and c:IsLocation(LOCATION_DECK)))
    card::is_set_card(d, c, &[SET_GLADIATOR_BEAST as u16])
        && card::is_can_be_special_summoned(d, c, e, 100, tp, false, false, None, None, None)
        && ((card::is_level_below(d, c, 4) && card::is_location(d, c, LOCATION_HAND | LOCATION_GRAVE))
            || (opp_chk && card::is_location(d, c, LOCATION_DECK)))
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_DECK,0,1,nil,e,tp,Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0) end
    if a.chk == Some(0) {
        let opp = duel::get_field_group_count(d, 0, 0, LOCATION_MZONE) > 0;
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(
                d,
                Some(&|d, c| spfilter(d, c, a.e, a.tp, opp)),
                a.tp,
                LOCATION_HAND | LOCATION_GRAVE | LOCATION_DECK,
                0,
                1,
                NO_EX,
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE | LOCATION_DECK) as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_DECK,0,1,1,nil,e,tp,Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0):GetFirst()
        let opp = duel::get_field_group_count(d, 0, 0, LOCATION_MZONE) > 0;
        let g = duel::select_matching_card(
            d,
            a.tp,
            Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp, opp))),
            a.tp,
            LOCATION_HAND | LOCATION_GRAVE | LOCATION_DECK,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        let sc = group::get_first(d, &g);
        // if sc and Duel.SpecialSummon(sc,100,tp,tp,false,false,POS_FACEUP)>0 then
        if let Some(sc) = sc {
            if duel::special_summon(d, CardSet::from_card(sc), 100, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
                // sc:RegisterFlagEffect(sc:GetOriginalCode(),RESET_EVENT|RESETS_STANDARD_DISABLE,0,1)
                let code = card::get_original_code(d, sc);
                card::register_flag_effect(d, sc, code, RESET_EVENT | RESETS_STANDARD_DISABLE, 0, 1, 0, 0);
            }
        }
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // You cannot use monsters as Link Material for the rest of this turn, except "Gladiator Beast" monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_LINK_MATERIAL)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_LINK_MATERIAL);
    // e1:SetTargetRange(0xff,0xff)
    effect::set_target_range(d, e1, 0xff, 0xff);
    // e1:SetTarget(function(e,c) return not c:IsSetCard(SET_GLADIATOR_BEAST) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_set_card(d, c, &[SET_GLADIATOR_BEAST as u16]))));
    // e1:SetValue(function(e,c) return c and c:IsControler(e:GetHandlerPlayer()) end)
    effect::set_value_fn(d, e1, effect_card_fn(|d, e, c| {
        let tp = effect::get_handler_player(d, e);
        card::is_controler(d, c, tp)
    }));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,1))
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
}
// e1 condition: function(e) return e:GetHandler():IsLinkSummoned() end
fn e1_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, c)
}
// e2 condition: function(e) return Duel.IsBattlePhase() and Duel.IsTurnPlayer(1-e:GetHandlerPlayer()) end
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_battle_phase(d, None) && duel::is_turn_player(d, 1 - effect::get_handler_player(d, a.e))
}
