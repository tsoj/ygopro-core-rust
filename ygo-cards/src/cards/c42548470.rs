//! Weights & Zenmaisures
use super::prelude::*;

pub const ID: u32 = 42548470;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_WIND_UP], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Change the level of 1 "Wind-up" monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_LVCHANGE+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_LVCHANGE + CATEGORY_DRAW);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,e)`
fn filter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_WIND_UP) and c:HasLevel() and c:IsCanBeEffectTarget(e)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_WIND_UP as u16]) && card::has_level(d, c) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.rescon(sg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetLevel)==#sg
    let res = group::get_class_count(d, sg, &|d, c| card::get_level(d, c) as i64) == sg.len();
    (res, false)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE,0,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| filter(d, c, e)), tp, LOCATION_MZONE, 0, NO_EX);
    let rs: aux::Rescon = &rescon;
    // if chk==0 then return #g>=2 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return g.len() >= 2
            && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(rs), 0, tp, None, None, None, false), Err(true));
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_TARGET)
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(rs), 1, tp, Some(HINTMSG_TARGET as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_LVCHANGE,tg,1,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_LVCHANGE, Some(tg), 1, tp, 1);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DRAW,nil,1,tp,1)
    duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 1, tp, 1);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetTargetCards(e):Match(Card.IsFaceup,nil)
    let mut g = duel::get_target_cards(d, e).nil();
    group::match_(d, &mut g, &|d, c| card::is_faceup(d, c), NO_EX);
    // if #g==2 and g:GetClassCount(Card.GetLevel)==2 then
    if g.len() == 2 && group::get_class_count(d, &g, &|d, c| card::get_level(d, c) as i64) == 2 {
        // Duel.Hint(HINT_SELECTMSG,1-tp,aux.Stringid(id,1))
        duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, aux::stringid(ID, 1));
        // local tc1=g:Select(1-tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, 1 - tp, 1, 1, false, NO_EX).nil();
        let tc1 = group::get_first(d, &sel).nil();
        // local tc2=g:RemoveCard(tc1):GetFirst()
        group::remove_card(&mut g, &CardSet::from_card(tc1));
        let tc2 = group::get_first(d, &g).nil();
        // local lv1=tc1:GetLevel()
        let lv1 = card::get_level(d, tc1);
        // local lv2=tc2:GetLevel()
        let lv2 = card::get_level(d, tc2);
        // Change the other monsters level to match the selected monster's
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_LEVEL)
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
        // e1:SetValue(lv1)
        effect::set_value(d, e1, lv1 as i64);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc2:RegisterEffect(e1)
        card::register_effect(d, tc2, e1, false);
        // if lv1<lv2 and Duel.IsPlayerCanDraw(tp,1) and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if lv1 < lv2 && duel::is_player_can_draw(d, tp, 1) && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Draw(tp,1,REASON_EFFECT)
            duel::draw(d, tp, 1, REASON_EFFECT);
        }
    }
}
