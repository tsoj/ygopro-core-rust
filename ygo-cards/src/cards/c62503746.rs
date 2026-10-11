//! Senko the Skybolt Star
//! 天雷星センコウ
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 62503746;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself from hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);

    // Target 1 monster to decrease ATK and 1 monster to destroy
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DESTROY+CATEGORY_ATKCHANGE)
    effect::set_category(d, e3, CATEGORY_DESTROY + CATEGORY_ATKCHANGE);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e3, Some(cost::self_banish()));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spcfilter(c,tp)`
fn spcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsAttribute(ATTRIBUTE_EARTH) and c:IsFaceup()
    card::is_controler(d, c, tp) && card::is_attribute(d, c, ATTRIBUTE_EARTH) && card::is_faceup(d, c)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.spcfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| spcfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.tgfilter(c,e,tp)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return (c:IsFaceup() and c:IsLevelAbove(5) and c:IsRace(RACE_WARRIOR) and c:IsControler(tp))
    //     or (c:IsControler(1-tp) and c:IsAttackPos()) and c:IsCanBeEffectTarget(e)
    (card::is_faceup(d, c) && card::is_level_above(d, c, 5) && card::is_race(d, c, RACE_WARRIOR) && card::is_controler(d, c, tp))
        || (card::is_controler(d, c, 1 - tp) && card::is_attack_pos(d, c) && card::is_can_be_effect_target(d, c, Some(e)))
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:FilterCount(Card.IsControler,nil,tp)==1
    (group::filter_count(d, sg, &|d, c| card::is_controler(d, c, tp), NO_EX) == 1, false)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local rg=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,e,tp)
    let rg = duel::get_matching_group(d, Some(&|d, c| tgfilter(d, c, a.e, a.tp)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    let rs: aux::Rescon = &rescon;
    // if chk==0 then return aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return matches!(aux::select_unselect_group(d, &rg, Some(a.e), a.tp, Some(2), Some(2), Some(rs), 0, a.tp, None, None, None, false), Err(true));
    }
    // local tg=aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,1,tp,HINTMSG_TARGET)
    let tg = aux::select_unselect_group(d, &rg, Some(a.e), a.tp, Some(2), Some(2), Some(rs), 1, a.tp, Some(HINTMSG_TARGET as u64), None, None, false)
        .unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // local hg=tg:Filter(Card.IsControler,nil,tp)
    let hg = group::filter(d, &tg, &|d, c| card::is_controler(d, c, a.tp), NO_EX);
    // e:SetLabelObject(hg:GetFirst())
    let hg_first = group::get_first(d, &hg);
    effect::set_label_object(d, a.e, hg_first.map_or(Value::Nil, Value::Card));
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,hg,1,tp,-1500)
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(hg.clone()), 1, a.tp, -1500);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tg-hg,1,tp,0)
    let mut rest = tg.clone();
    group::remove_card(&mut rest, &hg);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(rest), 1, a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    // if #g==0 then return end
    let g = duel::get_target_cards(d, a.e).nil();
    if g.is_empty() {
        return;
    }
    // local tc1=g:GetFirst()
    // local tc2=g:GetNext()
    let order = group::iter(d, &g);
    let first = order[0];
    let second = order.get(1).copied();
    // if tc2==e:GetLabelObject() then tc1,tc2=tc2,tc1 end
    // (the target group always has two cards, so tc2 is set here)
    let label = effect::get_label_object(d, a.e).as_card();
    let (tc1, tc2) = if second.is_some() && second == label { (second.unwrap_or(first), Some(first)) } else { (first, second) };
    // if tc1:IsControler(tp) and tc1:UpdateAttack(-1500,RESET_EVENT|RESETS_STANDARD,e:GetHandler())==-1500
    let h = effect::get_handler(d, a.e).nil();
    if card::is_controler(d, tc1, a.tp) && card::update_attack(d, tc1, -1500, Some(RESET_EVENT | RESETS_STANDARD), Some(h), None) == -1500 {
        // and tc2 and tc2:IsControler(1-tp) then
        if let Some(tc2) = tc2 {
            if card::is_controler(d, tc2, 1 - a.tp) {
                // Duel.Destroy(tc2,REASON_EFFECT)
                duel::destroy(d, tc2, REASON_EFFECT, None, None);
            }
        }
    }
}
