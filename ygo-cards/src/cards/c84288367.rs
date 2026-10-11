//! 法の神霊アイワス
use super::prelude::*;

pub const ID: u32 = 84288367;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ALEISTER, SET_INVOKED], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can banish this card from your hand or field; add 1 "Aleister" monster from your Deck to your hand, then if this effect was activated on the field, immediately after this effect resolves, you can Normal Summon 1 Spellcaster monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND|LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_MZONE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e1, Some(cost::self_banish()));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If a face-up Fusion Monster(s) you control is sent to the GY, except by battle, while this card is in your GY: You can Special Summon this card, then you can Fusion Summon 1 "Invoked" monster, using monsters from your hand or field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // c:IsSetCard(SET_ALEISTER) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_ALEISTER as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // local trigger_loc=Duel.GetChainInfo(0,CHAININFO_TRIGGERING_LOCATION)
    let trigger_loc = duel::get_chain_info(d, 0, &[CHAININFO_TRIGGERING_LOCATION])[0].int();
    // if (trigger_loc&LOCATION_MZONE)>0 then
    if (trigger_loc & LOCATION_MZONE as i64) > 0 {
        // Duel.SetPossibleOperationInfo(0,CATEGORY_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_MZONE)
        duel::set_possible_operation_info(d, 0, CATEGORY_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_MZONE) as i32);
    }
    true
}
fn nsfilter(d: &mut Duel, c: CardId) -> bool {
    // c:IsRace(RACE_SPELLCASTER) and c:IsSummonable(true,nil)
    card::is_race(d, c, RACE_SPELLCASTER) && card::is_summonable(d, c, true, None, 0, None)
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SendtoHand(g,nil,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::send_to_hand(d, &g, None, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // local trigger_loc=Duel.GetChainInfo(0,CHAININFO_TRIGGERING_LOCATION)
        let trigger_loc = duel::get_chain_info(d, 0, &[CHAININFO_TRIGGERING_LOCATION])[0].int();
        // if (trigger_loc&LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.nsfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if (trigger_loc & LOCATION_MZONE as i64) > 0
            && duel::is_existing_matching_card(d, Some(&nsfilter), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SUMMON as u64);
            // local sc=Duel.SelectMatchingCard(tp,s.nsfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil):GetFirst()
            let sg = duel::select_matching_card(d, tp, Some(&nsfilter), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
            // if sc then
            if let Some(sc) = group::get_first(d, &sg) {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Summon(tp,sc,true,nil)
                duel::summon(d, tp, sc, true, None, 0, None);
            }
        }
    }
}
fn spconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // c:IsPreviousPosition(POS_FACEUP) and c:IsFusionMonster() and c:IsPreviousLocation(LOCATION_MZONE)
    //     and c:IsPreviousControler(tp) and not c:IsReason(REASON_BATTLE)
    card::is_previous_position(d, c, POS_FACEUP)
        && card::is_fusion_monster(d, c)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_controler(d, c, tp)
        && !card::is_reason(d, c, REASON_BATTLE)
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    // return not eg:IsContains(e:GetHandler()) and eg:IsExists(s.spconfilter,1,nil,tp)
    !group::is_contains(&eg, h) && group::is_exists(d, &eg, &|d, c| spconfilter(d, c, tp), 1, NO_EX)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_FUSION_SUMMON,nil,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_FUSION_SUMMON, None, 1, tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
    if card::is_relate_to_effect(d, c, e) && duel::special_summon(d, CardSet::from_card(c), 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        // local fusion_params={handler=c,fusfilter=aux.FilterBoolFunction(Card.IsSetCard,SET_INVOKED)}
        let params = fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_INVOKED as u16]))),
            ..Default::default()
        };
        let eg_v = match a.eg.clone() {
            Some(g) => Value::Group(g),
            None => Value::Nil,
        };
        let re_v = match a.re {
            Some(re) => Value::Effect(re),
            None => Value::Nil,
        };
        // if Fusion.SummonEffTG(fusion_params)(e,tp,eg,ep,ev,re,r,rp,0) and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
        let tg_args = [
            Value::Effect(e),
            Value::Int(tp as i64),
            eg_v.clone(),
            Value::Int(a.ep as i64),
            Value::Int(a.ev as i64),
            re_v.clone(),
            Value::Int(a.r as i64),
            Value::Int(a.rp as i64),
            Value::Int(0),
        ];
        let tg = fusion::summon_eff_tg(d, params.clone()).call(d, &tg_args).truthy();
        if tg && duel::select_yes_no(d, tp, aux::stringid(ID, 3)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Fusion.SummonEffOP(fusion_params)(e,tp,eg,ep,ev,re,r,rp,1)
            let op_args = [
                Value::Effect(e),
                Value::Int(tp as i64),
                eg_v,
                Value::Int(a.ep as i64),
                Value::Int(a.ev as i64),
                re_v,
                Value::Int(a.r as i64),
                Value::Int(a.rp as i64),
            ];
            fusion::summon_eff_op(d, params).call(d, &op_args);
        }
    }
}
