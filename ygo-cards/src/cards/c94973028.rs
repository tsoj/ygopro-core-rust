//! Mecha Phantom Beast Coltwing
use super::prelude::*;

pub const ID: u32 = 94973028;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_MECHA_PHANTOM_BEAST],
    listed_names: &[TOKEN_MECHA_PHANTOM_BEAST],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Gains the levels of all "Mecha Phantom Beast Token"
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_UPDATE_LEVEL)
    effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
    // e1:SetValue(s.lvval)
    effect::set_value_fn(d, e1, effect_card_value(lvval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot be destroyed by battle or effects while you control a Token
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetCondition(s.indcon)
    effect::set_condition(d, e2, Some(condition(indcon)));
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
    // Special Summon 2 "Mecha Phantom Beast Tokens"
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // e4:SetCondition(s.spcon)
    effect::set_condition(d, e4, Some(condition(spcon)));
    // e4:SetTarget(s.sptg)
    effect::set_target(d, e4, Some(cost_target(sptg)));
    // e4:SetOperation(s.spop)
    effect::set_operation(d, e4, Some(operation(spop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Destroy and banish 1 card your opponent controls
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetCategory(CATEGORY_DESTROY+CATEGORY_REMOVE)
    effect::set_category(d, e5, CATEGORY_DESTROY + CATEGORY_REMOVE);
    // e5:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e5, EFFECT_FLAG_CARD_TARGET, 0);
    // e5:SetCountLimit(1)
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    // e5:SetCost(s.descost)
    effect::set_cost(d, e5, Some(cost_target(descost)));
    // e5:SetTarget(s.destg)
    effect::set_target(d, e5, Some(cost_target(destg)));
    // e5:SetOperation(s.desop)
    effect::set_operation(d, e5, Some(operation(desop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.lvval(e,c)`
fn lvval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsCode,TOKEN_MECHA_PHANTOM_BEAST),c:GetControler(),LOCATION_MZONE,0,nil):GetSum(Card.GetLevel)
    let f = aux::faceup_filter(|d, x| card::is_code(d, x, &[TOKEN_MECHA_PHANTOM_BEAST]));
    let p = card::get_controler(d, c);
    let g = duel::get_matching_group(d, Some(&f), p, LOCATION_MZONE, 0, NO_EX);
    group::get_sum(d, &g, &|d, x| card::get_level(d, x) as i64)
}
/// `s.tknfilter(c)`
fn tknfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_TOKEN) or c:IsOriginalType(TYPE_TOKEN)
    card::is_type(d, c, TYPE_TOKEN) || card::is_original_type(d, c, TYPE_TOKEN)
}
/// `s.indcon(e)`
fn indcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.tknfilter,e:GetHandlerPlayer(),LOCATION_ONFIELD,0,1,nil)
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&tknfilter), p, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `aux.FaceupFilter(Card.IsSetCard,SET_MECHA_PHANTOM_BEAST)`
fn mecha_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MECHA_PHANTOM_BEAST as u16])
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_MECHA_PHANTOM_BEAST),tp,LOCATION_MZONE,0,1,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    duel::is_existing_matching_card(d, Some(&mecha_faceup), a.tp, LOCATION_MZONE, 0, 1, h.into())
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.FaceupFilter(...),tp,LOCATION_MZONE,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&mecha_faceup), tp, LOCATION_MZONE, 0, 1, h.into());
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 2, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) or Duel.GetLocationCount(tp,LOCATION_MZONE)<2 then return end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()
        || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 2
    {
        return;
    }
    // if Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_MECHA_PHANTOM_BEAST,SET_MECHA_PHANTOM_BEAST,TYPES_TOKEN,0,0,3,RACE_MACHINE,ATTRIBUTE_WIND) then
    if duel::is_player_can_special_summon_monster(
        d,
        tp,
        TOKEN_MECHA_PHANTOM_BEAST,
        duel::MonsterData {
            setcodes: Some(vec![SET_MECHA_PHANTOM_BEAST as u16]),
            type_: Some(TYPES_TOKEN),
            attack: Some(0),
            defense: Some(0),
            level: Some(3),
            race: Some(RACE_MACHINE),
            attribute: Some(ATTRIBUTE_WIND),
        },
        None,
        None,
        0,
    ) {
        // local token=Duel.CreateToken(tp,TOKEN_MECHA_PHANTOM_BEAST)
        let token = duel::create_token(d, tp, TOKEN_MECHA_PHANTOM_BEAST).nil();
        // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        // token=Duel.CreateToken(tp,TOKEN_MECHA_PHANTOM_BEAST)
        let token = duel::create_token(d, tp, TOKEN_MECHA_PHANTOM_BEAST).nil();
        // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
/// `s.descfilter(c)`
fn descfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_TOKEN)
    card::is_type(d, c, TYPE_TOKEN)
}
/// `s.desfilter(c,e)`: `e` is nil when the filter is called without it
fn desfilter(d: &mut Duel, c: CardId, e: Option<EffectId>) -> bool {
    // return c:IsAbleToRemove() and (not e or c:IsCanBeEffectTarget(e))
    card::is_able_to_remove(d, c, None, None, None) && e.map_or(true, |e| card::is_can_be_effect_target(d, c, Some(e)))
}
/// `s.descost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local dg=Duel.GetMatchingGroup(s.desfilter,tp,0,LOCATION_ONFIELD,nil,e)
    let e = a.e;
    let dg = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c, Some(e))), tp, 0, LOCATION_ONFIELD, NO_EX);
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.descfilter,2,false,aux.ReleaseCheckTarget,nil,dg) end
    let check = |d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| aux::release_check_target(d, sg, tp, exg, &dg);
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&descfilter), 2, 2, false, Some(&check), NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.descfilter,2,2,false,aux.ReleaseCheckTarget,nil,dg)
    let g = duel::select_release_group_cost(d, tp, Some(&descfilter), 2, 2, false, Some(&check), NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, &g, REASON_COST, None);
    true
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsOnField() and s.desfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_on_field(d, chkc) && desfilter(d, chkc, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.desfilter,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| desfilter(d, c, None)), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.desfilter,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| desfilter(d, c, None)), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 1, tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then Duel.Destroy(tc,REASON_EFFECT,LOCATION_REMOVED) end
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, Some(LOCATION_REMOVED), None);
    }
}
