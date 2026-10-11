//! Arcana Force EX - The Chaos Ruler
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 12686296;

pub const INFO: ScriptInfo = ScriptInfo {
    toss_coin: true,
    listed_series: &[SET_ARCANA_FORCE as u32],
    material_setcode: &[SET_ARCANA_FORCE as u32],
    listed_names: &[CARD_LIGHT_BARRIER],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 3 "Arcana Force" monsters with different names
    // Fusion.AddProcMixN(c,true,true,s.ffilter,3)
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat_full(ffilter)), 3)]);
    // Fusion.AddContactProc(c,s.contactfil,s.contactop,true)
    fusion::add_contact_proc(
        d,
        c,
        fusion::Contact {
            material_filter: Some(Arc::new(contactfil)),
            material_operation: Some(Arc::new(contactop)),
            sumcon: Some(None),
            ..Default::default()
        },
    );
    // Toss a coin and apply the appropriate effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COIN+CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_COIN + CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetTarget(s.cointg)
    effect::set_target(d, e1, Some(cost_target(cointg)));
    // e1:SetOperation(s.coinop)
    effect::set_operation(d, e1, Some(operation(coinop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Your opponent cannot activate monster effects on the field while "Light Barrier" is in a Field Zone
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(0,1)
    effect::set_target_range(d, e2, 0, 1);
    // e2:SetCondition(function() return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_LIGHT_BARRIER),0,LOCATION_FZONE,LOCATION_FZONE,1,nil) end)
    effect::set_condition(
        d,
        e2,
        Some(condition(|d, _a| {
            duel::is_existing_matching_card(
                d,
                Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_LIGHT_BARRIER]))),
                0,
                LOCATION_FZONE,
                LOCATION_FZONE,
                1,
                NO_EX,
            )
        })),
    );
    // e2:SetValue(function(e,re,tp) return re:IsMonsterEffect() and re:GetHandler():IsOnField() end)
    effect::set_value_fn(
        d,
        e2,
        lua_fn(|d, a: Args| {
            let re = a.e(1);
            let h = effect::get_handler(d, re).nil();
            effect::is_monster_effect(d, re) && card::is_on_field(d, h)
        }),
    );
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.ffilter(c,fc,sumtype,tp,sub,mg,sg)`
fn ffilter(d: &mut Duel, c: CardId, a: &fusion::MatArgs) -> bool {
    // return c:IsSetCard(SET_ARCANA_FORCE,fc,0,tp) and (not sg or not sg:IsExists(s.fusfilter,1,c,c:GetCode(fc,0,tp),fc,0,tp))
    if !card::is_set_card_ctx(d, c, &[SET_ARCANA_FORCE as u16], Some(a.fc), 0, Some(a.tp)) {
        return false;
    }
    match a.sg {
        None => true,
        Some(sg) => {
            let code = card::get_code_ctx(d, c, Some(a.fc), 0, Some(a.tp))[0];
            let fc = a.fc;
            let tp = a.tp;
            !group::is_exists(d, sg, &|d, x| fusfilter(d, x, code, fc, tp), 1, c.into())
        }
    }
}
/// `s.fusfilter(c,code,fc,sumtype,tp)`
fn fusfilter(d: &mut Duel, c: CardId, code: u32, fc: CardId, tp: u8) -> bool {
    // return c:IsSummonCode(fc,0,tp,code) and not c:IsHasEffect(511002961)
    card::is_summon_code(d, c, Some(fc), 0, tp, &[code]) && card::is_has_effect(d, c, 511002961, None).is_empty()
}
/// `s.contactfilter(c,tp)`
fn contactfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ARCANA_FORCE) and c:IsAbleToGraveAsCost() and (c:IsControler(tp) or c:IsFaceup())
    card::is_set_card(d, c, &[SET_ARCANA_FORCE as u16])
        && card::is_able_to_grave_as_cost(d, c)
        && (card::is_controler(d, c, tp) || card::is_faceup(d, c))
}
/// `s.contactfil(tp)`
fn contactfil(d: &mut Duel, tp: u8) -> CardSet {
    // return Duel.GetMatchingGroup(s.contactfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,tp)
    duel::get_matching_group(d, Some(&|d, c| contactfilter(d, c, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX)
}
/// `s.contactop(g,tp,c)`
fn contactop(d: &mut Duel, g: &CardSet, _tp: u8, _c: CardId) {
    // Duel.SendtoGrave(g,REASON_COST|REASON_MATERIAL)
    duel::send_to_grave(d, g, REASON_COST | REASON_MATERIAL, None, None);
}
/// `s.cointg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cointg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_COIN,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, a.tp, 1);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevel(10) and c:IsSetCard(SET_ARCANA_FORCE) and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_level(d, c, &[10]) && card::is_set_card(d, c, &[SET_ARCANA_FORCE as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c.toss_coin and c:IsAbleToHand()
    crate::card_info(d, c).toss_coin && card::is_able_to_hand(d, c, None)
}
/// `s.coinop(e,tp,eg,ep,ev,re,r,rp)`
fn coinop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local coin=nil
    let coin: i32;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_LIGHT_BARRIER) then
    if duel::is_player_affected_by_effect(d, tp, CARD_LIGHT_BARRIER).is_some() {
        // local b1=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp)
        let b1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
        // local b2=Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
        let b2 = duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
        // local op=Duel.SelectEffect(tp,
        //     {b1,aux.GetCoinEffectHintString(COIN_HEADS)},
        //     {b2,aux.GetCoinEffectHintString(COIN_TAILS)})
        let op = duel::select_effect(
            d,
            tp,
            &[
                (b1, aux::get_coin_effect_hint_string(COIN_HEADS as i32).nil()),
                (b2, aux::get_coin_effect_hint_string(COIN_TAILS as i32).nil()),
            ],
        );
        // if not op then return end
        let Some(op) = op else {
            return;
        };
        // coin=op==1 and COIN_HEADS or COIN_TAILS
        coin = if op == 1 { COIN_HEADS as i32 } else { COIN_TAILS as i32 };
    // else
    } else {
        // coin=Duel.TossCoin(tp,1)
        coin = duel::toss_coin(d, tp, 1)[0];
    }
    // if coin==COIN_HEADS then
    if coin == COIN_HEADS as i32 {
        // Heads: Special Summon 1 Level 10 "Arcana Force" monster from your hand or Deck, ignoring its Summoning conditions
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,true,false,POS_FACEUP)
            duel::special_summon(d, &g, 0, tp, tp, true, false, POS_FACEUP as u8, None);
        }
    // elseif coin==COIN_TAILS then
    } else if coin == COIN_TAILS as i32 {
        // Tails: Add 1 card from your Deck to your hand that has a coin tossing effect
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, &g);
        }
    }
}
