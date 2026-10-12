//! Illusion of the White Crow Witch
//! scripted by pyrQ
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 101403078;

/// `fusion_params` of `Fusion.CreateSummonEff`.
fn fusion_params() -> fusion::SummonEff {
    fusion::SummonEff {
        // fusfilter=function(c) return c:IsRace(RACE_ILLUSION) end
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_race(d, c, RACE_ILLUSION))),
        // matfilter=Fusion.OnFieldMat
        matfilter: Some(fusion::on_field_mat()),
        // extrafil=function(e,tp,mg) return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToGrave),tp,LOCATION_EXTRA,0,nil) end
        extrafil: Some(Arc::new(extrafil)),
        // extratg=function(e,tp,eg,ep,ev,re,r,rp,chk) ... end
        extratg: Some(cost_target(extratg)),
        // stage2=function(e,tc,tp,mg,chk) ... end
        stage2: Some(Arc::new(|d: &mut Duel, e: EffectId, tc: Option<CardId>, tp: u8, mg: Option<&CardSet>, chk: i64| {
            stage2(d, e, tc, tp, mg, chk)
        })),
        ..Default::default()
    }
}

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion Summon 1 Illusion Fusion Monster from your Extra Deck, using monsters from your Extra Deck and/or field, also you cannot activate monster effects in the hand or GY for the rest of this turn after this card resolves, except Illusion monsters'
    // local e1=Fusion.CreateSummonEff({...})
    let e1 = fusion::create_summon_eff(d, c, fusion_params());
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some((TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E) as u32));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If the activation of this card in its owner's possession was negated by an opponent's card effect and sent to your GY: You can Set this card, and if you do, return all cards your opponent controls to the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SET+CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_SET + CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_CHAIN_NEGATED)
    effect::set_code(d, e2, EVENT_CHAIN_NEGATED);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCondition(s.setcon)
    effect::set_condition(d, e2, Some(condition(setcon)));
    // e2:SetTarget(s.settg)
    effect::set_target(d, e2, Some(cost_target(settg)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `extrafil=function(e,tp,mg) return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToGrave),tp,LOCATION_EXTRA,0,nil) end`
fn extrafil(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    let f = fusion::is_monster_filter(fusion::fus_filter(|d, c, _tp| card::is_able_to_grave(d, c, None)));
    let g = duel::get_matching_group(d, Some(&|d, c| f(d, c, tp)), tp, LOCATION_EXTRA, 0, NO_EX);
    (Some(g), None)
}
/// `extratg=function(e,tp,eg,ep,ev,re,r,rp,chk) if chk==0 then return true end Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,0,tp,LOCATION_EXTRA) end`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,0,tp,LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 0, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `stage2=function(e,tc,tp,mg,chk) if chk==2 then ... end end`
fn stage2(d: &mut Duel, e: EffectId, _tc: Option<CardId>, tp: u8, _mg: Option<&CardSet>, chk: i64) {
    // if chk==2 then
    if chk == 2 {
        // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
        if !effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) {
            return;
        }
        // Ylso you cannot activate monster effects in the hand or GY for the rest of this turn after this card resolves, except Illusion monsters'
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(aux.Stringid(id,2))
        effect::set_description(d, e1, aux::stringid(ID, 2));
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
        effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetValue(function(e,re,tp) ... end)
        effect::set_value_fn(d, e1, lua_fn(|d, a: Args| {
            let re = a.e(1);
            // local rc=re:GetHandler()
            let rc = effect::get_handler(d, re).nil();
            // return re:IsMonsterEffect() and rc:IsLocation(LOCATION_HAND|LOCATION_GRAVE) and rc:IsRaceExcept(RACE_ILLUSION)
            effect::is_monster_effect(d, re) && card::is_location(d, rc, LOCATION_HAND | LOCATION_GRAVE) && card::is_race_except(d, rc, RACE_ILLUSION as u64, None, 0, None)
        }));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
}
/// `s.setcon(e,tp,eg,ep,ev,re,r,rp)`
fn setcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local de,dp=Duel.GetChainInfo(ev,CHAININFO_DISABLE_REASON,CHAININFO_DISABLE_PLAYER)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_DISABLE_REASON, CHAININFO_DISABLE_PLAYER]);
    // return re:GetHandler()==e:GetHandler() and re:IsHasType(EFFECT_TYPE_ACTIVATE) and rp==tp and de and dp==1-tp
    let re = a.re.nil();
    effect::get_handler(d, re) == effect::get_handler(d, a.e)
        && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
        && a.rp == a.tp
        && info[0].truthy()
        && matches!(info[1], Value::Int(p) if p == 1 - a.tp as i64)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsSSetable() and Duel.IsExistingMatchingCard(Card.IsAbleToHand,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return card::is_s_setable(d, c, false, None) && duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_hand(d, x, None)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SET,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SET, Some(c.into()), 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,1-tp,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, 1 - a.tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SSet(tp,c)>0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::s_set(d, a.tp, c, None, true) > 0 {
        // local g=Duel.GetMatchingGroup(Card.IsAbleToHand,tp,0,LOCATION_ONFIELD,nil)
        let g = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_hand(d, x, None)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g, None, REASON_EFFECT, None);
        }
    }
}
