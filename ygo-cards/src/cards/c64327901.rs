//! Goblin Biker Grand Bash
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 64327901;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GOBLIN], ..ScriptInfo::DEFAULT };

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
    // "Goblin" monsters you control gain 300 ATK for each "Goblin" monster you control
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_GOBLIN))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_GOBLIN as u16]))));
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Additional Normal Summon for a "Goblin" monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_EXTRA_SUMMON_COUNT)
    effect::set_code(d, e3, EFFECT_EXTRA_SUMMON_COUNT);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(LOCATION_HAND|LOCATION_MZONE,0)
    effect::set_target_range(d, e3, LOCATION_HAND | LOCATION_MZONE, 0);
    // e3:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_GOBLIN))
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_GOBLIN as u16]))));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Change the Levels of 2 "Goblin" monsters
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_LVCHANGE)
    effect::set_category(d, e4, CATEGORY_LVCHANGE);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetTarget(s.lvtg)
    effect::set_target(d, e4, Some(cost_target(lvtg)));
    // e4:SetOperation(s.lvop)
    effect::set_operation(d, e4, Some(operation(lvop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    // return 300*Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsSetCard,SET_GOBLIN),e:GetHandlerPlayer(),LOCATION_MZONE,0,nil)
    let p = effect::get_handler_player(d, e);
    300 * duel::get_matching_group_count(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_GOBLIN as u16]))),
        p,
        LOCATION_MZONE,
        0,
        NO_EX,
    ) as i64
}
/// `s.lvfilter(c,e)`
fn lvfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_GOBLIN) and c:HasLevel() and c:IsCanBeEffectTarget(e)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_GOBLIN as u16])
        && card::has_level(d, c)
        && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetLevel)==2
    //     or not sg:IsExists(Card.IsLevel,1,nil,sg:GetSum(Card.GetOriginalLevel))
    let class2 = group::get_class_count(d, sg, &|d, c| card::get_level(d, c) as i64) == 2;
    let sum = group::get_sum(d, sg, &|d, c| card::get_original_level(d, c) as i64) as u32;
    (class2 || !group::is_exists(d, sg, &|d, c| card::is_level(d, c, &[sum]), 1, NO_EX), false)
}
/// `s.lvtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.lvfilter,tp,LOCATION_MZONE,0,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| lvfilter(d, c, a.e)), tp, LOCATION_MZONE, 0, NO_EX);
    // if chk==0 then return #g>=2 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return g.len() >= 2
            && matches!(aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false), Err(true));
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_TARGET)
    let sg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_TARGET as u64), None, None, false)
        .unwrap_or_default();
    // Duel.SetTargetCard(sg)
    duel::set_target_card(d, sg.clone());
    // local b1=sg:GetClassCount(Card.GetLevel)==2
    let b1 = group::get_class_count(d, &sg, &|d, c| card::get_level(d, c) as i64) == 2;
    // local b2=not sg:IsExists(Card.IsLevel,1,nil,sg:GetSum(Card.GetOriginalLevel))
    let sum = group::get_sum(d, &sg, &|d, c| card::get_original_level(d, c) as i64) as u32;
    let b2 = !group::is_exists(d, &sg, &|d, c| card::is_level(d, c, &[sum]), 1, NO_EX);
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,2)},{b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op.map_or(0, |v| v as i64)]);
    // Duel.SetOperationInfo(0,CATEGORY_LVCHANGE,sg,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_LVCHANGE, Some(sg), 2, tp, 0);
    true
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetTargetCards(e):Filter(Card.IsFaceup,nil)
    let tg = duel::get_target_cards(d, a.e).nil();
    let g = group::filter(d, &tg, &|d, c| card::is_faceup(d, c), NO_EX);
    // if #g~=2 then return end
    if g.len() != 2 {
        return;
    }
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if op==1 then
    if op == 1 {
        // if g:GetClassCount(Card.GetLevel)==1 then return end
        if group::get_class_count(d, &g, &|d, c| card::get_level(d, c) as i64) == 1 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,4)) --"Select the monster whose Level will be changed"
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 4));
        // local tc=g:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &sel).nil();
        // Duel.HintSelection(tc,true)
        duel::hint_selection(d, tc, true);
        // local lv=(g-tc):GetFirst():GetLevel()
        let mut rest = g.clone();
        group::remove_card(&mut rest, &CardSet::from_card(tc));
        let other = group::get_first(d, &rest).nil();
        let lv = card::get_level(d, other) as i64;
        // Change its Level
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_CHANGE_LEVEL)
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
        // e1:SetValue(lv)
        effect::set_value(d, e1, lv);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    // elseif op==2 then
    } else if op == 2 {
        // The Levels of both monsters become their combined original Levels
        // local lv=g:GetSum(Card.GetOriginalLevel)
        let lv = group::get_sum(d, &g, &|d, c| card::get_original_level(d, c) as i64);
        // if g:IsExists(Card.IsLevel,1,nil,lv) then return end
        if group::is_exists(d, &g, &|d, c| card::is_level(d, c, &[lv as u32]), 1, NO_EX) {
            return;
        }
        // for tc in g:Iter() do
        for tc in group::iter(d, &g) {
            // Change its Level
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e2:SetCode(EFFECT_CHANGE_LEVEL)
            effect::set_code(d, e2, EFFECT_CHANGE_LEVEL);
            // e2:SetValue(lv)
            effect::set_value(d, e2, lv);
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
}
