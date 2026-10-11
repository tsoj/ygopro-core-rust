//! Lunalight Serenade Dance
//! 月光小夜曲舞踊
//! scripted by Logical Nonsense
use super::prelude::*;

pub const ID: u32 = 13935001;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[13935002], listed_series: &[SET_LUNALIGHT], ..ScriptInfo::DEFAULT };

/// The token of `Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_LUNALIGHT,TYPES_TOKEN,2000,2000,1,RACE_BEASTWARRIOR,ATTRIBUTE_DARK,POS_FACEUP,1-tp)`
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_LUNALIGHT as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(2000),
        defense: Some(2000),
        level: Some(1),
        race: Some(RACE_BEASTWARRIOR),
        attribute: Some(ATTRIBUTE_DARK),
    }
}

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Lunalight Token" and icrease the ATK of a monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_TRIGGER_O+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_FIELD);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.tkcon)
    effect::set_condition(d, e2, Some(condition(tkcon)));
    // e2:SetTarget(s.tktg)
    effect::set_target(d, e2, Some(cost_target(tktg)));
    // e2:SetOperation(s.tkop)
    effect::set_operation(d, e2, Some(operation(tkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Send 1 card from the hand to the GY and Special Summon 1 "Lunalight" monster from the Deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e3, Some(cost::self_banish()));
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.cfilter(c,e,tp)`: If fusion monster(s) were fusion summoned to your field
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_FUSION) and c:IsControler(tp) and c:IsFusionSummoned()
    //     and c:IsCanBeEffectTarget(e)
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_FUSION)
        && card::is_controler(d, c, tp)
        && card::is_fusion_summoned(d, c)
        && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.tkcon(e,tp,eg,ep,ev,re,r,rp)`: If it ever happened
fn tkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,e,tp)
    let eg = a.eg.clone().nil();
    let (e, tp) = (a.e, a.tp);
    group::is_exists(d, &eg, &|d, x| cfilter(d, x, e, tp), 1, NO_EX)
}
/// `s.tktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`: Activation legality
fn tktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    let eg = a.eg.clone().nil();
    // if chkc then return eg:IsContains(chkc) and s.cfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return eg.iter().any(|x| x == chkc) && cfilter(d, chkc, e, tp);
    }
    // if chk==0 then return eg:IsExists(s.cfilter,1,nil,e,tp)
    //     and Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_LUNALIGHT,TYPES_TOKEN,2000,2000,1,RACE_BEASTWARRIOR,ATTRIBUTE_DARK,POS_FACEUP,1-tp) end
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &|d, x| cfilter(d, x, e, tp), 1, NO_EX)
            && duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), Some(POS_FACEUP as u8), Some(1 - tp), 0);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g=eg:FilterSelect(tp,s.cfilter,1,1,nil,e,tp)
    let g = group::filter_select(d, &eg, tp, &|d, x| cfilter(d, x, e, tp), 1, 1, false, NO_EX).nil();
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 0, 0);
    true
}
/// `s.tkop(e,tp,eg,ep,ev,re,r,rp)`: Performing the effect of ATK gain and token special summoned to opponent's field
fn tkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_LUNALIGHT,TYPES_TOKEN,2000,2000,1,RACE_BEASTWARRIOR,ATTRIBUTE_DARK,POS_FACEUP,1-tp) then
    if duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), None, None).0 > 0
        && duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), Some(POS_FACEUP as u8), Some(1 - tp), 0)
    {
        // local token=Duel.CreateToken(tp,id+1)
        let token = duel::create_token(d, tp, ID + 1).nil();
        // Duel.SpecialSummon(token,0,tp,1-tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(token), 0, tp, 1 - tp, false, false, POS_FACEUP as u8, None);
    }
    // local atk=Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)*500
    let atk = duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) as i64 * 500;
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
    let tc = tc.nil();
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, e) {
        // Increase ATK
        // tc:UpdateAttack(atk,RESET_EVENT|RESETS_STANDARD,e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        card::update_attack(d, tc, atk, Some(RESET_EVENT | RESETS_STANDARD), Some(h), None);
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`: Check if current phase is main phase
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp) and Duel.IsMainPhase()
    duel::is_turn_player(d, a.tp) && duel::is_main_phase(d, None)
}
/// `s.spfilter(c,e,tp)`: Check for "Lunalight" monster
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_LUNALIGHT) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_LUNALIGHT as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`: Activation legality
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToGrave,tp,LOCATION_HAND,0,1,nil)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE,tp)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_grave(d, x, None)), tp, LOCATION_HAND, 0, 1, NO_EX)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, Some(tp), None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_HAND as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`: Performing the effect of sending a card from hand to GY, and then special summmoning 1 "Lunalight" monster from deck
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local tgc=Duel.SelectMatchingCard(tp,Card.IsAbleToGrave,tp,LOCATION_HAND,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, x| card::is_able_to_grave(d, x, None)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let tgc = group::get_first(d, &sel);
    // if tgc and Duel.SendtoGrave(tgc,REASON_EFFECT)>0 and tgc:IsLocation(LOCATION_GRAVE)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE,tp)>0 then
    if let Some(tgc) = tgc {
        if duel::send_to_grave(d, CardSet::from_card(tgc), REASON_EFFECT, None, None) > 0
            && card::is_location(d, tgc, LOCATION_GRAVE)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, Some(tp), None, None).0 > 0
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local sg=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
            let sg = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            // if #sg>0 then
            if !sg.is_empty() {
                // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
}
