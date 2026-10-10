//! Fossil Fusion
use super::prelude::*;

pub const ID: u32 = 59419719;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FOSSIL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion summon 1 "Fossil" fusion monster
    // By banishing appropriate monsters from either GY as material
    // local e1=Fusion.CreateSummonEff({handler=c,fusfilter=aux.FilterBoolFunction(Card.IsSetCard,SET_FOSSIL),matfilter=aux.FALSE,extrafil=s.fextra,
    //     stage2=s.stage2,extraop=Fusion.BanishMaterial,extratg=s.extratarget})
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, x, _tp| card::is_set_card(d, x, &[SET_FOSSIL as u16]))),
            matfilter: Some(fusion::mat_filter_fn(|_d, _c, _e, _tp, _chk| false)),
            extrafil: Some(std::sync::Arc::new(fextra)),
            stage2: Some(std::sync::Arc::new(stage2)),
            extraop: Some(fusion::banish_material()),
            extratg: Some(cost_target(extratarget)),
            ..Default::default()
        },
    );
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add this card from GY to hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.fextra(e,tp,mg)
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // if not Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) then
    if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_none() {
        // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToRemove),tp,LOCATION_GRAVE,LOCATION_GRAVE,nil)
        let g = duel::get_matching_group(
            d,
            Some(&|d, c| card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)),
            tp,
            LOCATION_GRAVE,
            LOCATION_GRAVE,
            NO_EX,
        );
        (Some(g), None)
    } else {
        // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToRemove),tp,LOCATION_ONFIELD,0,nil)
        let g = duel::get_matching_group(
            d,
            Some(&|d, c| card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)),
            tp,
            LOCATION_ONFIELD,
            0,
            NO_EX,
        );
        (Some(g), None)
    }
}
// function s.stage2(e,tc,tp,sg,chk)
fn stage2(d: &mut Duel, e: EffectId, tc: Option<CardId>, tp: u8, sg: Option<&CardSet>, chk: i64) {
    // if chk==3 then
    if chk == 3 {
        let sg = sg.nil();
        // local mats=sg:Filter(Card.IsPreviousLocation,nil,LOCATION_GRAVE)
        let mats = group::filter(d, sg, &|d, c| card::is_previous_location(d, c, LOCATION_GRAVE), NO_EX);
        // if mats:IsExists(Card.IsPreviousControler,1,nil,tp) and mats:IsExists(Card.IsPreviousControler,1,nil,1-tp) then
        if group::is_exists(d, &mats, &|d, c| card::is_previous_controler(d, c, tp), 1, NO_EX)
            && group::is_exists(d, &mats, &|d, c| card::is_previous_controler(d, c, 1 - tp), 1, NO_EX)
        {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetDescription(3003)
            effect::set_description(d, e1, 3003);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
            effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
            // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetRange(LOCATION_MZONE)
            effect::set_range(d, e1, LOCATION_MZONE);
            // e1:SetValue(s.tgval)
            effect::set_value_fn(d, e1, lua_fn(tgval));
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1,true)
            card::register_effect(d, tc.nil(), e1, true);
        }
    }
}
// function s.extratarget(e,tp,eg,ep,ev,re,r,rp,chk)
fn extratarget(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,PLAYER_EITHER,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, PLAYER_EITHER as u8, LOCATION_GRAVE as i32);
    true
}
// function s.tgval(e,re,rp)
fn tgval(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    // local rc=re:GetHandler()
    let _rc = effect::get_handler(d, re).nil();
    // return re:IsMonsterEffect()
    effect::is_monster_effect(d, re)
}
// function s.thcfilter(c,tp)
fn thcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsReason(REASON_BATTLE|REASON_EFFECT) and c:IsSetCard(SET_FOSSIL) and c:IsType(TYPE_FUSION)
    //     and c:GetPreviousControler()==tp and c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousPosition(POS_FACEUP)
    card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && card::is_set_card(d, c, &[SET_FOSSIL as u16])
        && card::is_type(d, c, TYPE_FUSION)
        && card::get_previous_controler(d, c) == tp
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_position(d, c, POS_FACEUP)
}
// function s.thcon(e,tp,eg,ep,ev,re,r,rp)
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return eg:IsExists(s.thcfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| thcfilter(d, c, tp), 1, NO_EX)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsAbleToHand() end
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, e) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,c)
        duel::confirm_cards(d, 1 - tp, c);
    }
}
