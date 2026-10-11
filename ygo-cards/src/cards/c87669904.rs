//! Dual Avatar Invitation
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 87669904;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[TOKEN_DUAL_AVATAR_SPIRIT],
    listed_series: &[SET_DUAL_AVATAR],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon "Dual Avatar Spirit Tokens" and Fusion Summon up to 2 times
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_HANDES+CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_HANDES + CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp,se)`
fn splimit(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsType(TYPE_FUSION) and c:IsLocation(LOCATION_EXTRA)
    !card::is_type(d, c, TYPE_FUSION) && card::is_location(d, c, LOCATION_EXTRA)
}
/// `s.lizfilter(e,c)`
fn lizfilter(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsOriginalType(TYPE_FUSION)
    !card::is_original_type(d, c, TYPE_FUSION)
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsType,tp,LOCATION_MZONE,0,nil,TYPE_TOKEN)
    // Duel.Destroy(g,REASON_EFFECT)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_type(d, c, TYPE_TOKEN)), a.tp, LOCATION_MZONE, 0, NO_EX);
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,e:GetHandler(),REASON_EFFECT)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_DUAL_AVATAR_SPIRIT,SET_DUAL_AVATAR,TYPES_TOKEN,0,0,2,RACE_WARRIOR,ATTRIBUTE_LIGHT) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, e).nil();
        return duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_discardable(d, c, Some(REASON_EFFECT), None)),
            tp,
            LOCATION_HAND,
            0,
            1,
            h.into(),
        ) && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, tp, TOKEN_DUAL_AVATAR_SPIRIT, token_data(), None, None, 0);
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,ft,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, ft as u8, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,ft,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, ft as u8, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, tp, 1);
    true
}
/// The token data of `TOKEN_DUAL_AVATAR_SPIRIT` as `Duel.IsPlayerCanSpecialSummonMonster` gets it.
fn token_data() -> duel::MonsterData {
    // TYPES_TOKEN,0,0,2,RACE_WARRIOR,ATTRIBUTE_LIGHT with SET_DUAL_AVATAR
    duel::MonsterData {
        setcodes: Some(vec![SET_DUAL_AVATAR as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(2),
        race: Some(RACE_WARRIOR),
        attribute: Some(ATTRIBUTE_LIGHT),
    }
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| splimit(d, c))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // --lizard check
    // aux.addTempLizardCheck(e:GetHandler(),tp,s.lizfilter)
    aux::add_temp_lizard_check(d, c, tp, Some(effect_card_fn(|d, _e, c| lizfilter(d, c))), None, None, None, None);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UNRELEASABLE_SUM)
    effect::set_code(d, e2, EFFECT_UNRELEASABLE_SUM);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsType,TYPE_TOKEN))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_type(d, c, TYPE_TOKEN))));
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UNRELEASABLE_NONSUM)
    effect::set_code(d, e3, EFFECT_UNRELEASABLE_NONSUM);
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, tp);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e4, EVENT_PHASE + PHASE_END);
    // e4:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e4, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e4, RESET_PHASE | PHASE_END, 1);
    // e4:SetOperation(s.desop)
    effect::set_operation(d, e4, Some(operation(desop)));
    // Duel.RegisterEffect(e4,tp)
    duel::register_effect(d, e4, tp);
    // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,1),nil)
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
    // if Duel.DiscardHand(tp,nil,1,1,REASON_EFFECT|REASON_DISCARD)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_DUAL_AVATAR_SPIRIT,SET_DUAL_AVATAR,TYPES_TOKEN,0,0,2,RACE_WARRIOR,ATTRIBUTE_LIGHT) then
    if duel::discard_hand(d, tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX) > 0
        && duel::is_player_can_special_summon_monster(d, tp, TOKEN_DUAL_AVATAR_SPIRIT, token_data(), None, None, 0)
    {
        // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
        let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
        // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=math.min(ft,1) end
        if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
            ft = ft.min(1);
        }
        // if ft<=0 then return end
        if ft <= 0 {
            return;
        }
        // for i=1,ft do
        for _ in 0..ft {
            // local token=Duel.CreateToken(tp,TOKEN_DUAL_AVATAR_SPIRIT)
            let token = duel::create_token(d, tp, TOKEN_DUAL_AVATAR_SPIRIT).nil();
            // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
        // local fc=2
        let mut fc = 2;
        // while fc>0 do
        while fc > 0 {
            // local params={aux.FilterBoolFunction(Card.IsSetCard,SET_DUAL_AVATAR)}
            let p = fusion::SummonEff {
                fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_DUAL_AVATAR as u16]))),
                ..Default::default()
            };
            // if not (Fusion.SummonEffTG(table.unpack(params))(e,tp,eg,ep,ev,re,r,rp,0) and Duel.SelectYesNo(tp,aux.Stringid(id,2))) then return end
            let mut a0 = a.clone();
            a0.chk = Some(0);
            if !(fusion::summon_eff_tg_call(d, &p, &a0) && duel::select_yes_no(d, tp, aux::stringid(ID, 2))) {
                return;
            }
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Fusion.SummonEffOP(table.unpack(params))(e,tp,eg,ep,ev,re,r,rp)
            fusion::summon_eff_op_call(d, &p, a);
            // fc=fc-1
            fc -= 1;
        }
    }
}
